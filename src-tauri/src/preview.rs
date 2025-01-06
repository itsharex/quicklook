use std::sync::mpsc;
use std::thread;
use tauri::{
    webview::PageLoadEvent, AppHandle, Error as TauriError,  Manager, WebviewUrl, WebviewWindowBuilder
};
use windows::{
    core::{w, Error as WError, Interface, VARIANT},
    Win32::{
        Foundation::{BOOL, HWND, LPARAM, LRESULT, WPARAM},
        System::{
            Com::{
                CoCreateInstance, CoInitializeEx, CoUninitialize, IDispatch, IServiceProvider, CLSCTX_INPROC_SERVER, CLSCTX_LOCAL_SERVER, COINIT_APARTMENTTHREADED
            }, 
            SystemServices::SFGAO_FILESYSTEM, 
            Variant::{self}
        },
        UI::{
            Accessibility::{CUIAutomation, IUIAutomation, IUIAutomationSelectionPattern, UIA_NamePropertyId,  UIA_SelectionPatternId}, 
            Input::KeyboardAndMouse, 
            Shell::{
                IShellBrowser, IShellItemArray, IShellView, IShellWindows, ShellWindows, SIGDN_DESKTOPABSOLUTEPARSING, SIGDN_FILESYSPATH, SVGIO_SELECTION, SWFO_NEEDDISPATCH
            }, 
            WindowsAndMessaging
        },
    },
};

#[path = "./helper/mod.rs"]
mod helper;
use helper::{win, monitor};

#[path = "./utils/mod.rs"]
mod utils;
use utils::{get_file_info, File as UFile};

#[derive(Debug)]
pub struct PreviewFile {
    hook_handle: Option<WindowsAndMessaging::HHOOK>, // 钩子的句柄
    app_handle: Option<AppHandle>,
}

struct Selected;

impl Selected {
    pub fn new() -> Option<String> {
        let path = Self::get_selected_file();
        println!("path: {:?}", path);
        return path;
    }

    fn get_selected_file() -> Option<String> {
        if let Some(focused_type) = Self::get_focused_type() {
            return match focused_type.as_str() {
                "explorer" => unsafe { Self::get_select_file_from_explorer().ok() },
                "desktop" => unsafe { Self::get_select_file_from_desktop().ok() },
                "dialog" => {
                    match Self::get_select_file_from_dialog() {
                        Ok(result) => Some(result),
                        Err(err) => None
                    }
                },
                _ => None,
            };
        }
        None
    }
    fn get_focused_type() -> Option<String> {
        let mut type_str: Option<String> = None;
        let hwnd_gfw = unsafe { WindowsAndMessaging::GetForegroundWindow() };
        let class_name = win::get_window_class_name(hwnd_gfw);
        log::info!("class_name: {}", class_name);

        if class_name.contains("CabinetWClass") {
            type_str = Some("explorer".to_string());
        } else if class_name.contains("Progman") || class_name.contains("WorkerW") {
            let defview = unsafe { WindowsAndMessaging::FindWindowExW(hwnd_gfw, None, w!("SHELLDLL_DefView"), None) };
            if defview.is_ok() {
                type_str = Some("desktop".to_string());
            }
        }else if class_name.contains("#32770") {
            type_str = Some("dialog".to_string());
        }
        log::info!("type_str: {:?}", type_str);
        type_str
    }

    unsafe fn get_select_file_from_explorer() -> Result<String, WError> {
        let (tx, rx) = mpsc::channel();

        // 在新的线程中执行 COM 操作
        thread::spawn(move || {
            // 在子线程中初始化 COM 库为单线程单元
            let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

            let hwnd_gfw = WindowsAndMessaging::GetForegroundWindow();
            let shell_windows: IShellWindows =
                CoCreateInstance(&ShellWindows, None, CLSCTX_LOCAL_SERVER).unwrap();
            let result_hwnd =
                WindowsAndMessaging::FindWindowExW(hwnd_gfw, None, w!("ShellTabWindowClass"), None)
                    .unwrap();

            let mut target_path = String::new();
            let count = shell_windows.Count().unwrap_or_default();

            for i in 0..count {
                let variant = VARIANT::from(i);
                let dispatch: IDispatch = shell_windows.Item(&variant).unwrap();

                let shell_browser = Self::dispath2browser(dispatch);

                if shell_browser.is_none() {
                    continue;
                }
                let shell_browser = shell_browser.unwrap();
                // 调用 GetWindow 可能会阻塞 GUI 消息
                let phwnd = shell_browser.GetWindow().unwrap();
                if hwnd_gfw.0 != phwnd.0 && result_hwnd.0 != phwnd.0 {
                    continue;
                }

                let shell_view = shell_browser.QueryActiveShellView().unwrap();
                target_path = Self::get_selected_file_path_from_shellview(shell_view);
            }

            CoUninitialize();
            tx.send(target_path).unwrap();
        });

        let target_path = rx.recv().unwrap();

        Ok(target_path)
    }

    unsafe fn get_select_file_from_desktop() -> Result<String, WError> {
        let (tx, rx) = mpsc::channel();

        // 在新的线程中执行 COM 操作
        thread::spawn(move || {
            // 初始化 COM 库
            let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
            let mut target_path = String::new();
            let hwnd_gfw = WindowsAndMessaging::GetForegroundWindow(); // 获取当前活动窗口句柄
            log::info!("hwnd_gfw: {:?}", hwnd_gfw);
            let shell_windows: Result<IShellWindows, WError>  =
                CoCreateInstance(&ShellWindows, None, CLSCTX_LOCAL_SERVER);
            if shell_windows.is_err() {
                log::info!("shell_windows 不存在");
                tx.send(target_path.clone()).unwrap();
            }
            let shell_windows = shell_windows.unwrap();

            let pvar_loc: VARIANT = Variant::VariantInit();

            // 获取活动窗口
            let mut phwnd: i32 = 0;

            let dispatch = shell_windows
                .FindWindowSW(
                    &pvar_loc,
                    &pvar_loc,
                    windows::Win32::UI::Shell::SWC_DESKTOP,
                    &mut phwnd,
                    SWFO_NEEDDISPATCH,
                );
            if dispatch.is_err() {
                log::info!("dispatch 不存在");
                tx.send(target_path.clone()).unwrap();
            }

            let shell_browser = Self::dispath2browser(dispatch.unwrap());
            if shell_browser.is_none() {
                log::info!("shell_browser 不存在");
                tx.send(target_path.clone()).unwrap();
            }
            let shell_browser = shell_browser.unwrap();

            let phwnd = shell_browser.GetWindow().unwrap();
            let top = WindowsAndMessaging::GetAncestor(phwnd, WindowsAndMessaging::GA_ROOT);

            if !hwnd_gfw.eq(&top) {
               log::info!("top hwnd 不相等");
               tx.send(target_path.clone()).unwrap();
            }

            let shell_view = shell_browser.QueryActiveShellView();
            if shell_view.is_err() {
                log::info!("shell_view 不存在");
                tx.send(target_path.clone()).unwrap();
            }

            target_path = Self::get_selected_file_path_from_shellview(shell_view.unwrap());
            CoUninitialize();
            tx.send(target_path).unwrap();
        });
        let target_path = rx.recv().unwrap();
        Ok(target_path)
    }
     
    fn get_select_file_from_dialog() -> Result<String, WError> {
        let mut target_path = String::new();
        let fw_hwnd = unsafe {
            WindowsAndMessaging::GetForegroundWindow()
        };
        println!("fw_hwnd: {:?}", fw_hwnd);

        let defview = unsafe {
            let mut tmp: Option<HWND> = None;
            let _ = WindowsAndMessaging::EnumChildWindows(fw_hwnd, Some(Self::dialog_defview_proc), LPARAM(&mut tmp as *mut _ as isize));
            tmp
        };

        if defview.is_none() {
            log::info!("defview 不存在");
            return Ok(target_path);
        }
        let defview = defview.unwrap();

        let listview = unsafe {
            WindowsAndMessaging::FindWindowExW(defview, None, w!("DirectUIHWND"), None)
        };
        if listview.is_err() {
            log::info!("listview(DirectUIHWND) 不存在");
            return  Ok(target_path);
        }
        let listview = listview.unwrap();
        let seleced_file_title = unsafe {
            let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
            // 通过 ui automation 获取选中文件
            let automation: IUIAutomation = CoCreateInstance(
                &CUIAutomation,
                None,
                CLSCTX_INPROC_SERVER
            )?;
            // 获取列表元素
            let list_element = automation.ElementFromHandle(listview)?;
            
            // 获取选中项
            let selection_pattern = list_element.GetCurrentPattern(UIA_SelectionPatternId)?;
            let selection = selection_pattern.cast::<IUIAutomationSelectionPattern>()?;
            
            // 获取选中的元素
            let selected = selection.GetCurrentSelection()?;
            let count = selected.Length()?;
            let mut file_name = String::new();
            if count > 0 {
                // 获取第一个选中项
                let item = selected.GetElement(0)?;
                // 获取文件名
                let name = item.GetCurrentPropertyValue(UIA_NamePropertyId)?;
                println!("name: {:?}", name);
                file_name = name.to_string();
            }
            file_name
        };
        println!("seleced_file_title: {:?}", seleced_file_title);

        // 获取搜索框的 Text
        let mut breadcrumb_parent_hwnd: Option<HWND> = None;
        let _ = unsafe {
            WindowsAndMessaging::EnumChildWindows(fw_hwnd, Some(Self::breadcrumb_proc), LPARAM(&mut breadcrumb_parent_hwnd as *const _ as isize))
        };
        if breadcrumb_parent_hwnd.is_none() {
            return Ok(target_path);
        }
        let breadcrumb_parent_hwnd = breadcrumb_parent_hwnd.unwrap();
        let breadcrumb_hwnd = unsafe {
            WindowsAndMessaging::FindWindowExW(breadcrumb_parent_hwnd, None, w!("ToolbarWindow32"), None)
        };
        if breadcrumb_hwnd.is_err() {
            return Ok(target_path);
        }
        let breadcrumb_hwnd = breadcrumb_hwnd.unwrap();
        let mut breadcrumb_title = win::get_window_text(breadcrumb_hwnd);
        println!("breadcrumb_title: {:?}", breadcrumb_title);
        let re = regex::Regex::new(r"[A-Z]:\\.*").unwrap();
        if let Some(cap) = re.find(&breadcrumb_title) {
            breadcrumb_title = cap.as_str().to_string();
        }

        target_path = format!("{}\\{}", breadcrumb_title, seleced_file_title);
        println!("target_path: {:?}", target_path);
            
        Ok(target_path)
    }

    unsafe extern "system" fn dialog_defview_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let list_view = lparam.0 as *mut Option<HWND>;
        let class_name = win::get_window_class_name(hwnd);
        if class_name.contains("SHELLDLL_DefView") {
            *list_view = Some(hwnd);
            return BOOL(0);
        }
        BOOL(1)
    }
    unsafe extern "system" fn breadcrumb_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let list_view = lparam.0 as *mut Option<HWND>;
        let class_name = win::get_window_class_name(hwnd);
        if class_name.contains("Breadcrumb Parent") {
            *list_view = Some(hwnd);
            return BOOL(0);
        }
        BOOL(1)
    }

    unsafe fn dispath2browser(dispatch: IDispatch) -> Option<IShellBrowser> {
        
        let mut service_provider: Option<IServiceProvider> = None;
        dispatch
            .query(
                &IServiceProvider::IID,
                &mut service_provider as *mut _ as *mut _,
            )
            .ok()
            .unwrap();
        if service_provider.is_none() {
            return None;
        }
        let shell_browser = service_provider
            .unwrap()
            .QueryService::<IShellBrowser>(&IShellBrowser::IID)
            .ok();
        shell_browser
    }

    unsafe fn get_selected_file_path_from_shellview(shell_view: IShellView) -> String {
        let mut target_path = String::new();
        let shell_items = shell_view.GetItemObject::<IShellItemArray>(SVGIO_SELECTION);

        if shell_items.is_err() {
            return target_path;
        }
        println!("shell_items: {:?}", shell_items);
        let shell_items = shell_items.unwrap();
        let count = shell_items.GetCount().unwrap_or_default();
        for i in 0..count {
            let shell_item = shell_items.GetItemAt(i).unwrap();

            // 如果不是文件对象则继续循环
            if let Ok(attrs) = shell_item.GetAttributes(SFGAO_FILESYSTEM) {
                log::info!("attrs: {:?}", attrs);
                if attrs.0 == 0 {
                    continue;
                }
            }

            if let Ok(display_name) = shell_item.GetDisplayName(SIGDN_FILESYSPATH) {
                target_path = display_name.to_string().unwrap();
                break;
            }
            if let Ok(display_name) = shell_item.GetDisplayName(SIGDN_DESKTOPABSOLUTEPARSING)
            {
                target_path = display_name.to_string().unwrap();
                break;
            }
        }
        target_path
    }
}

impl Drop for Selected {
    fn drop(&mut self) {
        unsafe { CoUninitialize() }
    }
}

pub struct WebRoute {
    path: String,
    query: UFile,
}
impl WebRoute {
    pub fn to_url(&self) -> String {
        let mut url = self.path.clone();
        url.push_str("?");
        url.push_str(
            format!(
                "file_type={}&path={}&extension={}&size={}&last_modified={}&name={}",
                self.query.get_file_type(),
                urlencoding::encode(&self.query.get_path()),
                self.query.get_extension(),
                self.query.get_size(),
                self.query.get_last_modified(),
                self.query.get_name()
            )
            .as_str(),
        );
        url
    }
    pub fn new(path: String, query: UFile) -> Self {
        Self { path, query }
    }
    pub fn get_route(type_str: &str, file_info: &UFile) -> WebRoute {
        match type_str {
            "Markdown" => WebRoute::new("/preview/md".to_string(), file_info.clone()),
            "Text" => WebRoute::new("/preview/text".to_string(), file_info.clone()),
            "Image" => WebRoute::new("/preview/image".to_string(), file_info.clone()),
            "Video" => WebRoute::new("/preview/video".to_string(), file_info.clone()),
            "Font" => WebRoute::new("/preview/font".to_string(), file_info.clone()),
            "Code" => WebRoute::new("/preview/code".to_string(), file_info.clone()),
            "Book" => WebRoute::new("/preview/book".to_string(), file_info.clone()),
            "Archive" => WebRoute::new("/preview/archive".to_string(), file_info.clone()),
            "Doc" => WebRoute::new("/preview/document".to_string(), file_info.clone()),
            _ => WebRoute::new("/preview/not-support".to_string(), file_info.clone()),
        }
    }
}

#[allow(dead_code)]
impl PreviewFile {
    // 注册键盘钩子
    pub fn set_keyboard_hook(&mut self) {
        let hook_ex = unsafe {
            WindowsAndMessaging::SetWindowsHookExW(
                WindowsAndMessaging::WH_KEYBOARD_LL,
                Some(Self::keyboard_proc), // 使用结构体的键盘回调
                None,                      // 当前进程实例句柄
                0,
            )
        };
        match hook_ex {
            Ok(result) => {
                self.hook_handle = Some(result);
            }
            Err(_) => {
                self.hook_handle = None;
            }
        }
    }

    // 取消键盘钩子
    pub fn remove_keyboard_hook(&mut self) {
        if let Some(hook) = self.hook_handle {
            unsafe {
                let _ = WindowsAndMessaging::UnhookWindowsHookEx(hook);
            }
            self.hook_handle = None;
        }
    }

    // 按键处理逻辑
    pub fn handle_key_down(&self, vk_code: u32) {
        if vk_code == KeyboardAndMouse::VK_SPACE.0 as u32 {
            let _ = Self::preview_file(self.app_handle.clone().unwrap());
        }
    }

    // 全局键盘钩子的回调函数
    extern "system" fn keyboard_proc(ncode: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        // 确保消息被传递给其他应用程序
        let next_hook_result = unsafe { WindowsAndMessaging::CallNextHookEx(None, ncode, wparam, lparam) };
        #[cfg(debug_assertions)]
        log::info!("Hook called - next_hook_result: {:?}", next_hook_result);
        
        if ncode >= 0 && (wparam.0 == WindowsAndMessaging::WM_KEYDOWN as usize || wparam.0 == WindowsAndMessaging::WM_SYSKEYDOWN as usize) {
            let kb_struct = unsafe { *(lparam.0 as *const WindowsAndMessaging::KBDLLHOOKSTRUCT) };
            let vk_code = kb_struct.vkCode;

            if vk_code == KeyboardAndMouse::VK_SPACE.0 as u32 {
                // 获取 PreviewFile 实例并处理按键事件
                if let Some(app) = unsafe { APP_INSTANCE.as_ref() } {
                    app.handle_key_down(vk_code);
                }
            } 
        }
        
        next_hook_result
    }

    pub fn preview_file(app: AppHandle) -> Result<(), TauriError> {
        let file_path = Selected::new();
        if file_path.is_some() {
            let file_path = file_path.unwrap();
            let file_info = get_file_info(&file_path);

            if file_info.is_none() {
                return Ok(());
            }

            let file_info = file_info.unwrap();

            let monitor_info = monitor::get_monitor_info();
            let scale = monitor_info.scale;
            
            let mut width = 1000.0;
            let mut height = 600.0;

            if monitor_info.width > 0.0 {
                let tmp_width = monitor_info.width * 0.8;
                let tmo_height = monitor_info.height * 0.8;
                log::info!("tmp_width: {}, tmo_height: {}", tmp_width, tmo_height);
                width = helper::get_scaled_size(tmp_width, scale);
                height = helper::get_scaled_size(tmo_height, scale);
            }

            match app.get_webview_window("preview") {
                Some(window) => {
                    let type_str = file_info.get_file_type();
                    let route = WebRoute::get_route(&type_str, &file_info);

                    let url = route.to_url();
                    let js = format!("window.location.href = '{}'", &url);
                    let _ = window.eval(js.as_str());

                    let _ = window.show();
                    let _ = window.set_focus();
                }
                None => {

                    let result = WebviewWindowBuilder::new(
                        &app,
                        "preview",
                        WebviewUrl::App("/preview".into()),
                    )
                    .center()
                    .decorations(false)
                    .skip_taskbar(false)
                    .auto_resize()
                    .inner_size(width, height)
                    .min_inner_size(300.0, 300.0)
                    .on_page_load(move |window, payload| {
                        let cur_path = payload.url().path();
                        if cur_path == "/preview" {
                            match payload.event() {
                                PageLoadEvent::Finished => {
                                    let type_str = file_info.get_file_type();
                                    let route = WebRoute::get_route(&type_str, &file_info);

                                    let url = route.to_url();
                                    let js = format!("window.location.href = '{}'", &url);
                                    let _ = window.eval(js.as_str());

                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                                _ => {}
                            }
                        }
                    })
                    .focused(true)
                    .visible_on_all_workspaces(true)
                    
                    .build();

                    if let Ok(preview) = result {
                        
                        let _ = preview.show();
                    }
                }
            }
        }

        Ok(())
    }

    pub fn new() -> Self {
        Self {
            hook_handle: None,
            app_handle: None,
        }
    }
}

static mut APP_INSTANCE: Option<PreviewFile> = None;
impl Drop for PreviewFile {
    fn drop(&mut self) {
        println!("Dropping PreviewFile instance");
        self.remove_keyboard_hook();
    }
}

impl Default for PreviewFile {
    fn default() -> Self {
        PreviewFile::new()
    }
}

//noinspection ALL
// 公开一个全局函数来初始化 PreviewFile
pub fn init_preview_file(handle: AppHandle) {
    let mut preview_file = PreviewFile::default();
    preview_file.set_keyboard_hook();
    preview_file.app_handle = Some(handle);
    // 将实例存储在全局变量中
    unsafe {
        APP_INSTANCE = Some(preview_file);
    }
}

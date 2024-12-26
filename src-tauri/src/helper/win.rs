use std::path::Path;
use windows::{
    core::{PCWSTR, PWSTR},
    Win32::{
        Foundation::{HWND, S_OK},
        System::Com,
        UI::{Shell, WindowsAndMessaging},
    },
};

#[allow(unused)]
pub fn get_window_class_name(hwnd: HWND) -> String {
    let mut class_name = [0u16; 256];
    let len = unsafe { WindowsAndMessaging::GetClassNameW(hwnd, &mut class_name) };
    String::from_utf16_lossy(&class_name[0..len as usize])
}

#[allow(unused)]
pub fn open_by_default(file_path: &str, hwnd: HWND) -> windows::core::Result<()> {
    // 将字符串转为 PCWSTR
    let file_path_wide: Vec<u16> = file_path.encode_utf16().chain(std::iter::once(0)).collect();
    let file_path_wide = PCWSTR(file_path_wide.as_ptr());

    // 配置 SHELLEXECUTEINFOW 结构
    let mut sei = Shell::SHELLEXECUTEINFOW {
        cbSize: std::mem::size_of::<Shell::SHELLEXECUTEINFOW>() as u32,
        lpFile: file_path_wide,              // 文件路径
        nShow: 1,                            // SW_SHOWNORMAL
        fMask: Shell::SEE_MASK_INVOKEIDLIST, // 打开方式对话框
        hwnd,                                // 父窗口句柄（可替换为实际窗口句柄）
        ..Default::default()
    };

    // 调用 ShellExecuteExW
    unsafe {
        Shell::ShellExecuteExW(&mut sei)?;
    }
    Ok(())
}

#[allow(unused)]
pub fn show_open_with_dialog(file_path: &str, hwnd: HWND) -> windows::core::Result<()> {
    // 初始化 COM
    unsafe { Com::CoInitializeEx(None, Com::COINIT_APARTMENTTHREADED) };

    // 配置 OPENASINFO 结构
    let file_path_wide: Vec<u16> = file_path.encode_utf16().chain(std::iter::once(0)).collect();
    let file_path_wide = PCWSTR(file_path_wide.as_ptr());
    let open_as_info = Shell::OPENASINFO {
        pcszFile: file_path_wide,
        pcszClass: PCWSTR::null(), // 文件类型 (可选)
        oaifInFlags: Shell::OAIF_ALLOW_REGISTRATION | Shell::OAIF_EXEC, // 允许注册 & 立即执行
    };

    // 调用 SHOpenWithDialog 打开“打开方式”对话框
    unsafe {
        Shell::SHOpenWithDialog(hwnd, &open_as_info)?;
    }

    // 释放 COM
    unsafe { Com::CoUninitialize() };

    Ok(())
}

pub fn get_default_program_name(path: &str) -> Result<String, String> {
    let path = Path::new(path);
    let ext = path.extension()
        .and_then(|ext| ext.to_str())
        .ok_or("No file extension".to_string())?;
    
    let ext = format!(".{}", ext);
    let ext_wide: Vec<u16> = ext.encode_utf16().chain(std::iter::once(0)).collect();
    
    unsafe {
        // 获取所需缓冲区大小
        let mut size = 0u32;
        let _ = Shell::AssocQueryStringW(
            Shell::ASSOCF_NONE,
            Shell::ASSOCSTR_FRIENDLYAPPNAME,
            PCWSTR(ext_wide.as_ptr()),
            None,
            PWSTR::null(),
            &mut size,
        );

        // 分配缓冲区并获取程序名
        let mut buffer = vec![0u16; size as usize];
        let result = Shell::AssocQueryStringW(
            Shell::ASSOCF_NONE,
            Shell::ASSOCSTR_FRIENDLYAPPNAME,
            PCWSTR(ext_wide.as_ptr()),
            None,
            PWSTR(buffer.as_mut_ptr()),
            &mut size,
        );

        if result != S_OK {
            return Err(format!("获取 name Buffer 失败 -> {:?}", result));
        }

        // 转换为字符串
        let name = String::from_utf16_lossy(&buffer[..size as usize - 1]);
        Ok(name)
    }
}
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging;

pub fn get_window_class_name(hwnd: HWND) -> String {
    let mut class_name = [0u16; 256];
    let len = unsafe { WindowsAndMessaging::GetClassNameW(hwnd, &mut class_name) };
    String::from_utf16_lossy(&class_name[0..len as usize])
}

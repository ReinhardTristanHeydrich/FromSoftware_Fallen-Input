use winapi::um::processthreadsapi::GetCurrentProcessId;
use winapi::um::winuser::{GetForegroundWindow, GetWindowThreadProcessId};

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn is_focused() -> bool {
    let focused_window = GetForegroundWindow();
    if focused_window.is_null() {return false;}

    let mut pid = 0;
    GetWindowThreadProcessId(focused_window, &mut pid);

    let current_pid = GetCurrentProcessId();
    pid == current_pid
}
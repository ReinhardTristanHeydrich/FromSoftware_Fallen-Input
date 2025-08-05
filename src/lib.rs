mod window;
use winapi::um::libloaderapi::DisableThreadLibraryCalls as DisableThreadCalls;
use inputbot::{KeybdKey::*, MouseButton::*};
use std::{thread::sleep, time::Duration};

const GAME_INJECTION:u32 = 1;

#[allow(unused_parens)]
#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn DllMain(hmodule: usize, reason: u32) -> bool {
    if reason != GAME_INJECTION {return true;}
    DisableThreadCalls(hmodule as *mut _);
    //==========================================
    // Bind your caps lock key to a function that starts an autoclicker.
    // Just using an exemple from InputBot's Repository's ReadMe for testing.
    Numpad0Key.bind(move || {
        while (Numpad0Key.is_toggled() && window::is_focused()) {
            LeftButton.press();
            LeftButton.release();

            sleep(Duration::from_millis(1000));
        }
    });
    //==========================================
    true
}


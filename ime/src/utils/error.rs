// hook panic messagebox

use windows::core::{w, Result, PCWSTR};
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK};

use super::winutils::to_wide_16;

use std::panic;

pub fn set_panic_hook() -> Result<()> {
    let default_hook = panic::take_hook();

    panic::set_hook(Box::new(move |info| {
        let message = info.to_string();

        unsafe {
            MessageBoxW(
                None,
                PCWSTR(to_wide_16(&message).as_ptr()),
                w!("Panic"),
                MB_OK,
            );
        }

        default_hook(info);
    }));

    Ok(())
}

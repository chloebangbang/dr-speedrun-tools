#![windows_subsystem = "windows"]

use std::ffi::c_void;

use windows::Win32::UI::WindowsAndMessaging;
use windows::Win32::UI::WindowsAndMessaging::SystemParametersInfoA;
use windows::Win32::UI::Accessibility::STICKYKEYS;
use windows::Win32::UI::Accessibility;

use rodio::{self, Source};

fn main() {
    let sticky_keys: *mut STICKYKEYS = Box::into_raw(Box::new(STICKYKEYS{cbSize: 8, dwFlags: Accessibility::STICKYKEYS_FLAGS(0)}));
    unsafe {
        let _ = SystemParametersInfoA(
        WindowsAndMessaging::SPI_GETSTICKYKEYS,
            8,
            Some(sticky_keys as *mut c_void),
            WindowsAndMessaging::SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        );
        
        (*sticky_keys).dwFlags.0 ^= 1;

        let _ = SystemParametersInfoA(
            WindowsAndMessaging::SPI_SETSTICKYKEYS,
            8,
            Some(sticky_keys as *mut c_void),
            WindowsAndMessaging::SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        );
        // sound system
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
            .expect("Failed to open sound device");
        let mixer = stream_handle.mixer();
        let g = rodio::source::SineWave::new(783.99)
            .take_duration(std::time::Duration::from_millis(75))
            .amplify(0.1);
        let d = rodio::source::SineWave::new(1174.66)
            .take_duration(std::time::Duration::from_millis(75))
            .amplify(0.1);
        if (*sticky_keys).dwFlags.0 % 2 == 0 {
            mixer.add(d);
            std::thread::sleep(std::time::Duration::from_millis(75));
            mixer.add(g);
        } else {
            mixer.add(g);
            std::thread::sleep(std::time::Duration::from_millis(75));
            mixer.add(d);
        }
        std::thread::sleep(std::time::Duration::from_millis(75));
    }
}

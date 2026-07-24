#![allow(dead_code)]
#![allow(unused_imports)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub use idle_api as runner;
mod cosmos;

#[cfg(test)]
mod tests_perf;

#[unsafe(no_mangle)]
pub extern "C" fn create_screensaver() -> *mut idle_api::ScreensaverInstance {
    let effect = cosmos::Cosmos::new();
    let instance = idle_api::ScreensaverInstance {
        inner: Box::new(effect),
    };
    Box::into_raw(Box::new(instance))
}

/// Destroys a screensaver instance created by `create_screensaver`.
///
/// # Safety
///
/// The caller must ensure that `ptr` is a valid pointer allocated by `create_screensaver` and has not been freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn destroy_screensaver(ptr: *mut idle_api::ScreensaverInstance) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr);
        }
    }
}

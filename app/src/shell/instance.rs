//! Single instance: a named mutex marks the running copy; a second launch sets a named event
//! that makes the first one show its window, then exits.
use std::thread;

#[cfg(windows)]
mod imp {
    use windows::Win32::Foundation::{ERROR_ALREADY_EXISTS, GetLastError, HANDLE, WAIT_OBJECT_0};
    use windows::Win32::System::Threading::{
        CreateEventW, CreateMutexW, EVENT_MODIFY_STATE, INFINITE, OpenEventW, SetEvent,
        WaitForSingleObject,
    };
    use windows::core::w;

    pub struct Handle(pub HANDLE);
    // A kernel handle may be used from any thread.
    unsafe impl Send for Handle {}

    /// `Some(show_event)` when this is the first instance.
    pub fn acquire() -> Option<Handle> {
        unsafe {
            let mutex = CreateMutexW(None, true, w!("Local\\com.oratakashi.wwm.instance"));
            if mutex.is_err() || GetLastError() == ERROR_ALREADY_EXISTS {
                if let Ok(ev) = OpenEventW(EVENT_MODIFY_STATE, false, w!("Local\\com.oratakashi.wwm.show")) {
                    let _ = SetEvent(ev);
                }
                return None;
            }
            // The mutex handle is never closed: it lives as long as the process.
            let event = CreateEventW(None, false, false, w!("Local\\com.oratakashi.wwm.show")).ok()?;
            Some(Handle(event))
        }
    }

    pub fn wait(h: &Handle) -> bool {
        unsafe { WaitForSingleObject(h.0, INFINITE) == WAIT_OBJECT_0 }
    }
}

/// Returns `false` when another copy is already running (it has been asked to show itself).
/// Otherwise `on_show` runs, on a background thread, every time a later launch asks for the window.
pub fn acquire(on_show: impl Fn() + Send + 'static) -> bool {
    #[cfg(windows)]
    {
        let Some(event) = imp::acquire() else { return false };
        thread::Builder::new()
            .name("single-instance".into())
            .spawn(move || {
                while imp::wait(&event) {
                    on_show();
                }
            })
            .expect("spawn single-instance thread");
        true
    }
    #[cfg(not(windows))]
    {
        let _ = (on_show, thread::current());
        true
    }
}

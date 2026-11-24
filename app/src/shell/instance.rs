//! Single instance: a named mutex marks the running copy; a second launch sets a named event
//! that makes the first one show its window, then exits.
use std::thread;

#[cfg(windows)]
mod imp {
    use windows::Win32::Foundation::{
        CloseHandle, ERROR_ALREADY_EXISTS, GetLastError, HANDLE, HLOCAL, LocalFree, WAIT_OBJECT_0,
    };
    use windows::Win32::Security::Authorization::ConvertSidToStringSidW;
    use windows::Win32::Security::{GetTokenInformation, TOKEN_QUERY, TOKEN_USER, TokenUser};
    use windows::Win32::System::Threading::{
        CreateEventW, CreateMutexW, EVENT_MODIFY_STATE, GetCurrentProcess, INFINITE, OpenEventW,
        OpenProcessToken, SetEvent, WaitForSingleObject,
    };
    use windows::core::{HSTRING, PWSTR};

    pub struct Handle(pub HANDLE);
    // A kernel handle may be used from any thread.
    unsafe impl Send for Handle {}

    /// The user's SID (`S-1-5-21-…`), so another account's process in the same session
    /// cannot take our names first. `None` if it cannot be read (the plain names are used).
    pub(super) fn user_sid() -> Option<String> {
        unsafe {
            let mut token = HANDLE::default();
            OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).ok()?;
            let mut len = 0u32;
            let _ = GetTokenInformation(token, TokenUser, None, 0, &mut len);
            // u64 elements: TOKEN_USER holds a pointer and must be aligned for it.
            let mut buf = vec![0u64; (len as usize).div_ceil(8)];
            let got = GetTokenInformation(token, TokenUser, Some(buf.as_mut_ptr().cast()), len, &mut len);
            let _ = CloseHandle(token);
            got.ok()?;
            let user = &*(buf.as_ptr() as *const TOKEN_USER);
            let mut text = PWSTR::null();
            ConvertSidToStringSidW(user.User.Sid, &mut text).ok()?;
            let sid = text.to_string().ok();
            let _ = LocalFree(Some(HLOCAL(text.0.cast())));
            sid
        }
    }

    /// `Local\com.oratakashi.wwm.<what>[.<sid>]`.
    fn name(what: &str, sid: Option<&str>) -> HSTRING {
        match sid {
            Some(sid) => HSTRING::from(format!(r"Local\com.oratakashi.wwm.{what}.{sid}")),
            None => HSTRING::from(format!(r"Local\com.oratakashi.wwm.{what}")),
        }
    }

    /// `Some(show_event)` when this is the first instance.
    pub fn acquire() -> Option<Handle> {
        let sid = user_sid();
        let (mutex_name, show_name) = (name("instance", sid.as_deref()), name("show", sid.as_deref()));
        unsafe {
            let mutex = CreateMutexW(None, true, &mutex_name);
            if mutex.is_err() || GetLastError() == ERROR_ALREADY_EXISTS {
                if let Ok(ev) = OpenEventW(EVENT_MODIFY_STATE, false, &show_name) {
                    let _ = SetEvent(ev);
                }
                return None;
            }
            // The mutex handle is never closed: it lives as long as the process.
            let event = CreateEventW(None, false, false, &show_name).ok()?;
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

#[cfg(all(test, windows))]
mod tests {
    #[test]
    fn names_carry_the_user_sid() {
        let sid = super::imp::user_sid().expect("the process token has a user SID");
        assert!(sid.starts_with("S-1-"), "{sid}");
    }
}

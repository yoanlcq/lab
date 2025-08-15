#[inline]
#[must_use]
pub fn is_attached() -> bool {
    #[cfg(windows)]
    #[expect(unsafe_code, reason = "Necessary")]
    // Safety: This is obviously safe, being a Win32 API that can be called at any time
    unsafe {
        windows::Win32::System::Diagnostics::Debug::IsDebuggerPresent().into()
    }
}

macro_rules! breakpoint {
    () => {
        if crate::debugger::is_attached() {
            #[cfg(windows)]
            #[expect(unsafe_code, reason = "Necessary")]
            // Safety: This is obviously safe, being a Win32 API that can be called at any time
            unsafe {
                windows::Win32::System::Diagnostics::Debug::DebugBreak()
            }
        }
    };
}

pub(crate) use breakpoint;

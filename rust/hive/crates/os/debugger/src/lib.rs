#![no_std]

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

#[macro_export]
macro_rules! breakpoint_even_if_not_attached {
    () => {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        // Safety: Well this is as "safe" as it gets
        unsafe { std::arch::asm!("int3"); }
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        TODO;
    };
}

#[macro_export]
macro_rules! breakpoint {
    () => {
        if $crate::is_attached() {
            $crate::breakpoint_even_if_not_attached!();
        }
    };
}
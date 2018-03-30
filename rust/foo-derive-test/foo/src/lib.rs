pub trait ZeroedMem : Sized {
    fn zeroed_mem() -> Self {
        unsafe {
            ::std::mem::zeroed()
        }
    }
}

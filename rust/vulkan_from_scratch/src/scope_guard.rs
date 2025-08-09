pub struct ScopeGuard<T: FnMut()>(T);

impl<T: FnMut()> ScopeGuard<T> {
    #[must_use]
    pub const fn new(f: T) -> Self {
        Self(f)
    }
}

impl<T: FnMut()> Drop for ScopeGuard<T> {
    fn drop(&mut self) {
        (self.0)();
    }
}

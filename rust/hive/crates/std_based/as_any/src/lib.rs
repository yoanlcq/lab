//! [Rationale](https://stackoverflow.com/a/33687996)
 
#![no_std]

use core::any::Any;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

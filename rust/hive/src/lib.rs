pub mod hive {
    use std::rc::Rc;
    use std::cell::{Cell, RefCell};

    mod imp {
        use std::{cell::{Cell, UnsafeCell}, mem::MaybeUninit};
        use std::pin::Pin;
        
        #[derive(Debug)]
        pub struct PinArena<T> {
            memory: Box<[UnsafeCell<MaybeUninit<T>>]>,
            len: Cell<usize>,
        }

        impl <T> PinArena<T> {
            pub fn with_capacity(cap: usize) -> Self {
                assert_ne!(cap, 0);
                Self {
                    memory: (0..cap).map(|_| UnsafeCell::new(MaybeUninit::uninit())).collect::<Vec<_>>().into_boxed_slice(),
                    len: Cell::new(0),
                }
            }
            pub fn capacity(&self) -> usize {
                self.memory.len()
            }
            pub fn is_full(&self) -> bool {
                self.len.get() >= self.capacity()
            }
            pub fn try_push(&self, val: T) -> Option<Pin<&mut T>> {
                if self.is_full() {
                    None
                } else {
                    unsafe { Some(self.push_unchecked(val)) }
                }
            }
            pub unsafe fn push_unchecked(&self, val: T) -> Pin<&mut T> {
                let i = self.len.get();
                self.len.set(i + 1);
                Pin::new_unchecked((*self.memory[i].get()).write(val))
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Hive<T> {
        items: Vec<T>,
        nodes: RefCell<Vec<Rc<imp::PinArena<T>>>>,
        lock_counter: Cell<usize>,
    }

    impl<T> Hive<T> {
        pub fn add(&self, item: T) {
            if !self.is_locked() {
                self.items.push(item);
            } else {
                let mut vv = self.nodes.borrow_mut();
                if vv.is_empty() || vv.last().unwrap().is_full() {
                    vv.push(Rc::new(imp::PinArena::with_capacity(64)));
                }
                unsafe { vv.last_mut().unwrap().push_unchecked(item); }
            }
        }
        pub fn is_locked(&self) -> bool {
            self.lock_counter.get() > 0
        }
        pub fn increment_lock_counter(&self) {
            self.lock_counter.set(self.lock_counter.get() + 1);
        }
        pub fn decrement_lock_counter(&self) {
            self.lock_counter.set(self.lock_counter.get() - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
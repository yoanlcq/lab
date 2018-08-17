// use std::marker::PhantomData;

use std::ptr;
use std::mem;
use std::ops::{Index, IndexMut};

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct K(u64);

impl K {
    #[inline] pub fn with_index(i: usize) -> Self { Self::with_index_and_version(i, 0) }
    #[inline] pub fn with_index_and_version(i: usize, v: u32) -> Self { K(i as u64 | ((v as u64) << 32)) }
    #[inline] pub fn index(&self) -> usize { self.0 as usize & 0xffffffff }
    #[inline] pub fn version(&self) -> u32 { (self.0 >> 32) as u32 }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Info(u32);

impl Info {
    #[inline] fn new_occupied() -> Self     { Info(0) }
    #[inline] fn version(&self) -> u32      {  self.0 & 0x7fffffff }
    #[inline] fn is_vacant(&self) -> bool   { (self.0 & 0x80000000) != 0 }
    #[inline] fn is_occupied(&self) -> bool { (self.0 & 0x80000000) == 0 }
    #[inline] fn make_vacant(&mut self)     { self.0 |= 0x80000000; }
    #[inline] fn make_occupied_and_increment_version(&mut self) { self.0 += 1; self.0 &= 0x7fffffff; }
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub struct SparseMap<V> {
    // Guarantee that all elements are tightly packed for fast iteration on all occupied slots.
    items: Vec<V>,
    infos: Vec<Info>,
    frees: Vec<u32>,
}

impl<V> Drop for SparseMap<V> {
    fn drop(&mut self) {
        self.forget_vacant_items_before_dropping_everything();
    }
}

impl<V> Index<K> for SparseMap<V> {
    type Output = V;
    #[inline]
    fn index(&self, k: K) -> &V {
        self.get(k).unwrap()
    }
}

impl<V> IndexMut<K> for SparseMap<V> {
    #[inline]
    fn index_mut(&mut self, k: K) -> &mut V {
        self.get_mut(k).unwrap()
    }
}


impl<V> SparseMap<V> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }
    pub fn with_capacity(cap: usize) -> Self {
        Self { 
            items: Vec::with_capacity(cap),
            infos: Vec::with_capacity(cap),
            frees: Vec::new(),
        }
    }
    pub fn capacity(&self) -> usize {
        self.items.capacity()
    }
    pub fn reserve(&mut self, additional: usize) {
        self.items.reserve(additional);
        self.infos.reserve(additional);
    }
    pub fn reserve_free_indices(&mut self, additional: usize) {
        self.frees.reserve(additional);
    }
    pub fn contains_key(&self, k: K) -> bool {
        match self.infos.get(k.index()) {
            None => false,
            Some(info) => info.is_occupied() && info.version() == k.version(),
        }
    }
    #[inline]
    pub unsafe fn get_unchecked(&self, k: K) -> &V {
        &self.items[k.index()]
    }
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, k: K) -> &mut V {
        &mut self.items[k.index()]
    }
    pub fn get(&self, k: K) -> Option<&V> {
        if self.contains_key(k) {
            Some(unsafe { self.get_unchecked(k) })
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, k: K) -> Option<&mut V> {
        if self.contains_key(k) {
            Some(unsafe { self.get_unchecked_mut(k) })
        } else {
            None
        }
    }
    pub fn remove(&mut self, k: K) -> Option<V> {
        if self.contains_key(k) {
            let i = k.index();
            self.infos[i].make_vacant();
            self.frees.push(i as u32);
            let item = unsafe { 
                ptr::read(&self.items[i]) 
            };
            Some(item)
        } else {
            None
        }
    }
    pub fn push(&mut self, v: V) -> K {
        debug_assert_eq!(self.items.len(), self.infos.len());
        let i = self.items.len();
        self.items.push(v);
        self.infos.push(Info::new_occupied());
        K::with_index(i)
    }
    pub fn insert(&mut self, v: V) -> K {
        match self.frees.pop() {
            None => self.push(v),
            Some(i) => {
                let i = i as usize;
                let info = &mut self.infos[i];
                debug_assert!(info.is_vacant());
                info.make_occupied_and_increment_version();
                mem::forget(mem::replace(&mut self.items[i], v));
                K::with_index_and_version(i, info.version())
            },
        }
    }
    pub fn keys(&self) -> Keys {
        Keys::new(self)
    }
    pub fn values(&self) -> Values<V> {
        Values::new(self)
    }
    pub fn values_mut(&mut self) -> ValuesMut<V> {
        ValuesMut::new(self)
    }
    pub fn iter(&self) -> Iter<V> {
        Iter::new(self)
    }
    pub fn iter_mut(&mut self) -> IterMut<V> {
        IterMut::new(self)
    }
    pub fn len(&self) -> usize { self.items.len() - self.frees.len() }
    pub fn is_empty(&self) -> bool { self.len() == 0 }
    pub fn clear(&mut self) {
        self.forget_vacant_items_before_dropping_everything();
        self.items.clear();
        self.infos.clear();
        self.frees.clear();
    }
    fn forget_vacant_items_before_dropping_everything(&mut self) {
        for free in &self.frees {
            mem::forget(self.items.swap_remove(*free as _));
        }
    }
}

#[derive(Debug)]
pub struct Keys<'a> {
    infos: &'a [Info],
    frees: &'a [u32],
}

#[derive(Debug)]
pub struct Values<'a, V: 'a> {
    c: &'a SparseMap<V>,
}

#[derive(Debug)]
pub struct ValuesMut<'a, V: 'a> {
    c: &'a mut SparseMap<V>,
}

#[derive(Debug)]
pub struct Iter<'a, V: 'a> {
    c: &'a SparseMap<V>,
}

#[derive(Debug)]
pub struct IterMut<'a, V: 'a> {
    c: &'a mut SparseMap<V>,
}

impl<'a> Keys<'a> { pub fn new<V>(c: &'a SparseMap<V>) -> Self { Self { infos: &c.infos, frees: &c.frees, } } }
impl<'a, V> Values<'a, V> { pub fn new(c: &'a SparseMap<V>) -> Self { Self { c } } }
impl<'a, V> ValuesMut<'a, V> { pub fn new(c: &'a mut SparseMap<V>) -> Self { Self { c } } }
impl<'a, V> Iter<'a, V> { pub fn new(c: &'a SparseMap<V>) -> Self { Self { c } } }
impl<'a, V> IterMut<'a, V> { pub fn new(c: &'a mut SparseMap<V>) -> Self { Self { c } } }


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

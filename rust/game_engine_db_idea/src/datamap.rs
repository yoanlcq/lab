use std::ops::{Index, IndexMut, Range};
use std::slice;

/// A data-driven, associative data structure optimized for fast traversal, retrieval by key, insertion and
/// removal, at the cost of extra memory for bookkeeping.
///
/// Main selling points:
/// - Fast traversal of all items; All items are kept tightly packed in a vector, so traversal is as
///   cache-efficient as possible.
/// - Fast retrieval by key: Literally two array accesses.
/// - Fast removal by key; Three array accesses.
/// - Fast insertion; Either one array access into the free list or one `push`, then two `push`.
/// - Data-driven (untyped); This makes it suitable when the item type (and size) are not known at
///   compile-time, for instance when the types are defined dynamically by users (or data files) via some runtime
///   system. Obviously, all elements should be Plain Old Data, implying they are `Copy`.
///
/// For the data-driven use case, this data structure seemingly ticks all the boxes,
/// but a price has to be paid for these advantages.
///
/// Cons:
/// - Needs 12 extra bytes of memory per element;
///   Also, allocates 4 extra bytes (add an index to a free list) every time an element is removed
///   (these bytes are reclaimed at the next insertion).
///   This is the main disadvantage. It is what allows all operations to be fast, but consumes a
///   bunch of memory.
/// - Keys are 64-bit, as they use a generation number and index.
/// - You cannot sort the slice of items directly, otherwise this would break referential
///   integrity, so both the slice AND the bookeeping structures have to be kept in sync.
/// - The main overhead remains fetches from main RAM. Retrieval incurs two array accesses instead
///   of just one, because items are kept tightly packed and therefore an extra level of
///   indirection is needed.
///
/// In general, this structure favors theoretical speed over space.
///
///
/// References:
///
/// - [_Data Structures for Game Developers: The Slot Map_ by Sean Middleditch](http://seanmiddleditch.com/data-structures-for-game-developers-the-slot-map/);
/// - [The `slotmap` crate](https://crates.io/crates/slotmap);
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub struct DenseDataMap {
    item_size: usize,
    // Guarantee that all elements are tightly packed for fast iteration on all occupied slots.
    pool: Vec<u8>,  // Indexed by Info::index(). Retrieves a data item.
    ofni: Vec<u32>, // Indexed by Info::index(). Retrieves an info index.
    info: Vec<Info>, // Indexed by K::index().
    free: Vec<u32>, // List of info indices which were used at some point but are now available.
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Info {
    // Actually stores both a 1-bit "is_vacant" boolean + 31-bit generation
    generation: u32,
    index: u32,
}

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DataMapKey {
    generation: u32,
    index: u32,
}

use self::DataMapKey as K;

impl Index<Range<usize>> for DenseDataMap {
    type Output = [u8];
    fn index(&self, i: Range<usize>) -> &[u8] {
        &self.pool[self.items_range(i)]
    }
}

impl IndexMut<Range<usize>> for DenseDataMap {
    fn index_mut(&mut self, i: Range<usize>) -> &mut [u8] {
        let r = self.items_range(i);
        &mut self.pool[r]
    }
}

impl Index<usize> for DenseDataMap {
    type Output = [u8];
    #[inline]
    fn index(&self, i: usize) -> &[u8] {
        &self[i .. i+1]
    }
}

impl IndexMut<usize> for DenseDataMap {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut [u8] {
        &mut self[i .. i+1]
    }
}


impl DenseDataMap {
    pub fn new(item_size: usize) -> Self {
        Self::with_capacity(item_size, 0)
    }
    pub fn with_capacity(item_size: usize, cap: usize) -> Self {
        Self::with_capacity_and_free_list_capacity(item_size, cap, 0)
    }
    pub fn with_capacity_and_free_list_capacity(item_size: usize, cap: usize, free_cap: usize) -> Self {
        let cap = cap * item_size;
        let free_cap = free_cap * item_size;
        Self {
            item_size,
            pool: Vec::with_capacity(cap),
            ofni: Vec::with_capacity(cap),
            info: Vec::with_capacity(cap),
            free: Vec::with_capacity(free_cap),
        }
    }
    #[inline]
    fn items_range(&self, i: Range<usize>) -> Range<usize> {
        (i.start * self.item_size) .. (i.end * self.item_size)
    }
    #[inline]
    pub fn len(&self) -> usize {
        assert_eq!(self.pool.len() % self.item_size, 0);
        self.pool.len() / self.item_size
    }
    pub fn insert_uninitialized(&mut self) -> (K, &mut [u8]) {
        let i = self.len();
        let (info_i, info) = match self.free.pop() {
            None => {
                let (info_i, info) = (self.info.len(), Info::new_occupied(i));
                self.info.push(info);
                (info_i, info)
            },
            Some(info_i) => {
                let info = &mut self.info[info_i as usize];
                info.occupy(i);
                (info_i as _, *info)
            },
        };
        self.pool.reserve(self.item_size);
        unsafe {
            let new_len = self.pool.len() + self.item_size;
            self.pool.set_len(new_len);
        }
        self.ofni.push(info_i as _);
        (K::with_index_and_generation(info_i, info.generation()), &mut self[i])
    }
    pub fn insert_zeroed(&mut self) -> K {
        let (k, mem) = self.insert_uninitialized();
        unsafe {
            ::std::ptr::write_bytes(mem.as_mut_ptr(), 0, mem.len());
        }
        k
    }
    pub fn insert(&mut self, data: &[u8]) -> K {
        let (k, mem) = self.insert_uninitialized();
        mem.copy_from_slice(data);
        k
    }
    pub fn remove(&mut self, k: K) {
        if self.contains_key(k) {
            unsafe {
                self.remove_unchecked(k)
            }
        }
    }
    pub unsafe fn remove_unchecked(&mut self, k: K) {
        self.remove_with_info_i(k.index())
    }
    pub fn swap_remove(&mut self, i: usize) {
        let info_i = self.ofni[i as usize];
        unsafe {
            self.remove_with_info_i(info_i as _)
        }
    }
    unsafe fn remove_with_info_i(&mut self, info_i: usize) {
        let last_i = self.len() - 1;
        let i = self.info[info_i].index();
        self.swap_remove_in_pool(i);
        self.info[self.ofni[last_i] as usize].set_index(i);
        self.info[info_i].make_vacant();
        self.free.push(info_i as _);
    }
    fn swap_remove_in_pool(&mut self, i: usize) {
        assert!(i < self.len());
        unsafe {
            let last_i = self.len() - 1;
            let src = slice::from_raw_parts(self[last_i].as_ptr(), self.item_size);
            self[i].copy_from_slice(src);
            let new_len = self.pool.len() - self.item_size;
            self.pool.set_len(new_len);
        }
    }
    #[inline]
    pub fn contains_key(&self, k: K) -> bool {
        match self.info.get(k.index()) {
            None => false,
            Some(info) => info.is_occupied() && info.generation() == k.generation(),
        }
    }
    #[inline]
    pub unsafe fn get_unchecked(&self, k: K) -> &[u8] {
        &self[self.info[k.index()].index()]
    }
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, k: K) -> &mut [u8] {
        let i = self.info[k.index()].index();
        &mut self[i]
    }
    pub fn get(&self, k: K) -> Option<&[u8]> {
        if self.contains_key(k) {
            Some(unsafe { self.get_unchecked(k) })
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, k: K) -> Option<&mut [u8]> {
        if self.contains_key(k) {
            Some(unsafe { self.get_unchecked_mut(k) })
        } else {
            None
        }
    }
    pub fn key_at(&self, i: usize) -> K {
        let info_i = self.ofni[i] as usize;
        K::with_index_and_generation(info_i, self.info[info_i].generation())
    }
}


impl K {
    #[inline] pub fn with_index(i: usize) -> Self { Self::with_index_and_generation(i, 0) }
    #[inline] pub fn with_index_and_generation(index: usize, generation: u32) -> Self { Self { index: index as _, generation } }
    #[inline] pub fn index(&self) -> usize { self.index as _ }
    #[inline] pub fn generation(&self) -> u32 { self.generation }
}

impl Info {
    #[inline] fn new_occupied(index: usize) -> Self { Info { generation: 0, index: index as _, } }
    #[inline] fn index(&self) -> usize { self.index as _ }
    #[inline] fn set_index(&mut self, i: usize) { self.index = i as _; }
    #[inline] fn generation(&self) -> u32   {  self.generation  & 0x7fffffff }
    #[inline] fn is_vacant(&self) -> bool   { (self.generation  & 0x80000000) != 0 }
    #[inline] fn is_occupied(&self) -> bool { (self.generation  & 0x80000000) == 0 }
    #[inline] fn make_vacant(&mut self)     {  self.generation |= 0x80000000; }
    #[inline] fn occupy(&mut self, i: usize) {
        debug_assert!(self.is_vacant());
        self.index = i as _;
        self.generation = self.generation.wrapping_add(1);
        self.generation &= 0x7fffffff;
    }
}


impl DenseDataMap {
    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }
    pub fn iter_mut(&mut self) -> IterMut {
        IterMut::new(self)
    }
    pub fn keys(&self) -> Keys {
        Keys::new(self)
    }
    pub fn values(&self) -> Values {
        Values::new(self)
    }
    pub fn values_mut(&mut self) -> ValuesMut {
        ValuesMut::new(self)
    }
}

#[derive(Debug)] pub struct Iter     <'a> { map: &'a     DenseDataMap, i: usize, }
#[derive(Debug)] pub struct IterMut  <'a> { map: &'a mut DenseDataMap, i: usize, }
#[derive(Debug)] pub struct Keys     <'a> { map: &'a     DenseDataMap, i: usize, }
#[derive(Debug)] pub struct Values   <'a> { map: &'a     DenseDataMap, i: usize, }
#[derive(Debug)] pub struct ValuesMut<'a> { map: &'a mut DenseDataMap, i: usize, }

impl<'a> Iter     <'a> { fn new(map: &'a     DenseDataMap) -> Self { Self { map, i: 0, } } }
impl<'a> IterMut  <'a> { fn new(map: &'a mut DenseDataMap) -> Self { Self { map, i: 0, } } }
impl<'a> Keys     <'a> { fn new(map: &'a     DenseDataMap) -> Self { Self { map, i: 0, } } }
impl<'a> Values   <'a> { fn new(map: &'a     DenseDataMap) -> Self { Self { map, i: 0, } } }
impl<'a> ValuesMut<'a> { fn new(map: &'a mut DenseDataMap) -> Self { Self { map, i: 0, } } }

impl<'a> Iterator for Iter<'a> {
    type Item = (K, &'a [u8]);
    fn next(&mut self) -> Option<(K, &'a [u8])> {
        if self.i >= self.map.len() {
            return None;
        }
        let next = (self.map.key_at(self.i), &self.map[self.i]);
        self.i += 1;
        Some(next)
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = (K, &'a mut [u8]);
    fn next(&mut self) -> Option<(K, &'a mut [u8])> {
        if self.i >= self.map.len() {
            return None;
        }
        // Trick borrowck into believing this slice comes from "somewhere else" than our
        // collection, because it can't prove that the slices we return are all disjoint,
        // which is required to satisfy Rust's aliasing rules.
        let s = unsafe {
            let s = &mut self.map[self.i];
            slice::from_raw_parts_mut(s.as_mut_ptr(), s.len())
        };
        let next = (self.map.key_at(self.i), s);
        self.i += 1;
        Some(next)
    }
}

impl<'a> Iterator for Keys<'a> {
    type Item = K;
    fn next(&mut self) -> Option<K> {
        if self.i >= self.map.len() {
            return None;
        }
        let next = self.map.key_at(self.i);
        self.i += 1;
        Some(next)
    }
}

impl<'a> Iterator for Values<'a> {
    type Item = &'a [u8];
    fn next(&mut self) -> Option<&'a [u8]> {
        if self.i >= self.map.len() {
            return None;
        }
        let next = &self.map[self.i];
        self.i += 1;
        Some(next)
    }
}

impl<'a> Iterator for ValuesMut<'a> {
    type Item = &'a mut [u8];
    fn next(&mut self) -> Option<&'a mut [u8]> {
        if self.i >= self.map.len() {
            return None;
        }
        // Trick borrowck into believing this slice comes from "somewhere else" than our
        // collection, because it can't prove that the slices we return are all disjoint,
        // which is required to satisfy Rust's aliasing rules.
        let next = unsafe {
            let s = &mut self.map[self.i];
            slice::from_raw_parts_mut(s.as_mut_ptr(), s.len())
        };
        self.i += 1;
        Some(next)
    }
}

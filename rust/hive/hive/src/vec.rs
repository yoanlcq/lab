#![allow(dead_code, unused_variables, unreachable_code, unused_mut)]

use std::{iter::{Filter, Skip, Take}, marker::PhantomData, ops::{Deref, DerefMut}};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct RefCounter {
    refs: usize,
    refmuts: usize,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct HiveVec<T> {
    phantom: PhantomData<T>,
    settled_item_refs : RefCounter,
    pending_add_item_refs : RefCounter,
    settled_range_refs: RefCounter,
    pending_add_range_refs: RefCounter,
}

impl<T> Default for HiveVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

fn demo_how_to_handle_newly_added_items_differently<T>(v: &HiveVec<T>) {
    let mut fresh_iter = v.iter().start_at_current_len();
    for _it in v.iter().stop_at_current_len() {
        // do operation on already existing items, which could possibly add new items
    }
    loop {
        if fresh_iter.len() == 0 {
            break;
        }
        let new_fresh_iter = v.iter().start_at_current_len();
        for _it in fresh_iter {
            // do operation on newly added items... Which... Could also possibly add new items...
        }
        fresh_iter = new_fresh_iter;
    }
}

impl<T> HiveVec<T> {
    pub fn new() -> Self {
        todo!()
    }
    pub fn len_including_pending_removals(&self) -> usize {
        self.num_settled_plus_pending_add()
    }
    pub fn len_excluding_pending_removals(&self) -> usize {
        self.len_including_pending_removals() - self.num_pending_removal()
    }
    pub fn num_settled_plus_pending_add(&self) -> usize {
        self.num_settled() + self.num_pending_add()
    }
    pub fn num_settled(&self) -> usize {
        todo!()
    }
    pub fn num_pending_add(&self) -> usize {
        todo!()
    }
    pub fn num_pending_removal(&self) -> usize {
        self.num_settled_with_pending_removal_value(true) + self.num_pending_add_with_pending_removal_value(true)
    }
    pub fn num_settled_pending_removal(&self) -> usize {
        todo!()
    }
    pub fn num_pending_add_pending_removal(&self) -> usize {
        todo!()
    }
    pub fn num_settled_not_pending_removal(&self) -> usize {
        self.num_settled() - self.num_settled_pending_removal()
    }
    pub fn num_pending_add_not_pending_removal(&self) -> usize {
        self.num_pending_add() - self.num_pending_add_pending_removal()
    }
    // Returns the number of items where `item.is_pending_removal() == pending_removal`
    pub fn num_with_pending_removal_value(&self, pending_removal: bool) -> usize {
        if pending_removal {
            self.num_pending_removal()
        } else {
            self.len_excluding_pending_removals()
        }
    }
    // Returns the number of settled items where `item.is_pending_removal() == pending_removal`
    pub fn num_settled_with_pending_removal_value(&self, pending_removal: bool) -> usize {
        if pending_removal {
            self.num_settled_pending_removal()
        } else {
            self.num_settled_not_pending_removal()
        }
    }
    // Returns the number of pending add items where `item.is_pending_removal() == pending_removal`
    pub fn num_pending_add_with_pending_removal_value(&self, pending_removal: bool) -> usize {
        if pending_removal {
            self.num_pending_add_pending_removal()
        } else {
            self.num_pending_add_not_pending_removal()
        }
    }
    // - If the container is borrowed: this will conceptually enqueue a command to add the new item.
    //   Iterators may be configured to also evaluate items that are pending add, but the caller should note that such items do not respect the container's ordering requirement.
    //   When that command is executed, the item will be added in a way that respects the container's ordering requirement.
    //   Internally, sorting the container might happen in a "lazy" fashion, to ensure that consecutives calls to add() are fast.
    // - If the container is not borrowed, the behavior is as if the command was enqueued then executed immediately.
    pub fn add(&self, val: T) {
        todo!()
    }
    pub fn add_and_get(&self, val: T) -> ItemAccessor<T> {
        todo!()
    }
    // Same as add() but possibly faster thanks to the compile-time guarantee that nobody else is borrowing the container.
    pub fn add_mut(&mut self, val: T) -> &mut T {
        todo!()
    }
    // - If the container is borrowed: this will conceptually enqueue a command to remove the new item.
    //   Iterators may be configured to skip items that are pending removal.
    //   When that command is executed, the item will be removed in a way that respects the container's ordering requirement.
    // - If the container is not borrowed, the behavior is as if the command was enqueued then executed immediately.
    pub fn remove(&self, index: usize) -> Option<T> {
        todo!()
    }
    // Enqueues a command to remove the item and apply a function on the item after it is removed.
    pub fn remove_and<F: FnMut(T)>(&self, index: usize, mut f: F) -> bool {
        todo!()
    }
    // Same as remove() but possibly faster thanks to the compile-time guarantee that nobody else is borrowing the container.
    pub fn remove_mut(&mut self, index: usize) -> T {
        todo!()
    }
    pub fn try_remove_mut(&mut self, index: usize) -> Option<T> {
        todo!()
    }
    // TODO: set ordering requirement:
    // - None (no ordering whatsoever. Adds to the end. Removes via remove_swap)
    // - Sequential (Adds to the end. Removes via remove_shift)
    // - Custom (user-specified comparison predicate + swap callback)
    //
    // Rules the user has to follow:
    // - If the comparison predicate depends on external conditions, the user is responsible for marking the container as dirty when that condition changes.
    //   Otherwise the container would have to constantly sort itself, which is highly counterproductive.
    // - When an item is modified in a way that invalidates its order in the container, the user is responsible for marking the item (or the container) as dirty.
    //   The container does not try to "guess" or do this automatically, for the following reasons:
    //   - Dereferencing a mutable reference to item does not prove that it will be modified;
    //   - Immutable item refs may be modified if they use interior mutability;
    //   - An item may indirectly reference data that contributes to its order;
    //   - Therefore, only the user can know when a mutation affects the order.
    //
    // The move/swap callback is used for the following:
    // - Keeping track of when items are moved within the container
    // - Syncing the order of other containers with this one
    // - Allows maintaining ranges of items that satisfy a subset of the ordering predicate.
    //   For instance, if your ordering predicate separates oranges from bananas, you can maintain a cached index to the gap between the two sets; you can then quickly iterate over either oranges or bananas using that index.
    //   But remember, while you iterate on the container, you may encounter pending adds, and those cannot respect the container's order until they are settled.

    // TODO: iterator provides:
    // - Is the item pending add? (self.include_pending_adds && index < nb_settled)
    // - Is the item pending removal? (self.include_pending_removals && is_pending_removal[i])
    // - RefCell-like API to access the item
    //
    // TODO: iterator should have a fast implementation for nth(), fold(), size_hint(), etc, just like slice::Iter.

    // Due to proven exclusive access, all commands have been flushed, so all items are settled.
    pub fn into_iter(self) -> ValueIterator<T> {
        todo!()
    }
    // Due to proven exclusive access, all commands have been flushed, so all items are settled.
    pub fn iter_mut(&mut self) -> ExclusiveIterator<T> {
        todo!()
    }
    // This will iterate over all settled items, then all current and future pending adds
    pub fn iter(&self) -> SharedIterator<T> {
        todo!()
    }

    pub fn can_borrow_range(&self, filter: RangeFilter) -> bool {
        todo!()
    }
    pub fn can_borrow_range_mut(&self, filter: RangeFilter) -> bool {
        todo!()
    }

    pub fn try_borrow_range(&self, filter: RangeFilter) -> Option<RangeRef<T>> {
        if !self.can_borrow_range(filter) {
            return None;
        }
        todo!()
    }
    pub fn try_borrow_range_mut(&self, filter: RangeFilter) -> Option<RangeRefMut<T>> {
        if !self.can_borrow_range_mut(filter) {
            return None;
        }
        // TODO: should flush all pending commands, then sort immediately, because this is proof of an imminent access
        todo!()
    }

    pub fn enqueue_command<R, F: FnMut() -> R>(&self, mut f: F) -> Option<R> {
        // TODO: if self is locked, enqueue the command and return None. Otherwise execute immediately and return Some(r)
        todo!()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct RangeRef<'a, T> {
    it: SharedIterator<'a, T>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct RangeRefMut<'a, T> {
    it: SharedIterator<'a, T>,
}

// TODO: implement DoubleEndedIterator, ExactSizeIterator etc...
impl<'a, T> Iterator for RangeRef<'a, T> {
    type Item = ItemRef<T>;
    fn next(&mut self) -> Option<Self::Item> {
        Some(unsafe { self.it.next()?.borrow_unchecked_via_range_guard(self) })
    }
}

impl<'a, T> Iterator for RangeRefMut<'a, T> {
    type Item = ItemRefMut<T>;
    fn next(&mut self) -> Option<Self::Item> {
        Some(unsafe { self.it.next()?.borrow_mut_unchecked_via_range_guard(self) })
    }
}

impl<'a, T> Drop for RangeRef<'a, T> {
    fn drop(&mut self) {
        todo!()
    }
}

impl<'a, T> Drop for RangeRefMut<'a, T> {
    fn drop(&mut self) {
        todo!()
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RangeFilter {
    include_settled_items: bool,
    include_pending_adds: bool,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ValueIterator<T> {
    phantom: PhantomData<T>,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExclusiveIterator<'a, T> {
    phantom: PhantomData<&'a mut T>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SharedIterator<'a, T> {
    // TODO: this should lock the FatassVec, because we don't want any pending removal applied behind us (may cause us to iterate other some items twice)
    container: &'a HiveVec<T>,
    end_index: Option<usize>,
}

impl<'a, T> Drop for SharedIterator<'a, T> {
    fn drop(&mut self) {
        todo!()
    }
}

// TODO: implement DoubleEndedIterator, ExactSizeIterator etc...
impl<T> Iterator for ValueIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

// TODO: implement DoubleEndedIterator, ExactSizeIterator etc...
impl<'a, T> Iterator for ExclusiveIterator<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

// TODO: implement as many itertaor traits as possible
//
// TODO: investigate how well Rust supports having an ExactSizeIterator which size changes during iteration. Hopefully it doesn't cache the returned len? According to the docs, ExactSizeIterator::len(): "The implementation ensures that the iterator will return exactly len() more times a Some(T) value, before returning None.". This says "exactly", not "at least".
impl<'a, T> Iterator for SharedIterator<'a, T> {
    type Item = ItemAccessor<T>;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len_impl();
        // Uphold the guarantee required in the source of std's `ExactSizeIterator::len()`: they do `assert_eq!(upper, Some(lower))`.
        // Even though this contradicts the part where size_hint() may return a None upper bound if it is not known (I mean, we know it at the instant we call this, but it doesn't mean that it won't ever change, since the owning collection supports adding during iteration)
        (len, Some(len))
    }
    fn count(self) -> usize
        where
            Self: Sized, {
        self.len_impl()
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        todo!()
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }

    fn fold<B, F>(self, init: B, mut f: F) -> B
        where
            F: FnMut(B, Self::Item) -> B,
    {
        //
        // NOTE(Yoan): stolen from std::slice::Iter
        //
        // this implementation consists of the following optimizations compared to the
        // default implementation:
        // - do-while loop, as is llvm's preferred loop shape,
        //   see https://releases.llvm.org/16.0.0/docs/LoopTerminology.html#more-canonical-loops
        // - bumps an index instead of a pointer since the latter case inhibits
        //   some optimizations, see #111603
        // - avoids Option wrapping/matching
        if self.len_impl() == 0 {
            return init;
        }
        let mut acc = init;
        let mut i = 0usize;
        loop {
            // SAFETY: the loop iterates `i in 0..len`, which always is in bounds of
            // the slice allocation
            acc = f(acc, todo!());
            // SAFETY: `i` can't overflow since it'll only reach usize::MAX if the
            // slice had that length, in which case we'll break out of the loop
            // after the increment
            i = unsafe { i.unchecked_add(1) };
            if i == self.len_impl() {
                break;
            }
        }
        acc
    }
}

impl<'a, T> ExactSizeIterator for SharedIterator<'a, T> {
    fn len(&self) -> usize {
        self.len_impl()
    }
}

impl<'a, T> DoubleEndedIterator for SharedIterator<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.end_index.expect("Can't iterate backwards in the current state; it wouldn't make sense as items can be added to the back during iteration. If you really intend to iterate backwards ignoring newly added items, use one of the provided methods that sets self.end_index, ensuring that your intent is well-specified");
        todo!()
    }
}

impl<'a, T> SharedIterator<'a, T> {
    fn len_impl(&self) -> usize {
        todo!()
    }
    pub fn try_take_within_current_bounds(mut self, n: usize) -> Option<Take<Self>> {
        let end_index = n.saturating_add(todo!());
        self.try_take_within_current_bounds_via_end_index(end_index)
    }
    pub fn try_take_within_current_bounds_via_end_index(mut self, end_index: usize) -> Option<Take<Self>> {
        if end_index <= self.container.len_including_pending_removals() {
            self.end_index = Some(end_index);
            Some(self.take(todo!()))
        } else {
            None
        }
    }
    pub fn take_within_current_bounds(self, n: usize) -> Take<Self> {
        self.try_take_within_current_bounds(n).expect("end_index must be in bounds, because this function is used for unlocking backwards iteration")
    }
    // This is different from `take()` because `take()` explicitly supports numbers that may be greater than the current remaining number of iterations.
    // This function makes use of the knowledge that the user is uninterested in items that are added to the back during iteration, to "unlock" the backwards iteration feature.
    // That is, if you want to iterate backwards from one end of the container, this API forces you to be explicit about ignoring newly added items, because the implementation cannot support it in a way that is efficient or even makes sense.
    pub fn take_within_current_bounds_via_end_index(self, end_index: usize) -> Take<Self> {
        self.try_take_within_current_bounds_via_end_index(end_index).expect("end_index must be in bounds, because this function is used for unlocking backwards iteration")
    }
    pub fn stop_at_pending_adds(self) -> Take<Self> {
        let end_index = self.container.num_settled();
        self.take_within_current_bounds_via_end_index(end_index)
    }
    pub fn start_at_pending_adds(self) -> Skip<Self> {
        self.skip(todo!())
    }
    // Useful for skipping any pending adds that would be added after this call
    pub fn stop_at_current_len(self) -> Take<Self> {
        let end_index = self.container.len_including_pending_removals();
        self.take_within_current_bounds_via_end_index(end_index)
    }
    pub fn start_at_pending_adds_and_stop_at_current_len(self) -> Skip<Take<Self>> {
        self.stop_at_current_len().skip(todo!())
    }
    // Useful for getting only the items that were added after creation of this iterator
    pub fn start_at_current_len(self) -> Skip<Self> {
        self.skip(todo!())
    }
    pub fn only_pending_removals(self) -> Filter<Self, fn(&ItemAccessor<T>) -> bool> {
        self.filter(ItemAccessor::<T>::is_pending_removal)
    }
    /// Note that an item may become pending removal at any time. Checking that an item is not pending removal at this moment, doesn't prove that it will stay that way for the entirety of the current iteration's body.
    pub fn without_pending_removals(self) -> Filter<Self, fn(&ItemAccessor<T>) -> bool> {
        self.filter(is_not_pending_removal)
    }
}

fn is_not_pending_removal<T>(item: &ItemAccessor<T>) -> bool {
    !item.is_pending_removal()
}

// TODO: support ZSTs

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ItemRef<T> {
    phantom: PhantomData<T>,
}

impl<T> Deref for ItemRef<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl<T> ItemRef<T> {
    pub fn item_flags(&self) -> ItemFlags {
        todo!()
    }
    pub fn is_pending_add(&self) -> bool {
        todo!()
    }
    pub fn is_pending_removal(&self) -> bool {
        todo!()
    }
    pub fn is_settled(&self) -> bool {
        todo!()
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ItemRefMut<T> {
    phantom: PhantomData<T>,
}

impl<T> Deref for ItemRefMut<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl<T> DerefMut for ItemRefMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        todo!()
    }
}

impl<T> ItemRefMut<T> {
    pub fn item_flags(&self) -> ItemFlags {
        todo!()
    }
    pub fn is_pending_add(&self) -> bool {
        todo!()
    }
    pub fn is_pending_removal(&self) -> bool {
        todo!()
    }
    pub fn is_settled(&self) -> bool {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ItemFlags {
    Settled,
    PendingAdd,
    PendingRemoval,
    PendingAddAndRemoval,
}

impl ItemFlags {
    pub fn is_settled(&self) -> bool {
        self == &Self::Settled
    }
    pub fn is_pending_add(&self) -> bool {
        self == &Self::PendingAdd || self == &Self::PendingAddAndRemoval
    }
    pub fn is_pending_removal(&self) -> bool {
        self == &Self::PendingRemoval || self == &Self::PendingAddAndRemoval
    }
    pub fn is_pending(&self) -> bool {
        self == &Self::PendingAdd || self == &Self::PendingRemoval || self == &Self::PendingAddAndRemoval
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ItemAccessor<T> {
    phantom: PhantomData<T>,
}

impl<T> ItemAccessor<T> {
    pub fn item_flags(&self) -> ItemFlags {
        todo!()
    }
    pub fn is_pending_add(&self) -> bool {
        todo!()
    }
    pub fn is_pending_removal(&self) -> bool {
        todo!()
    }
    pub fn is_settled(&self) -> bool {
        todo!()
    }
    pub fn try_borrow(&self) -> Option<ItemRef<T>> {
        todo!() // Check this borrow counter + whole_container borrow counter
    }
    pub fn try_borrow_mut(&self) -> Option<ItemRefMut<T>> {
        todo!()
    }
    fn try_borrow_via_range_guard(&self, _range_guard: &RangeRef<T>) -> Option<ItemRef<T>> {
        todo!() // Check this borrow counter + skip ONE whole_container borrow counter
    }
    fn try_borrow_mut_via_range_guard(&self, _range_guard: &RangeRefMut<T>) -> Option<ItemRefMut<T>> {
        todo!()
    }
    pub fn borrow(&self) -> ItemRef<T> {
        self.try_borrow().unwrap()
    }
    pub fn borrow_mut(&self) -> ItemRefMut<T> {
        self.try_borrow_mut().unwrap()
    }
    fn borrow_via_range_guard(&self, range_guard: &RangeRef<T>) -> ItemRef<T> {
        self.try_borrow_via_range_guard(range_guard).unwrap()
    }
    fn borrow_mut_via_range_guard(&self, range_guard: &RangeRefMut<T>) -> ItemRefMut<T> {
        self.try_borrow_mut_via_range_guard(range_guard).unwrap()
    }
    /// # Safety
    /// 
    /// You must make sure that the item is not borrowed mutably, to respect Rust's strict aliasing rules. Failure to do so may result in Undefined Behavior.
    pub unsafe fn borrow_unchecked(&self) -> ItemRef<T> {
        #[cfg(feature = "trust_unchecked_borrows")]
        {
            todo!()
        }
        #[cfg(not(feature = "trust_unchecked_borrows"))]
        self.borrow()
    }
    /// # Safety
    /// 
    /// You must make sure that the item is not borrowed (mutably or not) by anyone else, to respect Rust's strict aliasing rules. Failure to do so may result in Undefined Behavior.
    pub unsafe fn borrow_mut_unchecked(&self) -> ItemRefMut<T> {
        #[cfg(feature = "trust_unchecked_borrows")]
        {
            todo!()
        }
        #[cfg(not(feature = "trust_unchecked_borrows"))]
        self.borrow_mut()
    }
    unsafe fn borrow_unchecked_via_range_guard(&self, range_guard: &RangeRef<T>) -> ItemRef<T> {
        #[cfg(feature = "trust_unchecked_borrows")]
        {
            todo!()
        }
        #[cfg(not(feature = "trust_unchecked_borrows"))]
        self.borrow_via_range_guard(range_guard)
    }
    unsafe fn borrow_mut_unchecked_via_range_guard(&self, range_guard: &RangeRefMut<T>) -> ItemRefMut<T> {
        #[cfg(feature = "trust_unchecked_borrows")]
        {
            todo!()
        }
        #[cfg(not(feature = "trust_unchecked_borrows"))]
        self.borrow_mut_via_range_guard(range_guard)
    }
    pub fn request<F: FnMut(ItemRef<T>)>(&self, mut f: F) -> bool {
        match self.try_borrow() {
            Some(r) => { f(r); true },
            None => todo!(), // Enqueue command in the container
        }
    }
    pub fn request_mut<F: FnMut(ItemRefMut<T>)>(&self, mut f: F) -> bool {
        match self.try_borrow_mut() {
            Some(r) => { f(r); true },
            None => todo!(), // Enqueue command in the container
        }
    }
}

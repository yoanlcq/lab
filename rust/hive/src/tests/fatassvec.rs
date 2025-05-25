#![allow(dead_code, unused_variables, unreachable_code, unused_mut)]

use std::{marker::PhantomData, ops::{ControlFlow, Deref, DerefMut}};

#[derive(Debug)]
struct Fatassvec<T> {
    phantom: PhantomData<T>,
}

impl<T> Default for Fatassvec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Fatassvec<T> {
    pub fn new() -> Self {
        unimplemented!()
    }
    pub fn len(&self, filter: Filter) -> usize {
        unimplemented!()
    }
    pub fn is_empty(&self, filter: Filter) -> bool {
        self.len(filter) == 0
    }
    // - If the container is borrowed: this will conceptually enqueue a command to add the new item.
    //   Iterators may be configured to also evaluate items that are pending add, but the caller should note that such items do not respect the container's ordering requirement.
    //   When that command is executed, the item will be added in a way that respects the container's ordering requirement.
    //   Internally, sorting the container might happen in a "lazy" fashion, to ensure that consecutives calls to add() are fast.
    // - If the container is not borrowed, the behavior is as if the command was enqueued then executed immediately.
    pub fn add(&self, val: T) {
        unimplemented!()
    }
    pub fn add_and_get(&self, val: T) -> ItemAccessor<T> {
        unimplemented!()
    }
    // Same as add() but possibly faster thanks to the compile-time guarantee that nobody else is borrowing the container.
    pub fn add_mut(&mut self, val: T) -> &mut T {
        unimplemented!()
    }
    // - If the container is borrowed: this will conceptually enqueue a command to remove the new item.
    //   Iterators may be configured to skip items that are pending removal.
    //   When that command is executed, the item will be removed in a way that respects the container's ordering requirement.
    // - If the container is not borrowed, the behavior is as if the command was enqueued then executed immediately.
    pub fn remove(&self, index: usize) -> Option<T> {
        unimplemented!()
    }
    // Enqueues a command to remove the item and apply a function on the item after it is removed.
    pub fn remove_and<F: FnMut(T)>(&self, index: usize, mut f: F) -> bool {
        unimplemented!()
    }
    // Same as remove() but possibly faster thanks to the compile-time guarantee that nobody else is borrowing the container.
    pub fn remove_mut(&mut self, index: usize) -> T {
        unimplemented!()
    }
    pub fn try_remove_mut(&mut self, index: usize) -> Option<T> {
        unimplemented!()
    }
    // TODO: set ordering requirement:
    // - None (no ordering whatsoever. Adds to the end. Removes via remove_swap)
    // - Sequential (Adds to the end. Removes via remove_shift)
    // - Custom (user-specified comparison predicate)

    // TODO: iterator params:
    // - Include pending add items? (index < nb_committed)
    // - Include pending removal items? (is_pending_removal[i])
    // - (Requires ordering?) <= Cannot be implemented in a shared way, because having a live unordered iterator prevents creating an ordered iterator
    // - Reverse iterator: iterates through committed items first (respecting the reverse order), THEN all pending adds in order (should be done last because the 1st iterations may cause pending adds)
    
    // TODO: iterator provides:
    // - Is the item pending add? (self.include_pending_adds && index < nb_committed)
    // - Is the item pending removal? (self.include_pending_removals && is_pending_removal[i])
    // - RefCell-like API to access the item

    // Items are sorted and never pending
    pub fn into_iter(self) -> ValueIterator<T> {
        unimplemented!()
    }
    // Items are sorted and never pending
    pub fn iter_mut(&mut self) -> ExclusiveIterator<T> {
        unimplemented!()
    }
    pub fn iter(&self, p: IteratorParams) -> SharedIterator<T> {
        unimplemented!()
    }

    pub fn can_borrow_range(&self, filter: RangeFilter) -> bool {
        unimplemented!()
    }
    pub fn can_borrow_range_mut(&self, filter: RangeFilter) -> bool {
        unimplemented!()
    }

    fn try_borrow_range(&self, filter: RangeFilter) -> Option<RangeRef<T>> {
        if !self.can_borrow_range(filter) {
            return None;
        }
        unimplemented!()
    }

    fn try_borrow_range_mut(&self, filter: RangeFilter) -> Option<RangeRefMut<T>> {
        if !self.can_borrow_range_mut(filter) {
            return None;
        }
        // TODO: should flush all pending commands, then sort immediately, because this is proof of an imminent access
        unimplemented!()
    }

    // This is more efficient than iterating on each item and calling try_borrow() or request_with_ref(), by avoiding these branches; in exchange, your operation might not execute immediately.
    // Enqueues the command and executes it as soon as the container has no live mutable borrow (i.e when calling borrow() on all items at once would not panic)
    // During execution of the command, all items are borrowed simultaneously as a single unit.
    pub fn request_for_each<F: FnMut(ItemRef<T>)>(&self, p: IteratorParams, mut f: F) -> bool {
        if let Some(_range_guard) = self.try_borrow_range(p.filter.into()) {
            for it in self.iter(p) {
                f(unsafe { it.borrow_unchecked() });
            }
            true
        } else {
            unimplemented!() // Enqueue the command
        }
    }
    pub fn request_for_each_with_control_flow<R, F: FnMut(ItemRef<T>) -> ControlFlow<R>>(&self, p: IteratorParams, mut f: F) -> Option<R> { // TODO: return Result instead to distinguish cases
        if let Some(_range_guard) = self.try_borrow_range(p.filter.into()) {
            for it in self.iter(p) {
                if let ControlFlow::Break(r) = f(unsafe { it.borrow_unchecked() }) {
                    return Some(r);
                }
            }
            None
        } else {
            unimplemented!() // Enqueue the command
        }
    }
    // This is more efficient than iterating on each item and calling try_borrow_mut() or request_with_mut(), by avoiding these branches; in exchange, your operation might not execute immediately.
    // Enqueues the command and executes it as soon as the container has no live borrow (i.e when calling borrow_mut() on all items at once would not panic)
    // During execution of the command, all items are borrowed mutably simultaneously as a single unit.
    //
    // When iteration starts, we guarantee there are no pending operations, and the array is sorted.
    // However, this doesn't prevent new commands from being issued during iteration.
    pub fn request_for_each_mut<F: FnMut(ItemRefMut<T>)>(&self, p: IteratorParams, mut f: F) -> bool {
        if let Some(_range_guard) = self.try_borrow_range_mut(p.filter.into()) {
            // TODO: We should store the valid range OR make sure we don't construct ItemBorrowMut() on items that were not there yet when we borrowed the whole container
            // QUESTION: due to its nature, can it support iterating on pending adds?? Maybe yes actually? That would allow a counterintuitive behavior where you add an item and you can't immediately borrow it mutably. Maybe this should be a param or this is already just include_pending_adds.
            // There could be a policy "include_pending_adds + try borrow but just for those"
            // So the possibilities are:
            // - Don't include_pending_adds (borrow all committed items)
            // - include_pending_adds, count any pending add as borrowed by the whole container (so the one who calls add() won't be able to borrow what was just added, which may be weird)
            // - include_pending_adds, but pending adds give an ItemAccessor instead of ItemRefMut. Efficient iteration on committed items, and controlled operation on pending adds.
            // NOTE that this question also applies to request_for_each_ref actually
            for it in self.iter(p) {
                f(unsafe { it.borrow_mut_unchecked() });
            }
            true
        } else {
            unimplemented!() // Enqueue the command
        }
    }
    pub fn request_for_each_mut_with_control_flow<R, F: FnMut(ItemRefMut<T>) -> ControlFlow<R>>(&self, p: IteratorParams, mut f: F) -> Option<R> { // TODO: return Result instead to distinguish cases
        if let Some(_range_guard) = self.try_borrow_range_mut(p.filter.into()) {
            for it in self.iter(p) {
                if let ControlFlow::Break(r) = f(unsafe { it.borrow_mut_unchecked() }) {
                    return Some(r);
                }
            }
            None
        } else {
            unimplemented!() // Enqueue the command
        }
    }
}

struct RangeRef<'a, T> {
    container: &'a Fatassvec<T>,
}

struct RangeRefMut<'a, T> {
    container: &'a Fatassvec<T>,
}

impl<'a, T> Drop for RangeRef<'a, T> {
    fn drop(&mut self) {
        unimplemented!()
    }
}

impl<'a, T> Drop for RangeRefMut<'a, T> {
    fn drop(&mut self) {
        unimplemented!()
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RangeFilter {
    include_committed_items: bool,
    include_pending_adds: bool,
}

impl From<Filter> for RangeFilter {
    fn from(value: Filter) -> Self {
        Self {
            include_committed_items: value.include_committed_items,
            include_pending_adds: value.include_pending_adds,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Filter {
    include_committed_items: bool,
    include_pending_adds: bool,
    include_pending_removals: bool,
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            include_committed_items: true,
            include_pending_adds: false,
            include_pending_removals: false,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct IteratorParams {
    filter: Filter,
    requires_respected_ordering: bool,
    reverse: bool,
}

pub struct ValueIterator<T> {
    phantom: PhantomData<T>,
}

pub struct ExclusiveIterator<'a, T> {
    phantom: PhantomData<&'a mut T>,
}

pub struct SharedIterator<T> {
    phantom: PhantomData<T>,
}

impl<T> Iterator for ValueIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

impl<'a, T> Iterator for ExclusiveIterator<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

impl<T> Iterator for SharedIterator<T> {
    type Item = ItemAccessor<T>;
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

pub struct ItemRef<T> {
    phantom: PhantomData<T>,
}

impl<T> Deref for ItemRef<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unimplemented!()
    }
}

impl<T> ItemRef<T> {
    pub fn item_flags(&self) -> ItemFlags {
        unimplemented!()
    }
    pub fn is_pending_add(&self) -> bool {
        unimplemented!()
    }
    pub fn is_pending_removal(&self) -> bool {
        unimplemented!()
    }
    pub fn is_committed(&self) -> bool {
        unimplemented!()
    }
}

pub struct ItemRefMut<T> {
    phantom: PhantomData<T>,
}

impl<T> Deref for ItemRefMut<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unimplemented!()
    }
}

impl<T> DerefMut for ItemRefMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unimplemented!()
    }
}

impl<T> ItemRefMut<T> {
    pub fn item_flags(&self) -> ItemFlags {
        unimplemented!()
    }
    pub fn is_pending_add(&self) -> bool {
        unimplemented!()
    }
    pub fn is_pending_removal(&self) -> bool {
        unimplemented!()
    }
    pub fn is_committed(&self) -> bool {
        unimplemented!()
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ItemFlags {
    Committed,
    PendingAdd,
    PendingRemoval,
    PendingAddAndRemoval,
}

impl ItemFlags {
    pub fn is_pending_add(&self) -> bool {
        self == &Self::PendingAdd || self == &Self::PendingAddAndRemoval
    }
    pub fn is_pending_removal(&self) -> bool {
        self == &Self::PendingRemoval || self == &Self::PendingAddAndRemoval
    }
}

pub struct ItemAccessor<T> {
    phantom: PhantomData<T>,
}

impl<T> ItemAccessor<T> {
    pub fn item_flags(&self) -> ItemFlags {
        unimplemented!()
    }
    pub fn is_pending_add(&self) -> bool {
        unimplemented!()
    }
    pub fn is_pending_removal(&self) -> bool {
        unimplemented!()
    }
    pub fn is_committed(&self) -> bool {
        unimplemented!()
    }
    pub fn borrow(&self) -> ItemRef<T> {
        self.try_borrow().unwrap()
    }
    pub fn borrow_mut(&self) -> ItemRefMut<T> {
        self.try_borrow_mut().unwrap()
    }
    pub unsafe fn borrow_unchecked(&self) -> ItemRef<T> {
        #[cfg(feature = "go_safe")]
        {
            unimplemented!()
        }
        #[cfg(not(feature = "go_safe"))]
        self.borrow()
    }
    pub unsafe fn borrow_mut_unchecked(&self) -> ItemRefMut<T> {
        #[cfg(feature = "go_safe")]
        {
            unimplemented!()
        }
        #[cfg(not(feature = "go_safe"))]
        self.borrow_mut()
    }
    pub fn try_borrow(&self) -> Option<ItemRef<T>> {
        unimplemented!() // Check this borrow counter + whole_container borrow counter
    }
    pub fn try_borrow_mut(&self) -> Option<ItemRefMut<T>> {
        unimplemented!()
    }
    pub fn request<F: FnMut(ItemRef<T>)>(&self, mut f: F) -> bool {
        match self.try_borrow() {
            Some(r) => { f(r); true },
            None => unimplemented!(), // Enqueue command in the container
        }
    }
    pub fn request_mut<F: FnMut(ItemRefMut<T>)>(&self, mut f: F) -> bool {
        match self.try_borrow_mut() {
            Some(r) => { f(r); true },
            None => unimplemented!(), // Enqueue command in the container
        }
    }
}

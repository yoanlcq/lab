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
    // TODO: But modifying items via mutable references may invalide ordering. How do we deal with this? Mark such items as dirty? automatically or manually? How about items modified via shared references with cells?
    // The user should be given a way to mark items are dirty for ordering. However it's still possible that they forget to do it.
    // TODO: Maintain a list of "ranges" to quickly access a range of items that satisfy a subset of the ordering predicate (and also remember that there may be pending adds, and those are not sorted, so the caller may have to iterate on them and check the predicate manually)

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
    pub fn request_for_each_with_pending_adds_op<F: FnMut(ItemRef<T>), F2: FnMut(ItemAccessor<T>)>(&self, p: IteratorParams, mut f: F, mut f2: F2) -> bool {
        if let Some(_range_guard) = self.try_borrow_range(p.filter.into()) {
            for it in self.iter(p) {
                f(unsafe { it.borrow_unchecked() });
            }
            self.internal_visit_pending_adds(p, f2);
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
    pub fn request_for_each_with_pending_adds_op_and_control_flow<R, F: FnMut(ItemRef<T>) -> ControlFlow<R>, F2: FnMut(ItemAccessor<T>) -> ControlFlow<R>>(&self, p: IteratorParams, mut f: F, mut f2: F2) -> Option<R> { // TODO: return Result instead to distinguish cases
        if let Some(_range_guard) = self.try_borrow_range(p.filter.into()) {
            for it in self.iter(p) {
                if let ControlFlow::Break(r) = f(unsafe { it.borrow_unchecked() }) {
                    return Some(r);
                }
            }
            self.internal_visit_pending_adds_with_control_flow(p, f2)
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
            for it in self.iter(p) {
                f(unsafe { it.borrow_mut_unchecked() });
            }
            true
        } else {
            unimplemented!() // Enqueue the command
        }
    }
    pub fn request_for_each_mut_with_pending_adds_op<F: FnMut(ItemRefMut<T>), F2: FnMut(ItemAccessor<T>)>(&self, p: IteratorParams, mut f: F, mut f2: F2) -> bool {
        if let Some(_range_guard) = self.try_borrow_range_mut(p.filter.into()) {
            for it in self.iter(p) {
                f(unsafe { it.borrow_mut_unchecked() });
            }
            self.internal_visit_pending_adds(p, f2);
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
    pub fn request_for_each_mut_with_pending_adds_op_and_control_flow<R, F: FnMut(ItemRefMut<T>) -> ControlFlow<R>, F2: FnMut(ItemAccessor<T>) -> ControlFlow<R>>(&self, p: IteratorParams, mut f: F, mut f2: F2) -> Option<R> { // TODO: return Result instead to distinguish cases
        if let Some(_range_guard) = self.try_borrow_range_mut(p.filter.into()) {
            for it in self.iter(p) {
                if let ControlFlow::Break(r) = f(unsafe { it.borrow_mut_unchecked() }) {
                    return Some(r);
                }
            }
            self.internal_visit_pending_adds_with_control_flow(p, f2)
        } else {
            unimplemented!() // Enqueue the command
        }
    }
    fn internal_visit_pending_adds_params(p: IteratorParams) -> Option<IteratorParams> {
        if !p.filter.include_pending_adds {
            let mut new_p = p;
            new_p.filter.include_pending_adds = true;
            new_p.filter.include_committed_items = false;
            Some(new_p)
        } else {
            None
        }
    }
    fn internal_visit_pending_adds<F: FnMut(ItemAccessor<T>)>(&self, p: IteratorParams, mut f: F) {
        if let Some(p) = Self::internal_visit_pending_adds_params(p) {
            for it in self.iter(p) {
                f(it);
            }
        }
    }
    fn internal_visit_pending_adds_with_control_flow<R, F: FnMut(ItemAccessor<T>) -> ControlFlow<R>>(&self, p: IteratorParams, mut f: F) -> Option<R> {
        if let Some(p) = Self::internal_visit_pending_adds_params(p) {
            for it in self.iter(p) {
                if let ControlFlow::Break(r) = f(it) {
                    return Some(r);
                }
            }
        }
        None
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

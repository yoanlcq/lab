#![feature(fnbox)]

use std::collections::VecDeque;
use std::sync::Arc;
use std::any::Any;
use std::boxed::FnBox;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::sync::atomic::{*};

//
//
// ENGINE SIDE
//
//

/*
/// Convenience for creating simple jobs based on a function that accepts anything (via the closure's
/// capture) and returns anything (including the unit `()` type).
///
/// This can indeed be used to turn any synchronous computation into an asynchronous one, with the
/// progress value being just a boolean: done, or not done.
///
/// If your needs are more complex, just create you own type that implements `Task` instead.
pub struct Async<T> {
    f: RefCell<Option<Box<FnBox() -> T>>>,
    result: RefCell<Option<T>>,
}

impl<T> Async<T> {
    pub fn new<F>(f: F) -> Self where F: FnBox() -> T + 'static {
        Self { f: RefCell::new(Some(Box::new(f))), result: RefCell::new(None), }
    }
}

impl<T> Task for Async<T> {
    type Progress = bool;
    type Result = T;
    fn make_progress(&self) {
        let f = self.f.borrow_mut().take().unwrap();
        *self.result.borrow_mut() = Some(f());
    }
    fn is_complete(&self) -> bool {
        self.f.borrow().is_none()
    }
    fn progress(&self) -> bool { 
        self.f.borrow().is_none()
    }
    fn result(&self) -> T {
        self.result.borrow_mut().take().unwrap()
    }
}

/// A combinator for chaining two tasks sequentially.
pub struct Then<T: Task, E: Task> {
    active: RefCell<Result<T, E>>,
    f: RefCell<Option<Box<FnBox(T::Result) -> E>>>,
}

impl<T: Task, E: Task> Then<T, E> {
    pub fn new<F>(t: T, f: F) -> Self where T: Task, F: FnBox(T::Result) -> E + 'static {
        Self {
            active: RefCell::new(Ok(t)),
            f: RefCell::new(Some(Box::new(f))),
        }
    }
}

impl<T: Task, E: Task> Task for Then<T, E> {
    type Progress = Result<T::Progress, E::Progress>;
    type Result = E::Result;
    fn make_progress(&self) {
        let mut active = self.active.borrow_mut();
        let first_result = match *active {
            Ok(ref t) => { t.make_progress(); if t.is_complete() { Some(t.result()) } else { None } },
            Err(ref t) => { t.make_progress(); None },
        };
        if let Some(r) = first_result {
            let f = self.f.borrow_mut().take().unwrap();
            *active = Err((f)(r));
        }
    }
    fn is_complete(&self) -> bool {
        match *self.active.borrow() {
            Ok(_) => false,
            Err(ref t) => t.is_complete(),
        }
    }
    fn progress(&self) -> Self::Progress {
        match *self.active.borrow() {
            Ok(ref t) => Ok(t.progress()),
            Err(ref t) => Err(t.progress()),
        }
    }
    fn result(&self) -> Self::Result {
        match *self.active.borrow() {
            Ok(_) => panic!(), // Not done yet!
            Err(ref t) => t.result(),
        }
    }
}

pub trait TaskExt: Task {
    /// Returns a Task which result is the one of the last task, which requires the
    /// completion of the first task.
    /// This effectively "merges" two tasks into one.
    fn then<T, F>(self, f: F) -> Then<Self, T> where Self: Sized, T: Task, F: FnBox(Self::Result) -> T + 'static {
        Then::new(self, f)
    }
    /// Returns a Task which result is the first returned by any of two tasks.
    /// This effectively "merges" two tasks into one.
    fn select<T>(self, t: T) -> Select<Self, T> where Self: Sized {
        Select::new(self, t)
    }
    /// Returns a Task which result is both results of two tasks.
    /// This effectively "merges" two tasks into one.
    fn join<T>(self, t: T) -> Join<Self, T> where Self: Sized {
        Join::new(self, t)
    }
}

impl<T: Task + ?Sized> TaskExt for T {}

pub struct Select<T, E>(T, E);
pub struct Join<T, E>(T, E);

// TODO: Implement Task!
impl<T, E> Select<T, E> {
    pub fn new(t: T, e: E) -> Self {
        Select(t, e)
    }
}

// TODO: Implement Task!
impl<T, E> Join<T, E> {
    pub fn new(t: T, e: E) -> Self {
        Join(t, e)
    }
}
*/

pub trait Progress {
    fn is_complete(&self) -> bool;
}

impl Progress for bool {
    fn is_complete(&self) -> bool { *self }
}

// Pipelining:
// Create two tasks A and B which are each given an Arc to shared state they agree on.
// A::resume() "produces" content into that shared state, and B::resume() "consumes" content if
// any.
// The value of A::progress() and B::progress() can be decided by reading the shared state.

// The concrete trait users have to implement for creating new kinds of tasks.
pub trait Task {
    type Progress: Progress;
    type Result;
    /// Asks this task to progress "a bit", where "a bit" depends on how the task is configured.
    /// This is expected to be somewhat expensive (perform a long-running computation or perform actual I/O...),
    /// but not too much.
    ///
    /// In essence, `resume()` partially completes the task, then _yields_ execution to the calling
    /// thread simply by returning.
    ///
    /// This doesn't take `&mut self` so as to avoid having to wrap the whole Task in a RefCell or Mutex.
    /// Instead, the task has to selectively use interior mutability for relevant state.
    fn resume(&self);
    /// Gets a specialized description of the current "progress" state of the task.
    ///
    /// This may be as simple as a `bool` but also contain extra information that could be useful
    /// for displaying.
    fn progress(&self) -> Self::Progress;
    /// Gets the task's result. Semantically, this _consumes_ the task, which should then be
    /// dropped.
    ///
    /// The following invariants must be held by the caller (otherwise the
    /// implementation is free to panic):
    /// - is_complete() is true;
    /// - This method is only ever called once, because semantically, the result is moved out of
    ///   this object. Unfortunately, this method cannot take `self` to enforce this, because
    ///   otherwise it could not be made into a trait object.
    ///
    /// These invariants are normally enforced at compile-time by the higher-level APIs.
    fn result(&self) -> Self::Result;
}

pub struct Future<T>(Arc<Box<AnyTask>>, PhantomData<T>);

impl<T: Task> Future<T> {
    pub fn poll(&self) -> T::Progress where T::Progress: 'static {
        *self.0.progress().downcast().unwrap()
    }
    pub fn wait(self) -> T::Result where T::Result: 'static {
        // TODO: Perform the actual wait, but how?
        // - Steal the work ??
        // - Block until the other thread is done ???
        // - Poll repeatedly ?? (but how would we know when the progress is "complete"? (hint: Add is_complete() in Task trait ??))
        *self.0.result().downcast().unwrap()
    }
    pub fn cancel(self) {}
    pub fn inner(&self) -> &AnyTask {
        &**self.0
    }
}





pub trait AnyTask {
    fn make_progress(&self);
    fn is_complete(&self) -> bool;
    fn scheduler_hints(&self) -> SchedulerHints;
    fn progress(&self) -> Box<Any>;
    fn result(&self) -> Box<Any>;
    fn run_to_completion(&self) -> Box<Any>;
}

impl<T: Task> AnyTask for T
    where T::Progress: 'static,
          T::Result: 'static,
{
    fn make_progress(&self)        { self.make_progress() }
    fn is_complete(&self) -> bool  { self.is_complete() }
    fn scheduler_hints(&self) -> SchedulerHints { self.scheduler_hints() }
    fn progress(&self) -> Box<Any> { Box::new(self.progress()) }
    fn result(&self) -> Box<Any>   { Box::new(self.result()) }
    fn run_to_completion(&self) -> Box<Any>   { Box::new(self.run_to_completion()) }
}

#[derive(Default)]
struct G {
    pub q: VecDeque<Arc<Box<AnyTask>>>,
}

impl G {
    pub fn schedule<T: Task + 'static>(&mut self, t: T) -> Future<T> {
        let t = Arc::new(Box::new(t) as Box<AnyTask>);
        self.q.push_back(t.clone());
        Future(t, PhantomData)
    }
    pub fn set_priority(&mut self, )
}



//
//
// GAME SIDE
//
//


#[derive(Debug)]
pub struct LoadingFileProgress {
    pub thread_id: isize,
    pub nb_bytes_read: usize,
    pub nb_bytes_total: usize,
}

pub struct LoadingFile {
    // Constants
    pub path: String,
    pub chunk_size: usize,
    // Progress
    pub thread_id: AtomicIsize,
    pub nb_bytes_read: AtomicUsize,
    pub nb_bytes_total: AtomicUsize,
    // Result
    pub data: RefCell<Vec<u8>>,
}

impl LoadingFile {
    pub fn new(path: String, chunk_size: usize) -> Self {
        Self {
            path,
            chunk_size,
            thread_id: AtomicIsize::new(-1),
            nb_bytes_read: AtomicUsize::new(0),
            nb_bytes_total: AtomicUsize::new(0),
            data: RefCell::new(vec![]),
        }
    }
}

impl Task for LoadingFile {
    type Result = Result<Vec<u8>, String>;
    type Progress = LoadingFileProgress;
    fn make_progress(&self) { }
    fn is_complete(&self) -> bool { unimplemented!{} }
    fn progress(&self) -> Self::Progress {
        LoadingFileProgress {
            thread_id: self.thread_id.load(Ordering::SeqCst),
            nb_bytes_total: self.nb_bytes_total.load(Ordering::SeqCst),
            nb_bytes_read: self.nb_bytes_read.load(Ordering::SeqCst),
        }
    }
    fn result(&self) -> Self::Result { Ok(::std::mem::replace(&mut self.data.borrow_mut(), vec![])) }
}

fn main() {
    let mut g = G::default();
    let val = g.schedule(LoadingFile::new("foo".to_owned(), 512)).wait().unwrap();
    println!("{:?}", val);
}

/*
#![feature(fnbox)]

use std::any::Any;
use std::mem;
use std::boxed::FnBox;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub type BoxFnResult = Box<Any>;
pub type BoxFn = Box<FnBox() -> BoxFnResult + 'static>;

#[derive(Default)]
struct G {
    pub q: VecDeque<Arc<Task>>,
}

struct Task {
    pub f: BoxFn,
}

impl std::fmt::Debug for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unimplemented!()
    }
}

impl Task {
    pub fn new(f: BoxFn) -> Self {
        Self {
            f,
        }
    }
}

struct Async(Arc<Task>);

impl Async {
    pub fn wait(self) -> BoxFnResult {
        (Arc::try_unwrap(self.0).unwrap().f)()
    }
}

impl G {
    pub fn do_async(&mut self, f: BoxFn) -> Async {
        let t = Arc::new(Task::new(f));
        //self.q.push_back(t.clone());
        Async(t)
    }
}

fn main() {
    let mut g = G::default();
    let val = g.do_async(Box::new(|| Box::new(2) as Box<Any>)).wait();
    println!("{}", val.downcast::<i32>().unwrap());
}
*/

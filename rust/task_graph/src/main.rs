// Possible improvements:
// - Currently, tasks are stored in a queue. It should be a graph instead so
//   that tasks can express dependencies.
//   Cycles cannot be handled, because if A depends on B and vice versa, 
//   none of them can be consumed.
//   But we intuitively want cycles in the task graph because the game
//   is basically an infinite loop (and ideally we would like to have any
//   number of sub-loops as needed).
//   The solution is to add some extra info.
//   For instance, A(N+1) depends on B(N), and B(N) depends on A(N), where
//   N is the tick/frame/iteration number.
//   N=0: A(0) depends on B(-1), and B(0) depends on A(0).
//        B(-1) is not considered (-1 is invalid), so we run A, then B.
//   N=1: A(1) depends on B(0), and B(1) depends on A(1).
//        B(0) was run in the previous step, so now we run A, then B.
//   N=2: A(2) depends on B(1), and B(2) depends on A(2).
//        B(1) was run in the previous step, so now we run A, then B.
//   etc....
// - Tasks are popped from the queue. In an actual game, tasks would instead
//   be marked as "done for current iteration" and remain in the queue, so
//   they can be processed again in the next iteration, and so forth.
// - It should be possible to set thread affinity to specific CPU cores
//   (but it's not mandatory; few people actually do this because the OS
//   supposedly does the right thing most of the time ??).
//   A reason would be to leverage shared caches based on our knowledge
//   of data locality.
// - Some tasks should run only on specific threads.
//   For instance, an OpenGL task, or window event processing task, should
//   only be allowed to happen while in the main thread.
//   See the TaskFlags struct.
// - Have a proper scheduler, which would require:
//   - Knowing the amount of physical CPU cores (and logical ones);
//   - Knowing the access patterns of tasks (read ? read-write ? on which data sets ?)
//   - Finding the critical path in the task graph;
//     This involves profiling tasks and using this knowledge to change
//     scheduling strategies dynamically.
//   - Tweaking responsibilities of threads dynamically so that they can
//     help in other domains;
// - Catch and recover from panics;
// - Task groups:
//   Solved by having dummy head and tail tasks.
// - Data parallel tasks:
//   Solved by spawning as many tasks as chunks of data we want to process.
//   However we might want to leverage slice::chunks() instead of
//   locking with an RwLock.
// - We might not need locks at all for data if we're careful about how
//   we set up the task graph.
// - Export graph to Graphviz ?
// - Cross-frame/cross-tick calculations
//   e.g a spatial query that would take 3 ticks to complete using 1 thread.
//   How would we go about that ?
//   Solution 1: Have a sub-graph that loops on itself until the task is
//   done (most cooperation with other tasks).
//   Solution 2: Have a single huge task for that, that no other task depends upon.
//   Hoping that other threads can cope with the increased charge in the
//   meantime.
//   threads are able to cope with other tasks
// - How to make tasks cancellable ?
// - Work Stealing ?

#[macro_use]
extern crate bitflags;

use std::thread;
use std::sync::{RwLock, Arc};
use std::collections::VecDeque;

mod experimental {
    bitflags! {
        /// This bitfield is used as a set for each thread and task.
        /// A thread is allowed to consume a task, if and only if their
        /// sets intersect.
        ///
        /// `MISC` and `MAIN` are the only two "standard" flags.
        /// You may add or remove others depending on your use cases.
        pub struct TaskFlags: u32 {
            /// This flag exists as the single catch-all for most
            /// tasks and some threads.
            ///
            /// When a thread has no raised flag, it's not allowed
            /// to consume any task at all. Likewise, when a task has no
            /// raised flag, no thread is allowed to consume it.
            ///
            /// However, some tasks are too general-purpose to fit in
            /// other categories, and some threads have no specific role
            /// either.
            /// In order for these tasks and threads to be useful at all,
            /// they should at a minimum raise this flag.
            const MISC = 0b00000001;
            /// The only thread that can (and must) have the `MAIN`
            /// flag set is, obviously, the main thread.
            /// Tasks that set this bit will therefore always be executed
            /// in the main thread, which is required by some APIs such as
            /// OpenGL and window event pumps.
            const MAIN = 0b000000010;

            // NOTE: "engine" flags start from 8th bit right now.

            /// Flag for persistent storage I/O.
            const FILE_IO = 0b100000000;
            /// Flag for network I/O.
            const NETWORK_IO = 0b1000000000;
            /// Union of all I/O flags.
            const IO = Self::FILE_IO.bits | Self::NETWORK_IO.bits;

            // NOTE: "game" flags start from 16th bit right now.
            // - GFX would fit in MAIN as long as we're using OpenGL;
            // - GAME_LOGIC would fit either in PHYSICS or in MISC.

            /// Flag for audio I/O and DSP.
            const AUDIO = 0b10000000000000000;
            /// Flag for physics calculations.
            const PHYSICS = 0b100000000000000000;
            /// Flag for AI and pathfinding.
            const AI = 0b1000000000000000000;
            /// Flag for on-CPU skeletal animation and blending.
            const ANIM = 0b10000000000000000000;
        }
    }
    // The default value for `TaskFlags` is `MISC | MAIN`, which
    // is always a safe default.
    impl Default for TaskFlags {
        fn default() -> Self {
            Self::MISC | Self::MAIN
        }
    }
}

#[derive(Default)]
struct Tasks {
    pub queue: VecDeque<fn(&TaskContext)>,
}

struct TaskContext<'a> {
    pub i: u32,
    pub task_i: usize,
    pub g: &'a Global,
}

#[derive(Default)]
struct Global {
    pub tasks: RwLock<Tasks>,
    pub threads: RwLock<Vec<thread::JoinHandle<()>>>,
    pub game: Game,
}

#[derive(Default)]
struct Game {
    pub score: RwLock<u32>,
}

impl Tasks {
    pub fn dumb_task_fn(ctx: &TaskContext) {
        let &TaskContext { i, task_i, ref g } = ctx;
        println!("Thread {}: Task {}: Started", i, task_i);
        {
            // Make sure to print while having the lock
            let score_lock = g.game.score.read().unwrap();
            println!("Thread {}: Task {}: Score = {}", i, task_i, *score_lock);
        }
        println!("Thread {}: Task {}: Done", i, task_i);
    }
    pub fn new() -> Self {
        let queue = (0..5).map(|_| Self::dumb_task_fn as _).collect();
        Self { queue, }
    }
}

impl Global {
    pub fn new() -> Self {
        Self {
            tasks: RwLock::new(Tasks::new()),
            .. Global::default()
        }
    }
    pub fn process_tasks(&self, i: u32) {
        loop {
            let mut tasks = self.tasks.write().unwrap();
            match tasks.queue.pop_front() {
                None => break,
                Some(f) => {
                    let task_i = tasks.queue.len();
                    (f)(&TaskContext { i, task_i, g: self, })
                },
            }
        }
        println!("Thread {}: No tasks left to process", i);
    }
    pub fn thread_inc_score(&self, i: u32) {
        // Make sure to print while having the lock
        let mut score_lock = self.game.score.write().unwrap();
        println!("Thread {}: Score = {} (was {}).", i, *score_lock + i, *score_lock);
        *score_lock += i;
    }
    pub fn thread_proc(&self, i: u32) {
        println!("Thread {}: Started.", i);
        self.thread_inc_score(i);
        self.process_tasks(i);
        println!("Thread {}: Done", i);
    }
}

fn main() {
    let g = Arc::new(Global::new());
    for i in 1..6 {
        g.threads.write().unwrap().push({
            let g = g.clone();
            println!("Main: Thread {}: Spawning", i);
            thread::Builder::new()
                .name(format!("Thread {}", i))
                .spawn(move || g.thread_proc(i))
                .unwrap()
        });
    }
    g.process_tasks(0);

    let mut t = g.threads.write().unwrap();
    for (i, t) in t.drain(..).enumerate() {
        t.join().unwrap();
        println!("Main: Thread {}: Joined", i+1);
    }
    println!("Main: Score = {}", *g.game.score.read().unwrap());
}

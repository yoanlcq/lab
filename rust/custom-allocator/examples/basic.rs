use std::sync::atomic::Ordering;

extern crate custom_allocator as ca;
use ca::TOTAL_USED_MEM;

fn main() {
    let v: Vec<_> = (0..).into_iter().take(8000).collect();
    let used_mem = TOTAL_USED_MEM.load(Ordering::SeqCst);
    println!("{:?}", v);
    println!("Total bytes allocated via custom allocator: {}", used_mem);
}

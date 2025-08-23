extern crate alloc;

use alloc::borrow::Cow;
use core::any::Any;
use linkme::distributed_slice;

pub use linkme;

pub enum TunableValue<'a> {
    Any(Cow<'a, &'a (dyn Any + Sync)>),
    String(Cow<'a, &'a str>),
    Bool(bool),
    I32(i32),
    U32(u32),
    F32(f32),
    I64(i64),
    U64(u64),
    F64(f64),
}

pub struct Tunable<'a> {
    pub name: &'a str,
    pub help: &'a str,
    pub default: TunableValue<'a>,
}

#[distributed_slice]
pub static TUNABLES: [Tunable];
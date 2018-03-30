// ![feature(custom_attribute)]
#![feature(repr_simd)]

use std::sync::Mutex;

lazy_static! {
    static ref TYTYS : Mutex<Vec<&'static str>> = Mutex::new(Vec::new());
}

pub fn tyty_add(name : &'static str) -> u32 {
    let mut tytys = TYTYS.lock().unwrap();
    tytys.push(name);
    0
}


#[macro_use]
extern crate foo_derive;
extern crate foo;
use foo::ZeroedMem;
#[macro_use]
extern crate lazy_static;


#[derive(Default, Debug, Copy, Clone, PartialEq)]
#[repr(packed,simd)]
struct Xvec3(i64,i64,i64);
#[derive(Default, Debug, Copy, Clone, PartialEq)]
#[repr(packed,simd)]
struct Xquat(f32,f32,f32,f32);

#[derive(Foo, Inspect, ZeroedMem, Default, Debug, Copy, Clone, PartialEq)]
#[tie(position, scale)]
#[hint(orientation, scale)]
#[repr(C)]
struct Xform {
    position : Xvec3,
    orientation : Xquat,
    scale : Xvec3
}

fn main() {
    let foo = Xform::default();
    let foo = Xform::zeroed_mem();
    println!("Hello, world! {:?}", foo);
    Xform::foo();
    let foo = &FOO_Xform;
    for t in TYTYS.lock().unwrap().iter() {
        println!("foo : {:?}", t);
    }
    //foo.inspect();
}

#![feature(repr_simd)]
#![feature(repr_align, attr_literals)]

use std::ops::*;

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(simd)]
pub struct Vec4<T> { pub x: T, pub y: T, pub z: T, pub w: T, }
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct CVec4<T> { pub x: T, pub y: T, pub z: T, pub w: T, }
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(align(16))]
pub struct Mat4<T> { pub cols: CVec4<Vec4<T>> }

impl<T: Add<Output=T>> Add for Vec4<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}
impl<T: Mul<Output=T>> Mul for Vec4<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec4 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}
impl<T: Mul<Output=T> + Clone> Mul<T> for Vec4<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Vec4 {
            x: self.x * rhs.clone(),
            y: self.y * rhs.clone(),
            z: self.z * rhs.clone(),
            w: self.w * rhs.clone(),
        }
    }
}

impl<T> Vec4<T> {
    pub fn broadcast(val: T) -> Self where T: Clone {
        Self {
            x: val.clone(),
            y: val.clone(),
            z: val.clone(),
            w: val.clone(),
        }
    }
}
impl<T: Add<Output=T> + Mul<Output=T> + Clone> Mul<Vec4<T>> for Mat4<T> {
    type Output = Vec4<T>;
    fn mul(self, _rhs: Vec4<T>) -> Self::Output {
        unimplemented!()
    }
}
#[no_mangle]
pub fn mat4_mul_vec4_concise(a: Mat4<f32>, b: Vec4<f32>) -> Vec4<f32> {
      a.cols.x * b.x
	+ a.cols.y * b.y
	+ a.cols.z * b.z
	+ a.cols.w * b.w
}
#[no_mangle]
pub fn mat4_mul_vec4_withbroadcast(m: Mat4<f32>, rhs: Vec4<f32>) -> Vec4<f32> {
    let xxxx = Vec4::broadcast(rhs.x);
    let yyyy = Vec4::broadcast(rhs.y);
    let zzzz = Vec4::broadcast(rhs.z);
    let wwww = Vec4::broadcast(rhs.w);
    let a = m.cols.x * xxxx;
	let b = m.cols.y * yyyy;
	let c = m.cols.z * zzzz;
	let d = m.cols.w * wwww;
	a + b + c + d
}
#[no_mangle]
pub fn mat4_mul_vec4_veryexplicit(m: Mat4<f32>, rhs: Vec4<f32>) -> Vec4<f32> {
    let col0 = m.cols.x;
    let col1 = m.cols.y;
    let col2 = m.cols.z;
    let col3 = m.cols.w;
    let xxxx = Vec4::broadcast(rhs.x);
    let yyyy = Vec4::broadcast(rhs.y);
    let zzzz = Vec4::broadcast(rhs.z);
    let wwww = Vec4::broadcast(rhs.w);
    let a = col0 * xxxx;
	let b = col1 * yyyy;
	let c = col2 * zzzz;
	let d = col3 * wwww;
	a + b + c + d
}
#[no_mangle] // Clone to enforce memory reads
pub fn mat4_mul_vec4_withclone(m: Mat4<f32>, rhs: Vec4<f32>) -> Vec4<f32> {
    let m = m.clone();
    let xxxx = Vec4::broadcast(rhs.x);
    let yyyy = Vec4::broadcast(rhs.y);
    let zzzz = Vec4::broadcast(rhs.z);
    let wwww = Vec4::broadcast(rhs.w);
    let a = m.cols.x * xxxx;
	let b = m.cols.y * yyyy;
	let c = m.cols.z * zzzz;
	let d = m.cols.w * wwww;
	a + b + c + d
}
#[no_mangle]
pub fn addps(a: Vec4<f32>, b: Vec4<f32>) -> Vec4<f32> {
    a + b
}

fn main() {
    println!("Hello, world!");
}
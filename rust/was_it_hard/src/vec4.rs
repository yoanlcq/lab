use std::ops::*;
use std::slice;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

pub type Vec4f = Vec4<f32>;

impl<T> Vec4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self { Self { x, y, z, w } }
    pub fn broadcast(m: T) -> Self where T: Copy { Self::new(m, m, m, m) }
    pub fn as_ptr(&self) -> *const T { self as *const _ as *const _ }
    pub fn as_mut_ptr(&mut self) -> *mut T { self as *mut _ as *mut _ }
    pub fn as_slice(&self) -> &[T] { unsafe { slice::from_raw_parts(self.as_ptr(), 4) } }
    pub fn as_mut_slice(&mut self) -> &mut [T] { unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), 4) } }
}

impl Vec4f {
    /// A signed value which tells in which half-space of the line segment `ab` this point lies.
    ///
    /// Returns the value of twice the signed area of the `abc` triangle, which also means:
    ///
    /// - ` < 0`: This point lies in the half-space right of segment `ab`.
    /// - `== 0`: This point lies in the infinite line along segment `ab`.
    /// - ` > 0`: This point lies in the half-space left of segment `ab`.
    pub fn determine_side(self, a: Self, b: Self) -> f32 {
        let c = self;
        (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
    }
    pub fn dot(a: Self, b: Self) -> f32 { a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w }
    pub fn magnitude_squared(self) -> f32 { Self::dot(self, self) }
    pub fn magnitude(self) -> f32 { self.magnitude_squared().sqrt() }
    pub fn normalized(self) -> Self { self / self.magnitude() }
    pub fn normalize(&mut self) { *self = self.normalized(); }
    pub fn zero() -> Self { Self::broadcast(0.) }
    pub fn one() -> Self { Self::broadcast(1.) }
    pub fn min(a: Self, b: Self) -> Self {
        use ::cmp::partial_min as min;
        Self::new(min(a.x, b.x), min(a.y, b.y), min(a.z, b.z), min(a.w, b.w))
    }
    pub fn max(a: Self, b: Self) -> Self {
        use ::cmp::partial_max as max;
        Self::new(max(a.x, b.x), max(a.y, b.y), max(a.z, b.z), max(a.w, b.w))
    }
}

impl<T> Deref for Vec4<T> {
    type Target = [T];
    fn deref(&self) -> &[T] { self.as_slice() }
}

impl<T> DerefMut for Vec4<T> {
    fn deref_mut(&mut self) -> &mut [T] { self.as_mut_slice() }
}

macro_rules! vec4_unop {
    ($Op:ident $op:ident) => {
        impl<T: $Op<Output=T>> $Op for Vec4<T> {
            type Output = Self;
            fn $op(self) -> Self {
                Self {
                    x: self.x.$op(),
                    y: self.y.$op(),
                    z: self.z.$op(),
                    w: self.w.$op(),
                }
            }
        }
    };
}
macro_rules! vec4_binop {
    ($Op:ident $op:ident) => {
        impl<T: $Op<T, Output=T>> $Op for Vec4<T> {
            type Output = Self;
            fn $op(self, rhs: Self) -> Self {
                Self {
                    x: self.x.$op(rhs.x),
                    y: self.y.$op(rhs.y),
                    z: self.z.$op(rhs.z),
                    w: self.w.$op(rhs.w),
                }
            }
        }
        impl<T: Copy + $Op<T, Output=T>> $Op<T> for Vec4<T> {
            type Output = Self;
            fn $op(self, rhs: T) -> Self {
                self.$op(Self::broadcast(rhs))
            }
        }
    };
}

vec4_binop!{Add add}
vec4_binop!{Sub sub}
vec4_binop!{Mul mul}
vec4_binop!{Div div}
vec4_binop!{Rem rem}
vec4_unop!{Neg neg}


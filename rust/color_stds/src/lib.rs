//! # Examples
//! 
//! ```
//! #[macro_use]
//! extern crate color_stds;
//! use color_stds::{
//!     Rgb24, 
//!     BasicColors as Basic,
//!     CssColors as Css, 
//!     XkcdColors as Xkcd, 
//! };
//! 
//! #[repr(packed)]
//! #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
//! struct MyRgba {
//!     r:f32, g:f32, b:f32, a:f32
//! }
//! impl From<Rgb24> for MyRgba {
//!     fn from(rgb : Rgb24) -> Self {
//!         MyRgba { 
//!             r : rgb.r as f32/255_f32, 
//!             g : rgb.g as f32/255_f32, 
//!             b : rgb.b as f32/255_f32, 
//!             a : 1_f32
//!         }
//!     }
//! }
//! // impl Xkcd for MyRgba {}
//! // impl Css for MyRgba {}
//! // impl Basic for MyRgba {}
//! impl_all_color_stds!(MyRgba);
//! 
//! fn main() {
//!     // "French Blue" is an XKCD color.
//!     println!("French Blue : {:?}", MyRgba::french_blue());
//!     
//!     // Standards have their own opinion on what "Green" is.
//!     println!("Green (basic) : {:?}", <MyRgba as Basic>::green());
//!     println!("Green (CSS)   : {:?}", <MyRgba as Css  >::green());
//!     println!("Green (XKCD)  : {:?}", <MyRgba as Xkcd >::green());
//! }
//! ```
//! 
//! TODO
//! ```toml
//! [dependencies.color_stds]
//! version = "0.2.0"
//! features = ["tables"]
//! ```

pub mod rgb24;
pub use rgb24::Rgb24;
pub mod tables;
pub use tables::Entry;

pub mod basic;
pub use basic::BasicColors;
#[cfg(any(feature="tables", feature="basic_table"))]
pub use basic::BASIC_COLORS;

pub mod xkcd;
pub use xkcd::XkcdColors;
#[cfg(any(feature="tables", feature="xkcd_table"))]
pub use xkcd::XKCD_COLORS;

pub mod css;
pub use css::CssColors;
#[cfg(any(feature="tables", feature="css_table"))]
pub use css::CSS_COLORS;

pub mod crayola;
pub use crayola::CrayolaColors;
#[cfg(any(feature="tables", feature="crayola_table"))]
pub use crayola::CRAYOLA_COLORS;

#[macro_export]
macro_rules! impl_all_color_stds {
    ($t:ty) => {
        impl $crate::BasicColors for $t {}
        impl $crate::XkcdColors for $t {}
        impl $crate::CssColors for $t {}
        impl $crate::CrayolaColors for $t {}
    }
}

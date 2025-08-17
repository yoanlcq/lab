//! Adds the trivial `AsAny` trait implemented automatically for all types that are `Any`.
//! 
//! The primary purpose is to streamline casting a trait object into a concrete type.
//! [See this StackOverflow answer](https://stackoverflow.com/a/33687996).
//! 
//! # Example
//! 
//! ```
//! # use as_any::AsAny;
//! #
//! // Define an AbstractLogger trait.
//! // Requiring `AsAny` automatically gives us the `as_any()` method that will allow us to try downcasting to any of the types that implement it.
//! trait AbstractLogger: AsAny {
//!     fn log(&self, msg: &str);
//! }
//! 
//! struct ConcreteLogger;
//! 
//! impl AbstractLogger for ConcreteLogger {
//!     fn log(&self, msg: &str) { println!("{}", msg); }
//! }
//! 
//! // Instantiate a concrete logger
//! let concrete_logger = ConcreteLogger;
//! 
//! // Get a trait object from it (this is what will generally be used in the codebase)
//! let logger: &dyn AbstractLogger = &concrete_logger;
//! 
//! // If you'd like to cast it back to a ConcreteLogger if possible, you can use `as_any()`.
//! let downcast_result = logger.as_any().downcast_ref::<ConcreteLogger>().unwrap();
//! assert!(core::ptr::eq(&concrete_logger, downcast_result));
//! ```
 
#![no_std]

use core::any::Any;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

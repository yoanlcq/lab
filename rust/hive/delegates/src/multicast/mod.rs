pub mod exclusive;
pub mod shared;

pub use exclusive::*;
pub use shared::*;

#[expect(clippy::module_name_repetitions, reason = "We don't want it to be named just `DelegateResult`")]
#[must_use = "The delegate uses this to know if the listener hasn't expired"]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MulticastDelegateResult {
    Keep,
    Remove,
}

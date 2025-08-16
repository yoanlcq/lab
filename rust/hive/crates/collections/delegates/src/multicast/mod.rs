pub mod exclusive;
pub mod shared;

pub use exclusive::*;
pub use shared::*;

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BroadcastStats {
    pub had_any_listener: bool,
    pub has_called_any_listener: bool,
}
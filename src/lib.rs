//! Deez is a DynamoDB abstraction for implementing Single Table Design easily,
//! inspired by [`ElectroDB`].
//!
//! [`ElectroDB`]: https://github.com/tywalch/electrodb

// todo: translate README to rustdoc

mod deez;
mod mocks;
mod macros;

pub use crate::deez::{IndexKey, IndexKeys, Key};
pub use deez_derive::Deez;

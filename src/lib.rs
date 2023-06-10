mod deez;
mod types;

pub use deez::{Deez, DeezEntity, DeezMeta, DeezResult};
pub use deez_derive::DeezEntity;
pub use types::error::DeezError;
pub use types::schema::{Index, IndexKeys, Key, Meta};

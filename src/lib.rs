#![allow(
    clippy::type_complexity,
    clippy::needless_pass_by_value,
    clippy::module_name_repetitions,
    clippy::wildcard_imports,
    clippy::similar_names
)]

pub mod audio;
pub mod ball;
pub mod component;
pub mod debug;
pub mod error;
pub mod field;
pub mod physics;
pub mod player;

/// Prelude for convenient imports
pub mod prelude {
    pub use crate::audio::*;
    pub use crate::ball::*;
    pub use crate::component::*;
    pub use crate::debug::*;
    pub use crate::error::*;
    pub use crate::field::*;
    pub use crate::physics::*;
    pub use crate::player::*;
}

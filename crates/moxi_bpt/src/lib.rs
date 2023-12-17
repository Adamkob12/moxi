pub(crate) use bevy_ecs::prelude::*;

pub(crate) mod action;
pub(crate) mod block;
pub(crate) mod block_action;
pub(crate) mod blockreg;
pub(crate) mod chunk;
pub(crate) mod plugin;
pub(crate) mod trigger;
pub(crate) mod world;

pub(crate) use world::*;

pub mod prelude {
    pub use super::action::*;
    pub use super::block::*;
    pub use super::blockreg::*;
    pub use super::trigger::*;
    pub use super::world::blockworld::*;
    pub use super::world::*;
}

pub(crate) use bevy_ecs::prelude::*;

pub(crate) mod action;
pub(crate) mod block;
pub(crate) mod blockreg;
pub(crate) mod trigger;
pub(crate) mod world;

pub(crate) use action::*;
pub(crate) use block::*;
pub(crate) use blockreg::*;
pub(crate) use trigger::*;
pub(crate) use world::*;

pub mod prelude {
    pub use super::action::*;
    pub use super::block::*;
    pub use super::blockreg::*;
    pub use super::trigger::*;
    pub use super::world::*;
}
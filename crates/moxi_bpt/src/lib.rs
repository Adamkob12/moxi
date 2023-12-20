#![allow(dead_code)]

pub(crate) use bevy_ecs::prelude::*;

pub(crate) mod action;
pub(crate) mod block;
pub(crate) mod block_action;
pub(crate) mod blockreg;
pub(crate) mod chunk;
pub(crate) mod plugin;
pub(crate) mod trigger;
pub(crate) mod world;

use moxi_utils::prelude::Dimensions;
pub(crate) use world::*;

pub mod prelude {
    pub use super::action::*;
    pub use super::block::*;
    pub use super::blockreg::*;
    pub use super::trigger::*;
    pub use super::world::blockworld::*;
    pub use super::world::*;
}

static CHUNK_DIMS: Dimensions = Dimensions::new(16, 64, 16);
static CHUNK_SIZE: u32 = CHUNK_DIMS.x * CHUNK_DIMS.y * CHUNK_DIMS.z;

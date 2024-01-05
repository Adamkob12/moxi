pub(crate) use bevy_ecs::prelude::*;

pub(crate) mod action;
pub(crate) mod block;
pub(crate) mod block_action;
pub(crate) mod blockreg;
pub(crate) mod chunk;
pub(crate) mod plugin;
pub(crate) mod this_block;
pub(crate) mod trigger;
pub(crate) mod world;

pub(crate) use world::*;

pub mod prelude {
    pub use super::action::*;
    pub use super::block::*;
    pub use super::blockreg::*;
    pub use super::chunk::{chunkbuilder::*, *};
    pub use super::plugin::*;
    pub use super::trigger::*;
    pub use super::world::blockworld::*;
    pub use super::world::*;
    pub use super::{block_id, block_name, get_block_id, get_block_name};
}

#[macro_export]
macro_rules! get_block_id {
    ($name:expr) => {
        BLOCKS_GLOBAL::get_id($name)
    };
}

#[macro_export]
macro_rules! get_block_name {
    ($id:expr) => {
        BLOCKS_GLOBAL::get_name($id)
    };
}

#[macro_export]
macro_rules! block_id {
    ($name:expr) => {
        BLOCKS_GLOBAL::id($name)
    };
}

#[macro_export]
macro_rules! block_name {
    ($id:expr) => {
        BLOCKS_GLOBAL::name($id)
    };
}

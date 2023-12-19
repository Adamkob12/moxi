#![allow(dead_code)]

pub mod chunk;
pub mod dir;
pub mod face;

pub mod prelude {
    pub use super::block::*;
    pub use super::chunk::*;
    pub use super::dir::*;
    pub use super::face::*;
}

pub mod block {
    use bevy_reflect::prelude::Reflect;
    /// A trait that represents a block in the grid. It doesn't have to be the block itself, just
    /// an id that represents the block and is unique, and sized, and ideally lightweight.
    pub trait BlockInGrid: Copy + Sized + Eq + PartialEq {
        fn id(&self) -> BlockId;
        fn from_id(id: BlockId) -> Self;
        fn into_id(self) -> BlockId;
    }

    #[allow(clippy::all)]
    #[cfg(feature = "block_id_8")]
    pub type BlockId = u8;

    #[allow(clippy::all)]
    #[cfg(feature = "block_id_16")]
    pub type BlockId = u16;

    #[allow(clippy::all)]
    #[cfg(feature = "block_id_32")]
    pub type BlockId = u32;

    #[allow(clippy::all)]
    #[cfg(feature = "block_id_64")]
    pub type BlockId = u64;

    /// A block in the grid, made up of an id and meta. The meta is optional.
    /// There can be no more than 2^(chosen id size) of blocks,
    #[derive(Reflect, Copy, Clone, Eq, PartialEq)]
    pub struct GridBlock {
        pub id: BlockId,
    }

    pub type BlockName = &'static str;

    impl BlockInGrid for GridBlock {
        fn id(&self) -> BlockId {
            self.id
        }

        fn from_id(id: BlockId) -> Self {
            Self { id }
        }

        fn into_id(self) -> BlockId {
            self.id
        }
    }

    impl BlockInGrid for BlockId {
        fn id(&self) -> BlockId {
            *self
        }

        fn from_id(id: BlockId) -> Self {
            id
        }

        fn into_id(self) -> BlockId {
            self
        }
    }
}

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
    pub trait BlockInGrid: Copy + Sized + Eq + PartialEq {}

    #[cfg(feature = "block_id_8")]
    pub type BlockId = u8;

    #[cfg(feature = "block_id_16")]
    pub type BlockId = u16;

    #[cfg(feature = "block_id_32")]
    pub type BlockId = u32;

    #[cfg(feature = "block_id_64")]
    pub type BlockId = u64;

    /// A block in the grid, made up of an id and meta. The meta is optional.
    /// There can be no more than 2^(chosen id size) of blocks,
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GridBlock {
        pub id: BlockId,
    }

    impl BlockInGrid for GridBlock {}
}

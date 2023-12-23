use crate::*;
use moxi_utils::prelude::*;

/// An event that is fired when a block is updated.
#[derive(Event, Clone, Copy, Debug)]
pub struct BlockWorldUpdateEvent {
    pub block_pos: BlockPos,
    pub chunk_cords: ChunkCords,
    pub block_update: BlockUpdate,
}

/// Whether the update happend to the block itself or to an adjecent block.
#[derive(Clone, Copy, Debug)]
pub enum BlockUpdate {
    /// This block was updated.
    Pure(BlockUpdateType),
    /// A reaction to an adjecent block being updated. [`Face`] is the direction of said adjecent block.
    Reaction(Face, BlockUpdateType),
}

impl BlockUpdate {
    pub fn is_pure_and<F>(&self, preidcate: F) -> bool
    where
        F: Fn(BlockUpdateType) -> bool,
    {
        match self {
            Self::Pure(block_update) => preidcate(*block_update),
            _ => false,
        }
    }

    pub fn is_reaction_and<F>(&self, preidcate: F) -> bool
    where
        F: Fn(Face, BlockUpdateType) -> bool,
    {
        match self {
            Self::Reaction(face, block_update) => preidcate(*face, *block_update),
            _ => false,
        }
    }
}

/// The type of update that happened to a block.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockUpdateType {
    id: u128,
}

impl BlockUpdateType {
    pub const fn from_u128(id: u128) -> Self {
        Self { id }
    }

    pub const fn is(&self, id: u128) -> bool {
        self.id == id
    }
}

pub const BLOCK_REMOVED: BlockUpdateType = BlockUpdateType::from_u128(48124891481412311);
pub const BLOCK_PLACED: BlockUpdateType = BlockUpdateType::from_u128(48124891481412312);

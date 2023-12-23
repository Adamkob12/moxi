use crate::*;
use moxi_utils::prelude::*;

/// An event that is fired when a block is updated.
#[derive(Event, Clone, Copy, Debug)]
pub struct BlockWorldUpdateEvent {
    pub block_id: BlockId,
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

/// The type of update that happened to a block.
#[derive(Clone, Copy, Debug)]
pub enum BlockUpdateType {
    BlockPlaced,
    BlockRemoved,
    BlockUpdated,
}

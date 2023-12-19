use crate::*;
use moxi_utils::prelude::*;

pub enum BlockCommand {
    PlaceBlock(BlockName, BlockPos, ChunkCords),
    BreakBlock(BlockPos, ChunkCords),
    UpdateBlock(BlockPos, ChunkCords),
}

#[derive(Resource, Default)]
pub struct BlockCommands {
    commands: Vec<BlockCommand>,
}

impl BlockCommands {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn place_block(
        &mut self,
        block_name: BlockName,
        block_pos: BlockPos,
        chunk_cords: ChunkCords,
    ) {
        self.commands
            .push(BlockCommand::PlaceBlock(block_name, block_pos, chunk_cords));
    }

    pub fn break_block(&mut self, block_pos: BlockPos, chunk_cords: ChunkCords) {
        self.commands
            .push(BlockCommand::BreakBlock(block_pos, chunk_cords));
    }

    pub fn update_block(&mut self, block_pos: BlockPos, chunk_cords: ChunkCords) {
        self.commands
            .push(BlockCommand::UpdateBlock(block_pos, chunk_cords));
    }

    pub fn drain(&mut self) -> Vec<BlockCommand> {
        std::mem::take(&mut self.commands)
    }
}

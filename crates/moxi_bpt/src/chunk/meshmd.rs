use bevy_ecs::component::Component;
use moxi_mesh_utils::{prelude::*, BlockMeshChange};
use moxi_utils::prelude::{BlockId, BlockPos, SurroundingBlocks};

#[derive(Component)]
pub enum ChunkMeshMd {
    Cube(CubeMD<BlockId>),
    Xsprite(XSpriteMD<BlockId>),
    Custom(CustomMD<BlockId>),
}

impl ChunkMeshMd {
    pub fn log_block_break(
        &mut self,
        block_pos: BlockPos,
        block_id: BlockId,
        surrounding_blocks: SurroundingBlocks<BlockId>,
    ) {
        match self {
            ChunkMeshMd::Cube(md) => md.log(
                BlockMeshChange::Broken,
                block_pos,
                block_id,
                surrounding_blocks,
            ),
            ChunkMeshMd::Xsprite(md) => md.log_break(block_id, block_pos),
            ChunkMeshMd::Custom(md) => md.log_break(block_id, block_pos),
        }
    }

    pub fn log_block_add(
        &mut self,
        block_pos: BlockPos,
        placed_block_id: BlockId,
        surrounding_blocks: SurroundingBlocks<BlockId>,
    ) {
        match self {
            ChunkMeshMd::Cube(md) => md.log(
                BlockMeshChange::Added,
                block_pos,
                placed_block_id,
                surrounding_blocks,
            ),
            ChunkMeshMd::Xsprite(md) => md.log_add(placed_block_id, block_pos),
            ChunkMeshMd::Custom(md) => md.log_add(placed_block_id, block_pos),
        }
    }

    pub fn update_block(
        &mut self,
        update_type: BlockMeshChange,
        block_pos: BlockPos,
        block_id: BlockId,
        surrounding_blocks: SurroundingBlocks<BlockId>,
    ) {
        match self {
            ChunkMeshMd::Cube(md) => md.log(update_type, block_pos, block_id, surrounding_blocks),
            ChunkMeshMd::Xsprite(_) | ChunkMeshMd::Custom(_) => {}
        }
    }
}

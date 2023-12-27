use crate::chunk::components::{ChildMeshChunks, ChunkGrid};
use crate::chunk::resources::ChunkMap;
use crate::prelude::{
    BlockIdtoEnt, BlockMarker, BlockName, GlobalBlockBreak, GlobalBlockPlace, BLOCKS_GLOBAL,
};
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use moxi_utils::prelude::{
    global_enumerate_neighboring_blocks, BlockGlobalPos, BlockId, BlockPos, ChunkCords, Dimensions,
    Face, Grid, SurroundingBlocks,
};

#[derive(SystemParam)]
pub struct Blocks<'w, 's, const N: usize> {
    _blocks_query: Query<'w, 's, (&'static BlockMarker, &'static BlockName)>,
    pub(crate) chunk_map: Res<'w, ChunkMap>,
    pub(crate) chunks_query: Query<'w, 's, (&'static mut ChunkGrid<N>, &'static ChildMeshChunks)>,
    _block_id_to_ent: Res<'w, BlockIdtoEnt>,
}

#[derive(SystemParam)]
pub struct BlocksMut<'w, 's, const N: usize> {
    blocks: Blocks<'w, 's, N>,
    global_block_place_sender: EventWriter<'w, GlobalBlockPlace>,
    global_block_break_sender: EventWriter<'w, GlobalBlockBreak>,
}

impl<'w, 's, const N: usize> std::ops::Deref for BlocksMut<'w, 's, N> {
    type Target = Blocks<'w, 's, N>;

    fn deref(&self) -> &Self::Target {
        &self.blocks
    }
}

impl<'w, 's, const N: usize> BlocksMut<'w, 's, N> {
    pub fn set_block_at_id(
        &mut self,
        chunk_cords: ChunkCords,
        block_pos: BlockPos,
        block_id: BlockId,
    ) {
        let current_block = self.get_block_id_at(chunk_cords, block_pos).unwrap_or(0);

        if current_block != 0 {
            self.global_block_break_sender.send(GlobalBlockBreak {
                chunk_cords,
                block_pos,
                block_id: current_block,
            });
        }
        if block_id != 0 {
            self.global_block_place_sender.send(GlobalBlockPlace {
                chunk_cords,
                block_pos,
                block_id,
            });
        }
    }

    pub fn set_block_at_name(
        &mut self,
        chunk_cords: ChunkCords,
        block_pos: BlockPos,
        block_name: &'static str,
    ) {
        let block_id = BLOCKS_GLOBAL::id(block_name);
        self.set_block_at_id(chunk_cords, block_pos, block_id);
    }
}

impl<'w, 's, const N: usize> Blocks<'w, 's, N> {
    pub fn get_block_name_at(
        &self,
        chunk_cords: ChunkCords,
        block_pos: BlockPos,
    ) -> Option<&'static str> {
        let chunk = self.chunk_map.get_chunk(chunk_cords)?;
        let chunk = self.chunks_query.get(chunk).ok()?;
        let block_id = chunk.0.get_block(block_pos)?;
        BLOCKS_GLOBAL::get_name(block_id)
    }

    pub fn block_name_at(&self, chunk_cords: ChunkCords, block_pos: BlockPos) -> &'static str {
        self.get_block_name_at(chunk_cords, block_pos)
            .unwrap_or("Air")
    }

    pub fn get_block_id_at(&self, chunk_cords: ChunkCords, block_pos: BlockPos) -> Option<BlockId> {
        let chunk = self.chunk_map.get_chunk(chunk_cords)?;
        let chunk = self.chunks_query.get(chunk).ok()?;
        let block = chunk.0.get_block(block_pos)?;
        Some(block)
    }

    pub fn block_id_at(&self, chunk_cords: ChunkCords, block_pos: BlockPos) -> BlockId {
        self.get_block_id_at(chunk_cords, block_pos).unwrap_or(0)
    }

    pub fn get_chunk_grid(&self, chunk_cords: ChunkCords) -> Option<&Grid<BlockId, N>> {
        let chunk = self.chunk_map.get_chunk(chunk_cords)?;
        let chunk = self.chunks_query.get(chunk).ok()?;
        Some(&chunk.0 .0)
    }

    const PLACEHOLDER_DIMS: Dimensions = Dimensions::new(16, 16, 16);

    pub fn get_global_surrounding_blocks(
        &self,
        chunk_cords: ChunkCords,
        block_pos: BlockPos,
    ) -> SurroundingBlocks<(Face, ChunkCords, BlockPos, BlockId)> {
        let default = SurroundingBlocks::default();
        let chunk_entity = self.chunk_map.get_chunk(chunk_cords);
        if chunk_entity.is_none() {
            return default;
        }
        let chunk_grid = self.chunks_query.get(chunk_entity.unwrap()).unwrap().0;
        let global_block_pos = BlockGlobalPos::new(block_pos, chunk_cords);
        global_enumerate_neighboring_blocks(global_block_pos, Self::PLACEHOLDER_DIMS)
            .map(|(face, gbp)| {
                let neighbor_chunk_cords = gbp.cords;
                let neighbor_block_pos = gbp.pos;
                if neighbor_chunk_cords == chunk_cords {
                    chunk_grid
                        .0
                        .get_block(neighbor_block_pos)
                        .map(|id| (face, neighbor_chunk_cords, neighbor_block_pos, id))
                } else {
                    self.get_block_id_at(neighbor_chunk_cords, neighbor_block_pos)
                        .map(|id| (face, neighbor_chunk_cords, neighbor_block_pos, id))
                }
            })
            .collect::<Vec<Option<(Face, ChunkCords, BlockPos, BlockId)>>>()
            .as_slice()
            .try_into()
            .unwrap()
    }

    pub fn get_global_surrounding_blocks_ids(
        &self,
        chunk_cords: ChunkCords,
        block_pos: BlockPos,
    ) -> SurroundingBlocks<BlockId> {
        let chunk = self.chunk_map.get_chunk(chunk_cords).unwrap();
        let chunk = self.chunks_query.get(chunk).unwrap().0;
        let global_block_pos = BlockGlobalPos::new(block_pos, chunk_cords);
        global_enumerate_neighboring_blocks(global_block_pos, Self::PLACEHOLDER_DIMS)
            .map(|(_, gbp)| {
                let neighbor_chunk_cords = gbp.cords;
                let neighbor_block_pos = gbp.pos;
                if neighbor_chunk_cords == chunk_cords {
                    chunk.0.get_block(neighbor_block_pos)
                } else {
                    self.get_block_id_at(neighbor_chunk_cords, neighbor_block_pos)
                }
            })
            .collect::<Vec<Option<BlockId>>>()
            .as_slice()
            .try_into()
            .unwrap()
    }

    pub fn get_global_surrounding_blocks_names(
        &self,
        chunk_cords: ChunkCords,
        block_pos: BlockPos,
    ) -> SurroundingBlocks<&'static str> {
        let chunk = self.chunk_map.get_chunk(chunk_cords).unwrap();
        let chunk = self.chunks_query.get(chunk).unwrap().0;
        let global_block_pos = BlockGlobalPos::new(block_pos, chunk_cords);
        global_enumerate_neighboring_blocks(global_block_pos, Self::PLACEHOLDER_DIMS)
            .map(|(_, gbp)| {
                let neighbor_chunk_cords = gbp.cords;
                let neighbor_block_pos = gbp.pos;
                if neighbor_chunk_cords == chunk_cords {
                    let block_id = chunk.0.get_block(neighbor_block_pos).unwrap_or(0);
                    BLOCKS_GLOBAL::get_name(block_id)
                } else {
                    self.get_block_name_at(neighbor_chunk_cords, neighbor_block_pos)
                }
            })
            .collect::<Vec<Option<&'static str>>>()
            .as_slice()
            .try_into()
            .unwrap()
    }
}

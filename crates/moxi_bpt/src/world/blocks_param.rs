use crate::blockreg::BlockRegistry;
use crate::chunk::components::{Chunk, ChunkGrid};
use crate::chunk::resources::ChunkMap;
use crate::prelude::{BlockIdtoEnt, BlockMarker, BlockName};
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use moxi_utils::prelude::{
    global_enumerate_neighboring_blocks, BlockGlobalPos, BlockId, BlockPos, ChunkCords, Dimensions,
    Grid, SurroundingBlocks,
};

#[derive(SystemParam)]
pub struct Blocks<'w, 's, const N: usize> {
    blocks_query: Query<'w, 's, (&'static BlockMarker, &'static BlockName)>,
    chunk_map: Res<'w, ChunkMap>,
    chunks_query: Query<'w, 's, (&'static ChunkGrid<N>, &'static Chunk)>,
    block_id_to_ent: Res<'w, BlockIdtoEnt>,
    block_registry: Res<'w, BlockRegistry>,
}

impl<'w, 's, const N: usize> Blocks<'w, 's, N> {
    pub fn get_block_name_at(
        &self,
        chunk_cords: ChunkCords,
        block_pos: BlockPos,
    ) -> Option<&'static str> {
        let chunk = self.chunk_map.get_chunk(chunk_cords)?;
        let chunk = self.chunks_query.get(chunk).ok()?;
        let block = chunk.0.get_block(block_pos)?;
        self.block_registry.id_to_name.get(&block).copied()
    }

    pub fn get_block_id_at(&self, chunk_cords: ChunkCords, block_pos: BlockPos) -> Option<BlockId> {
        let chunk = self.chunk_map.get_chunk(chunk_cords)?;
        let chunk = self.chunks_query.get(chunk).ok()?;
        let block = chunk.0.get_block(block_pos)?;
        Some(block)
    }

    pub fn get_chunk_grid(&self, chunk_cords: ChunkCords) -> Option<&Grid<BlockId, N>> {
        let chunk = self.chunk_map.get_chunk(chunk_cords)?;
        let chunk = self.chunks_query.get(chunk).ok()?;
        Some(&chunk.0 .0)
    }

    const PLACEHOLDER_DIMS: Dimensions = Dimensions::new(16, 16, 16);

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
                    self.block_registry
                        .id_to_name
                        .get(&chunk.0.get_block(neighbor_block_pos).unwrap_or(0))
                        .copied()
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

use std::sync::Arc;

use crate::*;
use moxi_utils::prelude::*;

pub trait ChunkBuilder<const N: usize, B: BlockInGrid>: Send + Sync {
    fn build_chunk(&self, chunk_cords: ChunkCords) -> Grid<B, N>;
}

#[derive(Resource)]
pub struct BoxedBuilder<const N: usize> {
    pub builder: Arc<dyn ChunkBuilder<N, BlockId>>,
}

impl<const N: usize> ChunkBuilder<N, BlockId> for BoxedBuilder<N> {
    fn build_chunk(&self, chunk_cods: ChunkCords) -> Grid<BlockId, N> {
        self.builder.build_chunk(chunk_cods)
    }
}

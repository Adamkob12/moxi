use crate::*;
use moxi_utils::prelude::*;

pub trait ChunkBuilder<B: BlockInGrid, const N: usize> {
    fn build_chunk(&mut self, chunk_cods: ChunkCords) -> ChunkGrid<B, N>;
}

#[derive(Resource)]
pub struct BoxedBuilder<B: BlockInGrid, const N: usize> {
    pub builder: Box<dyn ChunkBuilder<B, N>>,
}

impl<const N: usize, B: BlockInGrid> ChunkBuilder<B, N> for BoxedBuilder<B, N> {
    fn build_chunk(&mut self, chunk_cods: ChunkCords) -> ChunkGrid<B, N> {
        self.builder.build_chunk(chunk_cods)
    }
}

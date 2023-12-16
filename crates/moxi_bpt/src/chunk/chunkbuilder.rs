use crate::*;
use moxi_utils::prelude::*;

pub trait ChunkBuilder<const N: usize, B: BlockInGrid> {
    fn build_chunk(&mut self, chunk_cods: ChunkCords) -> ChunkGrid<B, N>;
}

#[derive(Resource)]
pub struct BoxedBuilder<const N: usize, B: BlockInGrid> {
    pub builder: Box<dyn ChunkBuilder<N, B>>,
}

impl<const N: usize, B: BlockInGrid> ChunkBuilder<N, B> for BoxedBuilder<N, B> {
    fn build_chunk(&mut self, chunk_cods: ChunkCords) -> ChunkGrid<B, N> {
        self.builder.build_chunk(chunk_cods)
    }
}

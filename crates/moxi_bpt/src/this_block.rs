// use std::ops::Deref;
//
// use bevy_ecs::system::In;
// use moxi_utils::prelude::{BlockPos, ChunkCords};
//
// use crate::prelude::{Blocks, BlocksMut};
//
// pub type ThisBlock<'w, 's, const N: usize> = In<ABlock<'w, 's, N>>;
// pub type ThisBlockMut<'w, 's, const N: usize> = In<ABlockMut<'w, 's, N>>;
//
// pub(crate) struct ABlock<'w, 's, const N: usize> {
//     blocks: Blocks<'w, 's, N>,
//     block_pos: BlockPos,
//     chunk_cords: ChunkCords,
// }
//
// pub(crate) struct ABlockMut<'w, 's, const N: usize> {
//     blocks: BlocksMut<'w, 's, N>,
//     block_pos: BlockPos,
//     chunk_cords: ChunkCords,
// }

use bevy_ecs::{component::Component, entity::Entity};
use moxi_utils::prelude::{BlockId, ChunkCords, Grid};

#[derive(Component)]
pub struct Chunk {
    pub cords: ChunkCords,
}

#[derive(Component)]
pub struct ChunkGrid<const N: usize>(pub Grid<BlockId, N>);

#[derive(Component)]
pub struct ToUpdate;

#[derive(Component)]
pub struct MeshChunk {
    pub parent_chunk: Entity,
}

#[derive(Component)]
pub struct CubeMeshChunk;

#[derive(Component)]
pub struct XSpriteMeshChunk;

#[derive(Component)]
pub struct CustomMeshChunk;

#[derive(Component)]
pub enum ChunkMeshType {
    Cube,
    Xsprite,
    Custom,
}

impl<const N: usize> std::ops::Deref for ChunkGrid<N> {
    type Target = Grid<BlockId, N>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> std::ops::DerefMut for ChunkGrid<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

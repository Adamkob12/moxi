use bevy_ecs::{component::Component, entity::Entity};
use moxi_mesh_utils::prelude::BlockMeshType;
use moxi_utils::prelude::{BlockId, ChunkCords, Face, Grid};

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
pub struct ChildMeshChunks {
    pub cube_mesh_chunk: Entity,
    pub xsprite_mesh_chunk: Entity,
    pub custom_mesh_chunk: Entity,
}

#[derive(Component)]
pub struct CubeMeshChunk;

#[derive(Component)]
pub struct XSpriteMeshChunk;

#[derive(Component)]
pub struct CustomMeshChunk;

#[derive(Component)]
pub struct ToIntroduce {
    pub cords: ChunkCords,
    pub adj_chunks_to_introduce: Vec<Face>,
}

#[derive(Component)]
pub enum ChunkMeshType {
    Cube,
    XSprite,
    Custom,
}

impl From<BlockMeshType> for ChunkMeshType {
    fn from(mesh_type: BlockMeshType) -> Self {
        match mesh_type {
            BlockMeshType::XSprite => Self::XSprite,
            BlockMeshType::Custom => Self::Custom,
            _ => Self::Cube,
        }
    }
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

impl ChildMeshChunks {
    pub fn get_from_type(&self, mesh_type: ChunkMeshType) -> Entity {
        match mesh_type {
            ChunkMeshType::Cube => self.cube_mesh_chunk,
            ChunkMeshType::XSprite => self.xsprite_mesh_chunk,
            ChunkMeshType::Custom => self.custom_mesh_chunk,
        }
    }
}

impl ToIntroduce {
    pub fn new(chunk_cords: ChunkCords) -> Self {
        Self {
            cords: chunk_cords,
            adj_chunks_to_introduce: vec![Face::Right, Face::Left, Face::Front, Face::Back],
        }
    }
}

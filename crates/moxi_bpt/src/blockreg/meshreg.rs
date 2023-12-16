use bevy_asset::AssetId;
use bevy_ecs::system::Resource;
use bevy_reflect::Reflect;
use bevy_render::mesh::Mesh;
use moxi_mesh_utils::prelude::{BlockMesh, BlockMeshRef, BlockMeshType, MeshRegistry};
use moxi_utils::prelude::BlockId;

#[derive(Reflect, Resource)]
pub struct MeshReg {
    pub(crate) meshes: Vec<BlockMesh>,
    pub(crate) assets: Vec<AssetId<Mesh>>,
}

impl MeshRegistry<BlockId> for MeshReg {
    fn get_block_mesh_ref(&self, block: &BlockId) -> BlockMeshRef {
        self.meshes[*block as usize].as_ref()
    }

    fn get_block_mesh_asset_id(&self, block: &BlockId) -> AssetId<Mesh> {
        self.assets[*block as usize]
    }

    fn get_block_mesh_type(&self, block: &BlockId) -> BlockMeshType {
        self.meshes[*block as usize].get_type()
    }
}

impl MeshReg {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            assets: Vec::new(),
        }
    }
}

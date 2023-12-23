use bevy_asset::Handle;
use bevy_ecs::system::Resource;
use bevy_reflect::Reflect;
use bevy_render::mesh::Mesh;
use moxi_mesh_utils::prelude::{BlockMesh, BlockMeshRef, BlockMeshType, MeshRegistry};
use moxi_utils::prelude::BlockId;

#[derive(Reflect, Resource, Default, Clone)]
pub struct MeshReg {
    pub(crate) meshes: Vec<BlockMesh>,
    pub(crate) handles: Vec<Handle<Mesh>>,
}

impl MeshRegistry<BlockId> for MeshReg {
    fn get_block_mesh_ref(&self, block: &BlockId) -> BlockMeshRef {
        self.meshes[*block as usize].as_ref()
    }

    fn get_block_mesh_handle(&self, block: &BlockId) -> Handle<Mesh> {
        Handle::clone(&self.handles[*block as usize])
    }

    fn get_block_mesh_type(&self, block: &BlockId) -> BlockMeshType {
        self.meshes[*block as usize].get_type()
    }
}

impl MeshReg {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            handles: Vec::new(),
        }
    }
}

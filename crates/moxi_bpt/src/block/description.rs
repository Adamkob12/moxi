use bevy_ecs::prelude::Bundle;
use moxi_mesh_utils::prelude::{BlockMesh, BlockMeshType};

pub trait Block {
    fn get_name() -> &'static str {
        std::any::type_name::<Self>()
    }
    fn get_mesh() -> BlockMesh {
        BlockMesh::Air
    }
    fn static_properties() -> impl Bundle {
        ()
    }
}

pub(crate) trait CommonBlock: Block {
    fn get_mesh_type() -> BlockMeshType {
        Self::get_mesh().get_type()
    }
}

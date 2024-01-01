use bevy_ecs::prelude::Bundle;
use moxi_mesh_utils::prelude::{BlockMesh, BlockMeshType};

#[allow(unused_imports)]
use crate::{
    prelude::{BlockWorldUpdateEvent, IntoActionSet},
    trigger::IntoTrigger,
};

pub trait Block {
    fn get_name() -> &'static str {
        std::any::type_name::<Self>()
    }
    fn get_mesh() -> BlockMesh {
        BlockMesh::Air
    }
    fn get_static_properties() -> impl Bundle {
        ()
    }
    // fn get_block_actions<I, M1, M2, M3, T, IA, NA>() -> Vec<(T, IA, NA)>
    // where
    //     T: IntoTrigger<I, M1>,
    //     IA: IntoActionSet<(), M2>,
    //     NA: IntoActionSet<BlockWorldUpdateEvent, M3>,
    // {
    //     vec![]
    // }
}

pub(crate) trait CommonBlock: Block {
    fn get_mesh_type() -> BlockMeshType {
        Self::get_mesh().get_type()
    }
}

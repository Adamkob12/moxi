use crate::{
    prelude::{ActionSet, IntoActionSet},
    trigger::{IntoTrigger, Trigger},
};
use moxi_mesh_utils::prelude::{BlockMesh, BlockMeshType};

use super::property::*;

pub trait Block {
    fn description() -> BlockDescription;
}

pub struct BlockDescription {
    name: Option<&'static str>,
    block_mesh: BlockMesh,
    static_properties: Vec<BoxedProperty<STATIC_PROPERTY_TYPE>>,
    dynamic_properties: Vec<BoxedProperty<DYNAMIC_PROPERTY_TYPE>>,
    block_actions: Vec<(Trigger, ActionSet)>,
    block_mesh_type: BlockMeshType,
    block_type: BlockType,
}

impl BlockDescription {
    pub fn new(block_mesh: BlockMesh) -> Self {
        let block_mesh_type = block_mesh.get_type();
        BlockDescription {
            name: None,
            block_mesh,
            static_properties: Vec::new(),
            dynamic_properties: Vec::new(),
            block_actions: Vec::new(),
            block_mesh_type,
            block_type: BlockType::Static,
        }
    }

    pub fn add_static_properties<B: PropertyBundle<STATIC_PROPERTY_TYPE>>(
        mut self,
        bundle: B,
    ) -> Self {
        self.static_properties.extend(bundle.get_properties());
        self
    }

    pub fn add_dynamic_properties<B: PropertyBundle<DYNAMIC_PROPERTY_TYPE>>(
        mut self,
        bundle: B,
    ) -> Self {
        self.dynamic_properties.extend(bundle.get_properties());
        self.block_type = BlockType::Dynamic;
        self
    }

    pub fn add_action(mut self, trigger: impl IntoTrigger, action_set: impl IntoActionSet) -> Self {
        self.block_actions
            .push((trigger.into_trigger(), action_set.into_action_set()));
        self
    }
}

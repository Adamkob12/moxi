use super::property::*;
use crate::{
    prelude::{ActionSet, IntoActionSet},
    trigger::{IntoTrigger, Trigger},
};
use bevy_ecs::bundle::Bundle;
use moxi_mesh_utils::prelude::BlockMesh;

pub trait Block<B: Bundle> {
    fn init() -> BlockDescription<B>;
}

pub struct BlockDescription<B: Bundle> {
    pub(crate) name: Option<&'static str>,
    pub(crate) block_mesh: BlockMesh,
    pub(crate) static_properties: B,
    pub(crate) dynamic_properties: DynamicProperties,
    pub(crate) block_actions: Vec<(Trigger, ActionSet)>,
}

impl<B: Bundle> BlockDescription<B> {
    pub fn new(block_mesh: BlockMesh) -> BlockDescription<()> {
        BlockDescription::<()> {
            name: None,
            block_mesh,
            static_properties: (),
            dynamic_properties: DynamicProperties::default(),
            block_actions: Vec::new(),
        }
    }

    pub fn add_static_properties(mut self, bundle: B) -> Self {
        self.static_properties = bundle;
        self
    }

    pub fn add_dynamic_properties<D: DynamicProperty>(
        mut self,
        dynamic_properties: Vec<D>,
    ) -> Self {
        self.dynamic_properties.extend(dynamic_properties);
        self
    }

    pub fn add_action(mut self, trigger: impl IntoTrigger, action_set: impl IntoActionSet) -> Self {
        self.block_actions
            .push((trigger.into_trigger(), action_set.into_action_set()));
        self
    }
}

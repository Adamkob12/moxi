use crate::blockreg::meshreg::MeshReg;
use crate::*;
use action::{Action, CommonActionSet};
use bevy_asset::{AssetId, Assets, Handle};
use bevy_render::mesh::Mesh;
use moxi_mesh_utils::prelude::BlockMesh;
use moxi_utils::prelude::BlockId;
use prelude::{Block, BlockDescription, BlockRegistry, DynamicBlock, StaticBlock, Trigger};
use std::any::TypeId;
use std::collections::{HashMap, HashSet};

type TriggerId = TypeId;
type ActionId = TypeId;

pub struct BlockWorld {
    block_name_hash_set: HashSet<&'static str>,
    block_id_counter: BlockId,
    block_name_to_block_id: HashMap<&'static str, BlockId>,
    block_id_to_block_actions: HashMap<BlockId, Vec<(TriggerId, Vec<ActionId>)>>,
    triggers_map: HashMap<TriggerId, Trigger>,
    actions_map: HashMap<ActionId, Action>,
    block_to_block_mesh: HashMap<BlockId, BlockMesh>,
    block_id_to_ent: BlockIdtoEnt,
}

impl Default for BlockWorld {
    fn default() -> Self {
        Self {
            block_id_counter: 0,
            block_name_hash_set: HashSet::new(),
            block_name_to_block_id: HashMap::new(),
            triggers_map: HashMap::new(),
            block_id_to_block_actions: HashMap::new(),
            actions_map: HashMap::new(),
            block_to_block_mesh: HashMap::new(),
            block_id_to_ent: BlockIdtoEnt(HashMap::new()),
        }
    }
}

#[derive(Component)]
pub(crate) struct BlockActions(pub Vec<(TriggerId, Vec<ActionId>)>);

#[derive(Component)]
pub(crate) struct BlockComponent;

#[derive(Component)]
pub(crate) struct TriggerMarker;

#[derive(Component)]
pub(crate) struct ActionMarker;

#[derive(Resource)]
pub(crate) struct BlockIdtoEnt(HashMap<BlockId, Entity>);

#[derive(Resource)]
pub(crate) struct TriggersMap(HashMap<TriggerId, Trigger>);

#[derive(Resource)]
pub(crate) struct ActionsMap(HashMap<ActionId, Action>);

impl BlockWorld {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init_world(mut self, world: &mut World) {
        let mut meshes = world
            .get_resource_mut::<Assets<Mesh>>()
            .expect("AssetServer not found");
        let mut mesh_reg = MeshReg::new();
        // mesh
        for block_id in 0..self.block_id_counter {
            let block_mesh = self.block_to_block_mesh.remove(&block_id).unwrap();
            let asset_id = {
                if let Some(mesh) = block_mesh.clone().as_option() {
                    let mesh_handle: Handle<Mesh> = meshes.add(mesh);
                    let asset_id = mesh_handle.id();
                    asset_id
                } else {
                    AssetId::invalid()
                }
            };
            mesh_reg.assets.push(asset_id);
            mesh_reg.meshes.push(block_mesh);
        }

        world.insert_resource(BlockRegistry::new(self.block_name_to_block_id));
        world.insert_resource(mesh_reg);
        world.insert_resource(self.block_id_to_ent);
        world.insert_resource(ActionsMap(self.actions_map));
        world.insert_resource(TriggersMap(self.triggers_map));
    }

    pub fn init_block<T: Bundle, B: Block<T>>(self, world: &mut World) -> Self {
        let mut block_description = B::init();
        block_description.name = Some(
            block_description
                .name
                .map_or_else(|| std::any::type_name::<B>(), |n| n),
        );
        self.insert_block(block_description, world)
    }

    pub fn insert_block<T: Bundle>(
        mut self,
        block_description: BlockDescription<T>,
        world: &mut World,
    ) -> Self {
        let id = self.block_id_counter;
        let name = block_description
            .name
            .expect("Cannot insert nameless block");
        assert!(
            self.block_name_hash_set.insert(name),
            "Block with name {} already exists",
            name
        );
        self.block_to_block_mesh
            .insert(id, block_description.block_mesh);
        self.block_name_to_block_id.insert(name, id);

        let block_actions = block_description.block_actions;
        let static_properties = block_description.static_properties;
        let dynamic_properties = block_description.dynamic_properties;
        let dynamic_block = dynamic_properties.0.len() > 0;

        // Config the block actions
        for (trigger, action_set) in block_actions {
            let trigger_id = trigger.get_id();
            let action_ids = action_set.get_ids();

            self.triggers_map.insert(trigger_id, trigger);
            action_set
                .into_iter()
                .zip(action_ids.clone())
                .for_each(|(action, id)| {
                    self.actions_map.insert(id, action);
                });
            if let Some(block_actions) = self.block_id_to_block_actions.get_mut(&id) {
                block_actions.push((trigger_id, action_ids));
            } else {
                self.block_id_to_block_actions
                    .insert(id, vec![(trigger_id, action_ids)]);
            }
        }

        // Init the static properties
        let mut block_entity = world.spawn((BlockComponent, static_properties));
        if dynamic_block {
            block_entity.insert(DynamicBlock);
        } else {
            block_entity.insert(StaticBlock);
        }
        if let Some(block_actions) = self.block_id_to_block_actions.remove(&id) {
            block_entity.insert(BlockActions(block_actions));
        }
        let block_entity_id = block_entity.id();

        self.block_id_to_ent.0.insert(id, block_entity_id);

        self.block_id_counter += 1;
        self
    }
}

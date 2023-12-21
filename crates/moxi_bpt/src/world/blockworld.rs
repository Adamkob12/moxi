use crate::blockreg::meshreg::MeshReg;
use crate::prelude::Trigger;
use crate::*;
use action::{Action, IntoActionSet};
use bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell;
use lazy_static::lazy_static;
use moxi_utils::prelude::BlockId;
use prelude::{Block, BlockRegistry, CommonActionSet, IntoTrigger};
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    pub(crate) static ref NAME_2_ID: Mutex<HashMap<&'static str, BlockId>> =
        Mutex::new(HashMap::new());
    pub(crate) static ref ID_2_NAME: Mutex<HashMap<BlockId, &'static str>> =
        Mutex::new(HashMap::new());
    pub static ref BLOCKS_GLOBAL: () = ();
}

impl BLOCKS_GLOBAL {
    pub fn get_id(name: &'static str) -> Option<BlockId> {
        let name_2_id = NAME_2_ID.lock().unwrap();
        name_2_id.get(name).copied()
    }

    pub fn get_name(id: BlockId) -> Option<&'static str> {
        let id_2_name = ID_2_NAME.lock().unwrap();
        id_2_name.get(&id).copied()
    }

    pub fn id(name: &'static str) -> BlockId {
        Self::get_id(name).unwrap_or(0)
    }

    pub fn name(id: BlockId) -> &'static str {
        Self::get_name(id).unwrap_or("Air")
    }
}

type TriggerId = TypeId;
type ActionId = TypeId;

#[derive(Component)]
pub(crate) struct BlockActions(pub Vec<(Entity, Vec<Entity>)>);

#[derive(Component)]
pub(crate) struct BlockMarker(pub BlockId);

#[derive(Component)]
pub(crate) struct BlockName(pub &'static str);

#[derive(Resource, Default)]
pub(crate) struct BlockIdtoEnt(pub HashMap<BlockId, Entity>);

#[derive(Resource, Default)]
pub(crate) struct TriggersMap(HashMap<TriggerId, Entity>);

#[derive(Resource, Default)]
pub(crate) struct ActionsMap(HashMap<ActionId, Entity>);

pub trait BlockInitiallizerTrait {
    fn init_block<'w, B: Block>(&'w mut self) -> BlockWorldMut<'w>;
}

impl BlockActions {
    pub fn execute_all(&self, world: &mut World, input: Option<BlockWorldUpdateEvent>) {
        let world = world.as_unsafe_world_cell();
        for (trigger_entity, action_entities) in self.0.iter() {
            unsafe {
                let mut trigger = world
                    .world_mut()
                    .get_mut::<Trigger>(*trigger_entity)
                    .unwrap();

                if !trigger.evaluate_unsafe(input, world) {
                    continue;
                }

                for action_entity in action_entities.iter() {
                    let mut action = world.world_mut().get_mut::<Action>(*action_entity).unwrap();
                    action.run_unsafe(input, world);
                }
            }
        }
    }

    pub fn execute_all_unsafe(&self, world: UnsafeWorldCell, input: Option<BlockWorldUpdateEvent>) {
        for (trigger_entity, action_entities) in self.0.iter() {
            unsafe {
                let mut trigger = world
                    .world_mut()
                    .get_mut::<Trigger>(*trigger_entity)
                    .unwrap();

                if !trigger.evaluate_unsafe(input, world) {
                    continue;
                }

                for action_entity in action_entities.iter() {
                    let mut action = world.world_mut().get_mut::<Action>(*action_entity).unwrap();
                    action.run_unsafe(input, world);
                }
            }
        }
    }
}

pub struct BlockWorldMut<'w> {
    block_world_mut: EntityWorldMut<'w>,
    unsafe_world_cell: UnsafeWorldCell<'w>,
}

impl<'w> BlockWorldMut<'w> {
    pub unsafe fn world_mut(&'w mut self) -> &'w mut World {
        self.block_world_mut.world_mut()
    }

    pub fn init_block<B: Block>(&'w mut self) -> BlockWorldMut<'w> {
        unsafe { self.world_mut().init_block::<B>() }
    }

    pub fn id(&self) -> Entity {
        self.block_world_mut.id()
    }

    pub fn with_static_properties<B: Bundle>(
        &'w mut self,
        static_properties: B,
    ) -> &mut BlockWorldMut<'w> {
        self.block_world_mut.insert(static_properties);
        self
    }

    pub fn with_block_actions<I, M1, M2, M3>(
        &'w mut self,
        into_trigger: impl IntoTrigger<I, M1>,
        no_input_actions: impl IntoActionSet<(), M2>,
        input_actions: impl IntoActionSet<BlockWorldUpdateEvent, M3>,
    ) -> &mut BlockWorldMut<'w> {
        let trigger = into_trigger.into_trigger();
        let mut action_set = no_input_actions.into_action_set();

        action_set.extend(input_actions.into_action_set());

        let trigger_id = trigger.get_id();

        let (trigger_entity, action_entities) = unsafe {
            let world = self.unsafe_world_cell.world_mut();

            let trigger_entity: Entity = {
                let trigger_map = world.resource_mut::<TriggersMap>();
                if let Some(trigger_ent) = trigger_map.0.get(&trigger_id) {
                    *trigger_ent
                } else {
                    world.spawn(trigger).id()
                }
            };
            let mut trigger_map = world.resource_mut::<TriggersMap>();
            trigger_map.0.insert(trigger_id, trigger_entity);

            let action_entities: Vec<Entity> = {
                let mut action_entities = Vec::new();
                for (action_id, action) in action_set.enumerate_ids_and_actions().into_iter() {
                    let action_entity: Entity = {
                        let action_map = world.resource_mut::<ActionsMap>();
                        if let Some(action_ent) = action_map.0.get(&action_id) {
                            *action_ent
                        } else {
                            world.spawn(action).id()
                        }
                    };
                    let mut action_map = world.resource_mut::<ActionsMap>();
                    action_map.0.insert(action_id, action_entity);
                    action_entities.push(action_entity);
                }
                action_entities
            };

            (trigger_entity, action_entities)
        };
        self.block_world_mut
            .insert(BlockActions(vec![(trigger_entity, action_entities)]));
        self
    }
}

impl BlockInitiallizerTrait for World {
    fn init_block<'w, B: Block>(&'w mut self) -> BlockWorldMut<'w> {
        let block_name = B::get_name();
        let block_id = self
            .get_resource::<BlockRegistry>()
            .map_or(0, |reg| reg.names.len()) as BlockId;
        if block_id == 0 {
            self.init_resource::<BlockRegistry>();
            self.init_resource::<MeshReg>();
            self.init_resource::<BlockIdtoEnt>();
            self.init_resource::<ActionsMap>();
            self.init_resource::<TriggersMap>();
        }

        assert!(
            self.resource_mut::<BlockRegistry>()
                .names
                .insert(block_name),
            "Block name already exists"
        );

        NAME_2_ID.lock().unwrap().insert(block_name, block_id);
        ID_2_NAME.lock().unwrap().insert(block_id, block_name);

        self.resource_mut::<MeshReg>().meshes.push(B::get_mesh());

        unsafe {
            let tmp_mut_ptr = self as *mut World;
            let block_world_mut = BlockWorldMut {
                block_world_mut: self.spawn((BlockMarker(block_id), BlockName(block_name))),
                unsafe_world_cell: tmp_mut_ptr.as_mut().unwrap().as_unsafe_world_cell(),
            };
            let block_entity = block_world_mut.id();

            tmp_mut_ptr
                .as_mut()
                .unwrap()
                .resource_mut::<BlockIdtoEnt>()
                .0
                .insert(block_id, block_entity);
            block_world_mut
        }
    }
}

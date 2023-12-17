use crate::blockreg::meshreg::MeshReg;
use crate::prelude::Trigger;
use crate::*;
use action::{Action, IntoActionSet};
use bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell;
use moxi_utils::prelude::BlockId;
use prelude::{Block, BlockRegistry, CommonActionSet, IntoTrigger};
use std::any::TypeId;
use std::collections::HashMap;

type TriggerId = TypeId;
type ActionId = TypeId;

#[derive(Component)]
pub(crate) struct BlockActions(pub Vec<(Entity, Vec<Entity>)>);

#[derive(Component)]
pub(crate) struct BlockMarker(pub BlockId);

#[derive(Resource, Default)]
pub(crate) struct BlockIdtoEnt(HashMap<BlockId, Entity>);

#[derive(Resource, Default)]
pub(crate) struct TriggersMap(HashMap<TriggerId, Entity>);

#[derive(Resource, Default)]
pub(crate) struct ActionsMap(HashMap<ActionId, Entity>);

pub struct BlockInitiallizer<'w> {
    world: &'w mut World,
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
}

pub struct BlockWorldMut<'w> {
    block_world_mut: EntityWorldMut<'w>,
    unsafe_world_cell: UnsafeWorldCell<'w>,
}

impl<'w> BlockWorldMut<'w> {
    pub unsafe fn world_mut(&'w mut self) -> &'w mut World {
        self.block_world_mut.world_mut()
    }

    pub fn id(&self) -> Entity {
        self.block_world_mut.id()
    }

    pub fn add_static_properties<B: Bundle>(
        &'w mut self,
        static_properties: B,
    ) -> &mut BlockWorldMut<'w> {
        self.block_world_mut.insert(static_properties);
        self
    }

    pub fn add_block_actions<I, M, M2, M3>(
        &'w mut self,
        into_trigger: impl IntoTrigger<I, M>,
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

impl<'w> BlockInitiallizer<'w> {
    pub fn new(world: &'w mut World) -> Self {
        Self { world }
    }
    pub fn init_block<B: Block>(&'w mut self) -> BlockWorldMut<'w> {
        let block_name = B::get_name();
        let block_id = self
            .world
            .get_resource::<BlockRegistry>()
            .map_or(0, |reg| reg.0.len()) as BlockId;
        if block_id == 0 {
            self.world.init_resource::<BlockRegistry>();
            self.world.init_resource::<MeshReg>();
            self.world.init_resource::<BlockIdtoEnt>();
            self.world.init_resource::<ActionsMap>();
            self.world.init_resource::<TriggersMap>();
        }

        self.world
            .resource_mut::<BlockRegistry>()
            .0
            .insert(block_name, block_id);
        self.world
            .resource_mut::<MeshReg>()
            .meshes
            .push(B::get_mesh());

        unsafe {
            let tmp_mut_ptr = self.world as *mut World;
            let block_world_mut = BlockWorldMut {
                block_world_mut: self.world.spawn(BlockMarker(block_id)),
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

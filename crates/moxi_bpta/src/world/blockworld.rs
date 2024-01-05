use crate::blockreg::meshreg::MeshReg;
use crate::prelude::Trigger;
use crate::*;
use action::{Action, IntoActionSet};
use bevy_asset::{Assets, Handle};
use bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell;
use bevy_render::mesh::Mesh;
use chunk::components::ToUpdate;
use chunk::meshmd::ChunkMeshMd;
use lazy_static::lazy_static;
use moxi_mesh_utils::prelude::{BlockMeshType, MeshRegistry};
use moxi_mesh_utils::BlockMeshChange;
use moxi_utils::prelude::{
    is_block_pos_on_edge, neighbor_across_chunk, to_cords, BlockId, BlockPos, ChunkCords,
    Dimensions, NDir, SurroundingBlocks, SurroundingBlocksCommon, FACES,
};
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
pub static mut PLACEHOLDER_DIMS: Dimensions = Dimensions::new(16, 16, 16);

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

#[derive(Component, Default)]
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

#[derive(Event)]
pub struct GlobalBlockPlace {
    pub block_id: BlockId,
    pub block_pos: BlockPos,
    pub chunk_cords: ChunkCords,
}

#[derive(Event)]
pub struct GlobalBlockBreak {
    pub block_pos: BlockPos,
    pub chunk_cords: ChunkCords,
    pub block_id: BlockId,
}

pub trait BlockInitiallizerTrait {
    fn init_block<'w, B: Block>(&'w mut self) -> BlockWorldMut<'w>;
}

impl BlockActions {
    pub fn _execute_all(&self, world: &mut World, input: Option<BlockWorldUpdateEvent>) {
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

    pub unsafe fn apply_deferred_all(&self, world: UnsafeWorldCell) {
        for (_, action_entities) in self.0.iter() {
            for action_entity in action_entities.iter() {
                let mut action = world.world_mut().get_mut::<Action>(*action_entity).unwrap();
                action.apply_deferred_unsafe(world);
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
        if let Some(mut block_actions_comp) = self.block_world_mut.get_mut::<BlockActions>() {
            block_actions_comp.0.push((trigger_entity, action_entities));
        } else {
            self.block_world_mut
                .insert(BlockActions(vec![(trigger_entity, action_entities)]));
        }
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

        let block_mesh = B::get_mesh();
        let handle = block_mesh
            .clone()
            .as_option()
            .map_or(Handle::default(), |mesh| {
                self.resource_mut::<Assets<Mesh>>().add(mesh)
            });
        let mut mesh_reg = self.resource_mut::<MeshReg>();
        mesh_reg.meshes.push(block_mesh);
        mesh_reg.handles.push(handle);

        unsafe {
            let tmp_mut_ptr = self as *mut World;
            let block_world_mut = BlockWorldMut {
                block_world_mut: self.spawn((
                    BlockMarker(block_id),
                    BlockName(block_name),
                    BlockActions::default(),
                )),
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

pub(crate) fn global_block_placer<const N: usize>(
    mut block_place_events: EventReader<GlobalBlockPlace>,
    mut blocks: _Blocks<N>,
    mut commands: Commands,
    mut chunk_meshes_query: Query<&mut ChunkMeshMd>,
    mesh_registry: Res<MeshReg>,
    mut block_world_update_sender: EventWriter<BlockWorldUpdateEvent>,
) {
    for event in block_place_events.read() {
        let GlobalBlockPlace {
            block_id,
            block_pos,
            chunk_cords,
        } = *event;

        let surrounding_blocks = blocks.get_global_surrounding_blocks(chunk_cords, block_pos);
        let chunk_entity = blocks.chunk_map.get_chunk(chunk_cords).unwrap();
        let mut chunk_grid = blocks.chunks_query.get_mut(chunk_entity).unwrap();
        let _ = chunk_grid.0.set_block(block_id, block_pos);
        let mesh_type = mesh_registry.get_block_mesh_type(&block_id);
        let chunk_mesh_entity = chunk_grid.1.get_from_type(mesh_type.into());
        let mut chunk_mesh_md = chunk_meshes_query.get_mut(chunk_mesh_entity).unwrap();
        commands.entity(chunk_mesh_entity).insert(ToUpdate);

        chunk_mesh_md.log_block_add(
            block_pos,
            block_id,
            surrounding_blocks.map(|x| x.map(|(_, _, _, id)| id)),
        );

        block_world_update_sender.send(BlockWorldUpdateEvent {
            block_pos,
            chunk_cords,
            block_update: BlockUpdate::Pure(BLOCK_PLACED),
        });
    }
}

pub(crate) fn global_block_breaker<const N: usize>(
    mut block_break_events: EventReader<GlobalBlockBreak>,
    mut blocks: _Blocks<N>,
    mesh_registry: Res<MeshReg>,
    mut block_world_update_sender: EventWriter<BlockWorldUpdateEvent>,
    mut commands: Commands,
    mut chunk_meshes_query: Query<&mut ChunkMeshMd>,
) {
    for event in block_break_events.read() {
        let GlobalBlockBreak {
            block_pos,
            chunk_cords,
            block_id,
        } = *event;

        let surrounding_blocks = blocks.get_global_surrounding_blocks(chunk_cords, block_pos);
        let chunk_entity = blocks.chunk_map.get_chunk(chunk_cords).unwrap();
        let mut chunk_grid = blocks.chunks_query.get_mut(chunk_entity).unwrap();
        let dims = chunk_grid.0.dims;
        let _ = chunk_grid.0.set_block(0, block_pos);
        let mesh_type = mesh_registry.get_block_mesh_type(&block_id);
        let chunk_mesh_entity = chunk_grid.1.get_from_type(mesh_type.into());
        let mut chunk_mesh_md = chunk_meshes_query.get_mut(chunk_mesh_entity).unwrap();
        commands.entity(chunk_mesh_entity).insert(ToUpdate);

        //

        chunk_mesh_md.log_block_break(
            block_pos,
            block_id,
            surrounding_blocks.map(|x| x.map(|(_, _, _, id)| id)),
        );

        if matches!(mesh_type, BlockMeshType::Cube) {
            for face in FACES.into_iter().filter(|face| !face.is_vertical()) {
                if is_block_pos_on_edge(block_pos, face, dims) {
                    let adj_chunk_cords = chunk_cords + to_cords(Some(NDir::from(face)));
                    let neighbor_block_pos = neighbor_across_chunk(block_pos, face, dims).unwrap();
                    let adj_chunk_entity = blocks.chunk_map.get_chunk(adj_chunk_cords).unwrap();
                    let adj_chunk_grid = blocks.chunks_query.get(adj_chunk_entity).unwrap();
                    let neighbor_block = adj_chunk_grid.0.get_block_or(neighbor_block_pos, 0);
                    let adj_mesh_type = mesh_registry.get_block_mesh_type(&neighbor_block);
                    if matches!(adj_mesh_type, BlockMeshType::Cube) {
                        let adj_chunk_mesh_entity =
                            adj_chunk_grid.1.get_from_type(adj_mesh_type.into());
                        let mut adj_chunk_mesh_md =
                            chunk_meshes_query.get_mut(adj_chunk_mesh_entity).unwrap();
                        adj_chunk_mesh_md.update_block(
                            BlockMeshChange::AddFaces,
                            neighbor_block_pos,
                            neighbor_block,
                            SurroundingBlocks::<BlockId>::uniform(neighbor_block)
                                .with_face(face.opposite(), 0),
                        );
                        commands.entity(adj_chunk_mesh_entity).insert(ToUpdate);
                    }
                }
            }
        }

        //

        block_world_update_sender.send(BlockWorldUpdateEvent {
            block_pos,
            chunk_cords,
            block_update: BlockUpdate::Pure(BLOCK_REMOVED),
        });
    }
}

use crate::*;
use bevy_ecs::{system::SystemParam, world::unsafe_world_cell::UnsafeWorldCell};
use blockworld::{BlockActions, BlockIdtoEnt};
use moxi_utils::prelude::BlockId;

#[derive(SystemParam)]
pub(crate) struct BlockActionRunner<'w, 's> {
    unsafe_world_cell: WrapperWorldCell<'w>,
    block_actions: Query<'w, 's, &'static BlockActions>,
    block_id_to_ent: Res<'w, BlockIdtoEnt>,
}

impl<'w, 's> BlockActionRunner<'w, 's> {
    pub fn execute_all_block_actions(
        &mut self,
        block_id: BlockId,
        block_world_update_event: BlockWorldUpdateEvent,
    ) {
        let block_entity = self.block_id_to_ent.0.get(&block_id).unwrap();
        let block_actions = self.block_actions.get(*block_entity).unwrap();
        block_actions.execute_all_unsafe(self.unsafe_world_cell.0, Some(block_world_update_event));
    }

    pub unsafe fn apply_deferred(&mut self) {
        for block_actions in self.block_actions.iter() {
            block_actions.apply_deferred_all(self.unsafe_world_cell.0);
        }
    }
}

pub fn apply_deferred_for_all_actions(mut block_action_runner: BlockActionRunner) {
    unsafe {
        block_action_runner.apply_deferred();
    }
}

pub struct WrapperWorldCell<'w>(UnsafeWorldCell<'w>);

unsafe impl SystemParam for WrapperWorldCell<'_> {
    type State = ();
    type Item<'w, 's> = WrapperWorldCell<'w>;

    fn init_state(
        _world: &mut World,
        _system_meta: &mut bevy_ecs::system::SystemMeta,
    ) -> Self::State {
        ()
    }

    unsafe fn get_param<'world, 'state>(
        _state: &'state mut Self::State,
        _system_meta: &bevy_ecs::system::SystemMeta,
        world: UnsafeWorldCell<'world>,
        _change_tick: bevy_ecs::component::Tick,
    ) -> Self::Item<'world, 'state> {
        WrapperWorldCell(world)
    }
}

#[derive(Event, Clone, Copy)]
pub(crate) struct InBetweenerEvent(pub(crate) BlockWorldUpdateEvent);

pub(crate) fn handle_world_block_update<const N: usize>(
    mut world_block_update_events: EventReader<BlockWorldUpdateEvent>,
    mut inbetweener_event_sender: EventWriter<InBetweenerEvent>,
    mut block_action_runner: BlockActionRunner,
    blocks: _Blocks<N>,
) {
    for event in world_block_update_events.read() {
        let block_id_at_pos = blocks.block_id_at(event.chunk_cords(), event.block_pos());
        block_action_runner.execute_all_block_actions(block_id_at_pos, *event);
        inbetweener_event_sender.send(InBetweenerEvent(*event));
    }
}

pub(crate) fn send_world_block_updates_to_surrounding_blocks<const N: usize>(
    mut world_block_update_event_sender: EventWriter<BlockWorldUpdateEvent>,
    mut inbetweener_events: EventReader<InBetweenerEvent>,
    blocks: _Blocks<N>,
) {
    for event in inbetweener_events.read() {
        let block_update = match event.0.block_update() {
            BlockUpdate::Pure(block_update) => block_update,
            BlockUpdate::Reaction(_, _) => {
                continue;
            }
        };
        let surrounding_blocks =
            blocks.get_global_surrounding_blocks(event.0.chunk_cords(), event.0.block_pos());
        surrounding_blocks
            .iter()
            .filter_map(|x| *x)
            .for_each(|(face, cc, bp, _)| {
                world_block_update_event_sender.send(BlockWorldUpdateEvent {
                    block_pos: bp,
                    chunk_cords: cc,
                    block_update: BlockUpdate::Reaction(face.opposite(), block_update),
                })
            });
    }
}

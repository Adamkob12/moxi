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

pub(crate) fn handle_world_block_update<const N: usize>(
    mut world_block_update_events: EventReader<BlockWorldUpdateEvent>,
    mut block_action_runner: BlockActionRunner,
) {
    for event in world_block_update_events.read() {
        block_action_runner.execute_all_block_actions(event.block_id, *event);
    }
}

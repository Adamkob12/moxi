pub mod app;
mod systems;

use self::{
    app::MoxiApp,
    systems::{
        apply_deferred_for_all_actions, handle_world_block_update,
        send_world_block_updates_to_surrounding_blocks, InBetweenerEvent,
    },
};
use crate::*;
use bevy_app::{Plugin, PreUpdate};
use blockworld::{global_block_breaker, global_block_placer, GlobalBlockBreak, GlobalBlockPlace};
use chunk::MoxiChunkPlugin;
use prelude::Block;

pub struct _MoxiBptPlugin<const N: usize>;

pub struct Air;

pub const RENDER_DISTANCE: i32 = 10;

impl Block for Air {
    fn get_name() -> &'static str {
        "Air"
    }
}

impl<const N: usize> Plugin for _MoxiBptPlugin<N> {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_plugins(MoxiChunkPlugin::<N>::default());
        app.add_event::<BlockWorldUpdateEvent>()
            .add_event::<GlobalBlockBreak>()
            .add_event::<GlobalBlockPlace>()
            .add_event::<InBetweenerEvent>();

        app.init_block::<Air>();
        app.add_systems(
            PreUpdate,
            (
                global_block_breaker::<N>,
                global_block_placer::<N>,
                handle_world_block_update::<N>,
                send_world_block_updates_to_surrounding_blocks::<N>,
                apply_deferred_for_all_actions,
                apply_deferred,
            )
                .chain(),
        );
    }
}

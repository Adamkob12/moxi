pub mod app;
mod systems;

use self::{app::MoxiApp, systems::handle_world_block_update};
use crate::*;
pub use app::*;
use bevy_app::{Plugin, Update};
use blockworld::{global_block_breaker, global_block_placer, GlobalBlockBreak, GlobalBlockPlace};
use chunk::ChunkPlugin;
use prelude::Block;

pub struct MoxiBptPlugin<const N: usize>;

pub struct Air;

pub const RENDER_DISTANCE: i32 = 10;

impl Block for Air {
    fn get_name() -> &'static str {
        "Air"
    }
}

impl<const N: usize> Plugin for MoxiBptPlugin<N> {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_plugins(ChunkPlugin::<N>);
        app.add_event::<BlockWorldUpdateEvent>()
            .add_event::<GlobalBlockBreak>()
            .add_event::<GlobalBlockPlace>();

        app.init_block::<Air>();
        app.add_systems(
            Update,
            (
                handle_world_block_update::<N>,
                global_block_breaker::<N>,
                global_block_placer::<N>,
            )
                .chain(),
        );
    }
}

mod app;
mod systems;

use self::app::MoxiApp;
use crate::*;
use bevy_app::Plugin;
use prelude::Block;
use world::block_commands::BlockCommands;

pub struct MoxiBptPlugin;

pub struct Air;

pub const RENDER_DISTANCE: i32 = 10;

impl Block for Air {
    fn get_name() -> &'static str {
        "Air"
    }
}

impl Plugin for MoxiBptPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_event::<BlockWorldUpdateEvent>();
        app.init_resource::<BlockCommands>();
        app.init_block::<Air>();
    }
}

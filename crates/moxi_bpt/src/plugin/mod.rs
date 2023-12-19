mod app;
mod systems;

use crate::*;
use bevy_app::Plugin;
use world::block_commands::BlockCommands;

pub struct MoxiBptPlugin;

impl Plugin for MoxiBptPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_event::<BlockWorldUpdateEvent>();
        app.init_resource::<BlockCommands>();
    }
}

mod blocks;
mod chunks;
mod player;

use bevy::{prelude::*, window::WindowResolution};
use bevy_xpbd_3d::prelude::PhysicsPlugins;
pub use blocks::*;
use chunks::ChunksPlugin;
use moxi::prelude::*;
use moxi_physics::config_physics_from_dimensions;
use player::PlayerPlugin;

pub(crate) const HEIGHT: u32 = 16;
pub(crate) const WIDTH: u32 = 16;
pub(crate) const LENGTH: u32 = 16;
pub(crate) const CHUNK_DIMS: Dimensions = Dimensions::new(WIDTH, HEIGHT, LENGTH);

config_from_dimensions!(CHUNK_DIMS);
// config_physics_from_dimensions!(CHUNK_DIMS);

fn main() -> Result<(), std::io::Error> {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    mode: bevy::window::WindowMode::Windowed,
                    resolution: WindowResolution::new(1400.0, 900.0),

                    ..Default::default()
                }),
                ..Default::default()
            }),
        MoxiBptPlugin::default(),
        // MoxiPhysicsPlugin::default(),
        BlocksPlugin,
        PlayerPlugin,
        ChunksPlugin,
        PhysicsPlugins::default(),
    ));

    app.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.8,
    });

    app.run();

    Ok(())
}

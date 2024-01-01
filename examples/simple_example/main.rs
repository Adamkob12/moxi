mod blocks;
mod chunks;
mod player;

use bevy::{prelude::*, window::WindowResolution};
use bevy_flycam::prelude::NoCameraPlayerPlugin;
pub use bevy_moxi as moxi;
pub use blocks::*;
use chunks::ChunksPlugin;
use moxi::prelude::*;
use player::PlayerPlugin;

pub(crate) const HEIGHT: u32 = 16;
pub(crate) const WIDTH: u32 = 16;
pub(crate) const LENGTH: u32 = 16;
pub(crate) const CHUNK_DIMS: Dimensions = Dimensions::new(WIDTH, HEIGHT, LENGTH);
pub(crate) const BLOCKS_IN_CHUNK: usize =
    CHUNK_DIMS.x as usize * CHUNK_DIMS.y as usize * CHUNK_DIMS.z as usize;

fn main() {
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
        _MoxiBptPlugin::<BLOCKS_IN_CHUNK>,
        BlocksPlugin,
        NoCameraPlayerPlugin,
        PlayerPlugin,
        ChunksPlugin,
    ));

    app.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.8,
    });

    app.run();
}

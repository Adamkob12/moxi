mod chunkbuilder;
use bevy_app::prelude::Plugin;

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut bevy_app::App) {}
}

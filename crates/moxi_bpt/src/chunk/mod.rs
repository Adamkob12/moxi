mod chunkbuilder;
mod components;
mod resources;

use bevy_app::prelude::Plugin;

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.init_resource::<resources::ChunkMap>()
            .init_resource::<resources::ChunkQueue>();
    }
}

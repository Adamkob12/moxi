mod chunkbuilder;
mod components;
mod meshmd;
mod resources;
mod systems;

use bevy_app::prelude::Plugin;
use bevy_asset::Handle;
use bevy_ecs::system::Resource;
use bevy_pbr::StandardMaterial;

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.init_resource::<resources::ChunkMap>()
            .init_resource::<resources::ChunkQueue>();
    }
}

#[derive(Resource)]
pub struct CubeMeshMaterial(pub Handle<StandardMaterial>);

#[derive(Resource)]
pub struct XSpriteMeshMaterial(pub Handle<StandardMaterial>);

#[derive(Resource)]
pub struct CustomMeshMaterial(pub Handle<StandardMaterial>);

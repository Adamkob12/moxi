pub(crate) mod chunkbuilder;
pub(crate) mod components;
pub(crate) mod meshmd;
pub(crate) mod resources;
pub(crate) mod systems;

use self::systems::*;
use bevy_app::{prelude::Plugin, Update};
use bevy_asset::Handle;
use bevy_ecs::{
    prelude::{apply_deferred, resource_changed},
    schedule::IntoSystemConfigs,
    system::Resource,
};
use bevy_pbr::StandardMaterial;

pub use components::{Chunk, MeshChunk};
use moxi_utils::prelude::ChunkCords;
pub use resources::CurrentChunk;

pub struct MoxiChunkPlugin<const N: usize> {
    pub starting_chunk: ChunkCords,
}

impl<const N: usize> Default for MoxiChunkPlugin<N> {
    fn default() -> Self {
        Self {
            starting_chunk: [0, 0].into(),
        }
    }
}

impl<const N: usize> Plugin for MoxiChunkPlugin<N> {
    fn build(&self, app: &mut bevy_app::App) {
        app.init_resource::<resources::ChunkMap>()
            .init_resource::<resources::ChunkQueue>()
            .insert_resource(CurrentChunk(self.starting_chunk));
        app.add_systems(
            Update,
            (
                (
                    (queue_chunks_to_spawn, despawn_chunks)
                        .run_if(resource_changed::<CurrentChunk>()),
                    build_chunks::<N>,
                    spawn_chunks::<N>,
                    handle_chunk_updates,
                    introduce_adj_chunks::<N>,
                ),
                apply_deferred,
            )
                .chain(),
        );
    }
}

#[derive(Resource)]
pub struct CubeMeshMaterial(pub Handle<StandardMaterial>);

#[derive(Resource)]
pub struct XSpriteMeshMaterial(pub Handle<StandardMaterial>);

#[derive(Resource)]
pub struct CustomMeshMaterial(pub Handle<StandardMaterial>);

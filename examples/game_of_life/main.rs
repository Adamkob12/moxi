mod blocks;
mod chunks;
mod player;

use bevy::{prelude::*, render::primitives::Aabb, window::WindowResolution};
use bevy_xpbd_3d::prelude::{
    AsyncCollider, CollisionLayers, ComputedCollider, PhysicsPlugins, RigidBody, TriMeshFlags,
};
pub use blocks::*;
use chunks::ChunksPlugin;
use moxi::prelude::*;
use player::{PlayerPlugin, RigidLayer};

pub(crate) const HEIGHT: u32 = 1;
pub(crate) const WIDTH: u32 = 16;
pub(crate) const LENGTH: u32 = 16;
pub(crate) const CHUNK_DIMS: Dimensions = Dimensions::new(WIDTH, HEIGHT, LENGTH);
pub(crate) const BLOCKS_IN_CHUNK: usize =
    CHUNK_DIMS.x as usize * CHUNK_DIMS.y as usize * CHUNK_DIMS.z as usize;
// use bevy_mod_debugdump::schedule_graph::{settings::Style, Settings};
// use std::path::PathBuf;
// use bevy::ecs::schedule::ScheduleLabel;
//
// #[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
// struct ScheduleDebugGroup;

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
        MoxiBptPlugin::<BLOCKS_IN_CHUNK>,
        BlocksPlugin,
        PlayerPlugin,
        ChunksPlugin,
        PhysicsPlugins::default(),
    ));

    app.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.8,
    });

    app.add_systems(PostUpdate, insert_collider_for_chunks);

    app.run();

    Ok(())
}

// Will be depracated when built in physics is introduced
fn insert_collider_for_chunks(
    mut commands: Commands,
    mesh_chunks_query: Query<Entity, (Changed<Aabb>, With<MeshChunk>)>,
) {
    for mesh_chunk_entity in mesh_chunks_query.iter() {
        commands
            .entity(mesh_chunk_entity)
            .insert(AsyncCollider(ComputedCollider::TriMeshWithFlags(
                TriMeshFlags::MERGE_DUPLICATE_VERTICES,
            )))
            .insert(RigidBody::Static)
            .insert(CollisionLayers::all_masks::<RigidLayer>().add_group(RigidLayer::Ground));
    }
}

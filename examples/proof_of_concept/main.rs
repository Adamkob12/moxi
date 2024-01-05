mod blocks;
mod chunks;
mod player;

use bevy::{prelude::*, window::WindowResolution};
use bevy_xpbd_3d::prelude::{
    AsyncCollider, CollisionLayers, ComputedCollider, PhysicsPlugins, RigidBody, TriMeshFlags,
};

pub use bevy_moxi as moxi;
pub use blocks::*;
use chunks::ChunksPlugin;
use moxi::prelude::*;
use moxi_mesh_utils::prelude::Aabb;
use moxi_physics::MoxiCollisionLayer;
use player::PlayerPlugin;

pub(crate) const HEIGHT: u32 = 40;
pub(crate) const WIDTH: u32 = 12;
pub(crate) const LENGTH: u32 = 12;
pub(crate) const CHUNK_DIMS: Dimensions = Dimensions::new(WIDTH, HEIGHT, LENGTH);

config_from_dimensions!(CHUNK_DIMS);

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
        MoxiBptaPlugin::default(),
        BlocksPlugin,
        PlayerPlugin,
        ChunksPlugin,
        PhysicsPlugins::default(),
    ));

    app.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.8,
    });

    app.add_systems(Update, insert_collider_for_chunks);

    app.run();
}

/// System to insert colliders for chunks, in the future this will likely be abstracted in a
/// seperate physics plugin, but for now it's user defined. Note this is using bevy_xpbd_3d,
/// using the rapier game engine shouldn't be much different.
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
            .insert(
                CollisionLayers::all_masks::<MoxiCollisionLayer>()
                    .add_group(MoxiCollisionLayer::Terrain),
            );
    }
}

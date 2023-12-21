use crate::*;
use bevy::prelude::*;
use bevy_flycam::FlyCam;
use moxi_bpt::prelude::CurrentChunk;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
        app.add_systems(Update, update_current_chunk);
    }
}

fn setup_player(mut commands: Commands) {
    commands.spawn(FlyCam).insert(Camera3dBundle {
        transform: Transform::from_xyz(0.0, HEIGHT as f32 + 5.0, 0.0),
        ..Default::default()
    });
}

fn update_current_chunk(
    player_query: Query<&Transform, With<FlyCam>>,
    mut current_chunk: ResMut<CurrentChunk>,
) {
    let player_transform = player_query.single();
    let player_position = player_transform.translation;
    let player_chunk = point_to_chunk_cords(player_position, CHUNK_DIMS);
    current_chunk.set(player_chunk);
}

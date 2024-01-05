//! This example shows how to configure the plugin with a custom chunk size.

use std::sync::Arc;

use bevy::prelude::*;
pub use bevy_moxi as moxi; // Import moxi, the alias is optional
use moxi::prelude::*;

pub const CHUNK_DIMS: Dimensions = Dimensions::new(16, 16, 16); // The dimensions of a chunk

config_from_dimensions!(CHUNK_DIMS); // This macro configures a lot of types that take
                                     // a generic const. The macro is optional but highly recommended.
fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, MoxiBptaPlugin::default())); // add the plugin
    app.insert_resource(BoxedBuilder {
        // The mandatory resource that defines how to build chunks.
        builder: Arc::new(MyChunkBuilder { sea_level: 8 }),
    });
    app.add_systems(Startup, setup_texture);
    app.run();
}

// Define an object that's responsible for building chunks. This is required but not enforced statically.
// The game will panic if The `BoxedBuilder` resource is not in the world.
pub struct MyChunkBuilder {
    sea_level: u32,
}

// Implement the `ChunkBuilder` trait for the object. This defines how to build the chunk.
// We want the chunk to be flat, the top layer is grass, the next 3 layers are dirt, the rest is stone.
impl ChunkBuilder<BLOCKS_IN_CHUNK, BlockId> for MyChunkBuilder {
    fn build_chunk(&self, _chunk_cords: ChunkCords) -> Grid<BlockId, BLOCKS_IN_CHUNK> {
        let mut grid = [0; BLOCKS_IN_CHUNK];
        grid = grid
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let block_pos = index_to_pos(i, CHUNK_DIMS).unwrap();
                if block_pos.y == self.sea_level {
                    get_block_id!("Grass").unwrap()
                } else if block_pos.y < self.sea_level - 3 {
                    get_block_id!("Stone").unwrap()
                } else if block_pos.y < self.sea_level {
                    get_block_id!("Dirt").unwrap()
                } else {
                    get_block_id!("Air").unwrap()
                }
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Grid::new(grid, CHUNK_DIMS)
    }
}

// System to setup the texture for the blocks. This is required but not enforced statically. The
// game will panic if `CubeMeshMaterial`, `XSpriteMeshMaterial` or `CustomMeshMaterial` are not in
// the world.
fn setup_texture(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle: Handle<Image> = asset_server.load("blocks.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        ..Default::default()
    });
    commands.insert_resource(CubeMeshMaterial(material_handle.clone()));

    commands.insert_resource(XSpriteMeshMaterial(material_handle.clone()));

    commands.insert_resource(CustomMeshMaterial(material_handle));
}

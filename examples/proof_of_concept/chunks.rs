use std::sync::Arc;

use super::*;

pub struct ChunksPlugin;

impl Plugin for ChunksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_texture);
        app.insert_resource(BoxedBuilder {
            builder: Arc::new(FlatChunkBuilder { sea_level: 8 }),
        });
    }
}

/// Setup the texture, similiar to the `chunks` example.
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

pub struct FlatChunkBuilder {
    sea_level: u32,
}

impl ChunkBuilder<BLOCKS_IN_CHUNK, BlockId> for FlatChunkBuilder {
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

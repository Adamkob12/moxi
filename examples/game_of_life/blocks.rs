use crate::{
    player::{PhysicalPlayer, RigidLayer},
    CHUNK_DIMS,
};

use super::BLOCKS_IN_CHUNK;
use bevy::{prelude::*, utils::HashSet};
use bevy_xpbd_3d::prelude::{
    CoefficientCombine, Collider, CollisionLayers, Friction, GravityScale, LinearVelocity,
    LockedAxes, MassPropertiesBundle, Restitution, RigidBody, ShapeCaster, ShapeHits,
    SpatialQueryFilter,
};
use moxi_bpt::prelude::{app::MoxiApp, *};
use moxi_mesh_utils::prelude::*;
use moxi_utils::prelude::{
    global_block_pos_to_block_trans, global_neighbor, point_to_global_block_pos, BlockGlobalPos,
    BlockPos, ChunkCords, Face,
};

pub type BlocksX<'w, 's> = Blocks<'w, 's, BLOCKS_IN_CHUNK>;
pub type BlocksMutX<'w, 's> = BlocksMut<'w, 's, BLOCKS_IN_CHUNK>;

const VOXEL_DIMS: [f32; 3] = [1.0, 1.0, 1.0];
const TEXTURE_ATLAS_DIMS: [u32; 2] = [10, 10];
const VOXEL_CENTER: [f32; 3] = [0.0, 0.0, 0.0];
const PADDING: f32 = 1.0 / 16.0;
const DEFAULT_COLOR_INTENSITY: f32 = 1.0;
const ALPHA: f32 = 1.0;

pub struct BlocksPlugin;

#[derive(Resource)]
pub struct Pause(pub bool);

impl Plugin for BlocksPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Pause(true));

        app.init_block::<Dead>()
            .with_block_actions(trigger_birth, (), birth)
            .init_block::<Alive>()
            .with_block_actions(trigger_death, (), death);

        app.add_systems(Update, (update_all,).run_if(resource_changed::<Pause>()));
    }
}

const DHNIU: BlockUpdateType = BlockUpdateType::from_u128(572389);

fn update_all(
    blocks: BlocksX,
    chunks: Query<&Chunk>,
    mut update_sender: EventWriter<BlockWorldUpdateEvent>,
) {
    for Chunk { cords } in chunks.iter() {
        let chunk_grid = blocks.get_chunk_grid(*cords);
        if let Some(chunk_grid) = chunk_grid {
            chunk_grid.enumerate_blocks().for_each(|(pos, _)| {
                update_sender.send(BlockWorldUpdateEvent {
                    block_pos: pos,
                    chunk_cords: *cords,
                    block_update: BlockUpdate::Pure(DHNIU),
                });
            });
        }
    }
}

fn trigger_death(block_update: In<BlockWorldUpdateEvent>, blocks: BlocksX) -> bool {
    let block_pos = block_update.0.block_pos;
    let chunk_cords = block_update.0.chunk_cords;

    let tmp = get_neighbors_count(block_pos, chunk_cords, blocks);
    println!("{} to kill", tmp);
    !(tmp < 4 && tmp > 1)
}

fn trigger_birth(block_update: In<BlockWorldUpdateEvent>, blocks: BlocksX) -> bool {
    let block_pos = block_update.0.block_pos;
    let chunk_cords = block_update.0.chunk_cords;

    let tmp = get_neighbors_count(block_pos, chunk_cords, blocks);
    println!("{} to give birth", tmp);
    tmp == 3
}

fn death(block_update: In<BlockWorldUpdateEvent>, mut blocks: BlocksMutX, pause: Res<Pause>) {
    if pause.0 {
        return;
    }
    let block_pos = block_update.0.block_pos;
    let chunk_cords = block_update.0.chunk_cords;

    blocks.set_block_at_name(chunk_cords, block_pos, "Dead");
}

fn birth(block_update: In<BlockWorldUpdateEvent>, mut blocks: BlocksMutX, pause: Res<Pause>) {
    if pause.0 {
        return;
    }
    let block_pos = block_update.0.block_pos;
    let chunk_cords = block_update.0.chunk_cords;

    blocks.set_block_at_name(chunk_cords, block_pos, "Alive");
}

fn get_neighbors_count(block_pos: BlockPos, chunk_cords: ChunkCords, blocks: BlocksX) -> usize {
    let mut neighbors = 0;
    for neigbhor in blocks.get_global_surrounding_blocks(chunk_cords, block_pos) {
        if let Some((face, chunk_cords, block_pos, block_id)) = neigbhor {
            let global_pos = BlockGlobalPos::new(block_pos, chunk_cords);
            if block_id == BLOCKS_GLOBAL::id("Alive") {
                neighbors += 1;
            }
            match face {
                Face::Front | Face::Back => {
                    for tmp_face in [Face::Right, Face::Left] {
                        let tmp = global_neighbor(global_pos, tmp_face, CHUNK_DIMS);
                        if blocks
                            .get_block_name_at(tmp.cords, tmp.pos)
                            .map_or(false, |name| name == "Alive")
                        {
                            neighbors += 1;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    neighbors
}

struct Dead;

struct Alive;

impl Block for Dead {
    fn get_name() -> &'static str {
        "Dead"
    }

    fn get_mesh() -> BlockMesh {
        generate_cube_mesh(
            VOXEL_DIMS,
            TEXTURE_ATLAS_DIMS,
            CubeTextureCords::uniform([2, 0]),
            VOXEL_CENTER,
            PADDING,
            Some(DEFAULT_COLOR_INTENSITY),
            ALPHA,
        )
    }
}

impl Block for Alive {
    fn get_name() -> &'static str {
        "Alive"
    }

    fn get_mesh() -> BlockMesh {
        generate_cube_mesh(
            VOXEL_DIMS,
            TEXTURE_ATLAS_DIMS,
            CubeTextureCords::uniform([0, 0]),
            VOXEL_CENTER,
            PADDING,
            Some(DEFAULT_COLOR_INTENSITY),
            ALPHA,
        )
    }
}

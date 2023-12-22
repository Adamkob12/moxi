use super::BLOCKS_IN_CHUNK;
use bevy::prelude::{App, In, Plugin};
use moxi_bpt::prelude::{app::MoxiApp, *};
use moxi_mesh_utils::prelude::*;
use moxi_utils::prelude::{BlockPos, Face};

const VOXEL_DIMS: [f32; 3] = [1.0, 1.0, 1.0];
const TEXTURE_ATLAS_DIMS: [u32; 2] = [10, 10];
const VOXEL_CENTER: [f32; 3] = [0.0, 0.0, 0.0];
const PADDING: f32 = 1.0 / 16.0;
const DEFAULT_COLOR_INTENSITY: f32 = 1.0;
const ALPHA: f32 = 1.0;

pub struct BlocksPlugin;

impl Plugin for BlocksPlugin {
    fn build(&self, app: &mut App) {
        app.init_block::<Grass>()
            .with_block_actions(
                trigger_if_block_above_is_not_air,
                (),
                transform_into::<Dirt>,
            )
            .init_block::<Dirt>()
            .init_block::<Stone>();
    }
}

pub type BlocksX<'w, 's> = Blocks<'w, 's, BLOCKS_IN_CHUNK>;
pub type BlocksMutX<'w, 's> = BlocksMut<'w, 's, BLOCKS_IN_CHUNK>;

fn trigger_if_block_above_is_not_air(
    block_world_update: In<BlockWorldUpdateEvent>,
    blocks: BlocksX,
) -> bool {
    let block_pos = block_world_update.0.block_pos;
    let chunk_cords = block_world_update.0.chunk_cords;
    let block_above_pos = block_pos + BlockPos::from([0, 1, 0]);

    blocks.block_name_at(chunk_cords, block_above_pos) != "Air"
}

fn transform_into<B: Block>(block_world_update: In<BlockWorldUpdateEvent>, mut blocks: BlocksMutX) {
    let block_pos = block_world_update.0.block_pos;
    let chunk_cords = block_world_update.0.chunk_cords;

    blocks.set_block_at_name(chunk_cords, block_pos, B::get_name());
}

pub struct Grass;

pub struct Dirt;

pub struct Stone;

impl Block for Grass {
    fn get_name() -> &'static str {
        "Grass"
    }

    fn get_mesh() -> BlockMesh {
        generate_cube_mesh(
            VOXEL_DIMS,
            TEXTURE_ATLAS_DIMS,
            CubeTextureCords::uniform([1, 0])
                .with_face(Face::Top, [0, 0])
                .with_face(Face::Bottom, [2, 0]),
            VOXEL_CENTER,
            PADDING,
            Some(DEFAULT_COLOR_INTENSITY),
            ALPHA,
        )
    }
}

impl Block for Dirt {
    fn get_name() -> &'static str {
        "Dirt"
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

impl Block for Stone {
    fn get_name() -> &'static str {
        "Stone"
    }

    fn get_mesh() -> BlockMesh {
        generate_cube_mesh(
            VOXEL_DIMS,
            TEXTURE_ATLAS_DIMS,
            CubeTextureCords::uniform([3, 0]),
            VOXEL_CENTER,
            PADDING,
            Some(DEFAULT_COLOR_INTENSITY),
            ALPHA,
        )
    }
}

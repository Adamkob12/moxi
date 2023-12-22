use bevy::app::{App, Plugin};
use moxi_bpt::prelude::{app::MoxiApp, *};
use moxi_mesh_utils::prelude::*;
use moxi_utils::prelude::Face;

const VOXEL_DIMS: [f32; 3] = [1.0, 1.0, 1.0];
const TEXTURE_ATLAS_DIMS: [u32; 2] = [10, 10];
const VOXEL_CENTER: [f32; 3] = [0.0, 0.0, 0.0];
const PADDING: f32 = 0.0 / 16.0;
const DEFAULT_COLOR_INTENSITY: f32 = 1.0;
const ALPHA: f32 = 1.0;

pub struct BlocksPlugin;

impl Plugin for BlocksPlugin {
    fn build(&self, app: &mut App) {
        app.init_block::<Grass>()
            .init_block::<Dirt>()
            .init_block::<Stone>();
    }
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

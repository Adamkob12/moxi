use crate::{
    player::{PhysicalPlayer, PlayerCamera, RigidLayer},
    CHUNK_DIMS,
};

use super::BLOCKS_IN_CHUNK;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::{
    CoefficientCombine, Collider, CollisionLayers, Friction, GravityScale, LinearVelocity,
    LockedAxes, MassPropertiesBundle, Restitution, RigidBody, ShapeCaster, ShapeHits,
    SpatialQueryFilter,
};
use moxi_bpt::prelude::{app::MoxiApp, *};
use moxi_mesh_utils::prelude::*;
use moxi_utils::prelude::{
    global_block_pos_to_block_trans, point_to_global_block_pos, BlockGlobalPos, BlockPos, Face,
};

const VOXEL_DIMS: [f32; 3] = [1.0, 1.0, 1.0];
const TEXTURE_ATLAS_DIMS: [u32; 2] = [10, 10];
const VOXEL_CENTER: [f32; 3] = [0.0, 0.0, 0.0];
const PADDING: f32 = 1.0 / 16.0;
const DEFAULT_COLOR_INTENSITY: f32 = 1.0;
const ALPHA: f32 = 1.0;
const PLAYER_STEPPED_ON_BLOCK: BlockUpdateType = BlockUpdateType::from_u128(12);

pub struct BlocksPlugin;

impl Plugin for BlocksPlugin {
    fn build(&self, app: &mut App) {
        app.init_block::<Grass>()
            .with_block_actions(trigger_if_block_above_isnt_air, (), transform_into::<Dirt>)
            .with_block_actions(trigger_player_step_on, (), transform_into::<Stone>)
            .init_block::<Dirt>()
            .init_block::<Stone>()
            .init_block::<Sand>()
            .with_block_actions(trigger_falling_block, (), spawn_falling_block);
        app.add_systems(
            PreUpdate,
            (follow_falling_block, check_if_player_stepped_on_block),
        );
    }
}

pub type BlocksX<'w, 's> = Blocks<'w, 's, BLOCKS_IN_CHUNK>;
pub type BlocksMutX<'w, 's> = BlocksMut<'w, 's, BLOCKS_IN_CHUNK>;

fn trigger_if_block_above_isnt_air(
    block_world_update: In<BlockWorldUpdateEvent>,
    blocks: BlocksX,
) -> bool {
    let block_pos = block_world_update.0.block_pos;
    let chunk_cords = block_world_update.0.chunk_cords;
    let block_above_pos = block_pos + BlockPos::from([0, 1, 0]);

    blocks.block_name_at(chunk_cords, block_above_pos) != "Air"
}

fn trigger_player_step_on(block_world_update: In<BlockWorldUpdateEvent>) -> bool {
    block_world_update
        .0
        .block_update
        .is_pure_and(|block_update| block_update == PLAYER_STEPPED_ON_BLOCK)
}

fn trigger_falling_block(block_world_update: In<BlockWorldUpdateEvent>, blocks: BlocksX) -> bool {
    let block_pos = block_world_update.0.block_pos;
    let chunk_cords = block_world_update.0.chunk_cords;
    let mut block_below_pos = block_pos;
    block_below_pos.y -= 1;
    blocks.block_name_at(chunk_cords, block_below_pos) == "Air"
        && !matches!(
            block_world_update.0.block_update,
            BlockUpdate::Pure(BLOCK_REMOVED)
        )
}

fn spawn_falling_block(
    block_world_update: In<BlockWorldUpdateEvent>,
    mut commands: Commands,
    mut blocks: BlocksMutX,
    mesh_registry: Res<MeshReg>,
    cube_mesh_material: Res<CubeMeshMaterial>,
) {
    let block_pos = block_world_update.0.block_pos;
    let chunk_cords = block_world_update.0.chunk_cords;
    let sand_block_id = BLOCKS_GLOBAL::id("Sand");
    let global_pos = BlockGlobalPos::new(block_pos, chunk_cords);

    blocks.set_block_at_name(chunk_cords, block_pos, "Air");

    commands.spawn(FallingBlockBundle::new(
        global_pos,
        mesh_registry.get_block_mesh_handle(&sand_block_id),
        cube_mesh_material.0.clone(),
    ));
}

fn follow_falling_block(
    mut blocks: BlocksMutX,
    mut commands: Commands,
    query: Query<(Entity, &FallingBlock, &ShapeHits, &Transform)>,
) {
    for (entity, falling_block, hits, &Transform { translation, .. }) in query.iter() {
        if !hits.is_empty() {
            // println!("hit");
            let BlockGlobalPos { pos, cords, valid } =
                point_to_global_block_pos(translation + Vec3::Y * 0.2, CHUNK_DIMS);
            if valid && falling_block.origin != pos {
                blocks.set_block_at_name(cords, pos, "Sand");
                commands.entity(entity).despawn();
            }
        }
    }
}

fn check_if_player_stepped_on_block(
    mut block_world_update_events: EventWriter<BlockWorldUpdateEvent>,
    player_pos: Query<&GlobalTransform, (With<PhysicalPlayer>, Changed<LinearVelocity>)>,
) {
    if let Ok(&global_transform) = player_pos.get_single() {
        let BlockGlobalPos { pos, cords, valid } =
            point_to_global_block_pos(global_transform.translation() - Vec3::Y * 1.3, CHUNK_DIMS);
        if valid {
            println!("sent");
            block_world_update_events.send(BlockWorldUpdateEvent {
                block_pos: pos,
                chunk_cords: cords,
                block_update: BlockUpdate::Pure(PLAYER_STEPPED_ON_BLOCK),
            });
        }
    }
}

fn transform_into<B: Block>(block_world_update: In<BlockWorldUpdateEvent>, mut blocks: BlocksMutX) {
    let block_pos = block_world_update.0.block_pos;
    let chunk_cords = block_world_update.0.chunk_cords;

    blocks.set_block_at_name(chunk_cords, block_pos, B::get_name());
}

#[derive(Component)]
struct FallingBlock {
    origin: BlockPos,
}

#[derive(Bundle)]
pub struct FallingBlockBundle {
    falling_block: FallingBlock,
    gravity_scale: GravityScale,
    mass_properties: MassPropertiesBundle,
    friction: Friction,
    restitution: Restitution,
    collider: Collider,
    rigid_body: RigidBody,
    shape_caster: ShapeCaster,
    collision_layers: CollisionLayers,
    locked_axes: LockedAxes,
    pbr_bundle: PbrBundle,
}

impl FallingBlockBundle {
    pub fn new(
        global_pos: BlockGlobalPos,
        mesh_handle: Handle<Mesh>,
        material_handle: Handle<StandardMaterial>,
    ) -> Self {
        let block_collider = Collider::cuboid(0.97, 0.97, 0.97);
        let mut caster_shape = block_collider.clone();
        caster_shape.set_scale(Vec3::ONE * 0.95, 10);
        Self {
            falling_block: FallingBlock {
                origin: global_pos.pos,
            },
            gravity_scale: GravityScale(2.4),
            mass_properties: MassPropertiesBundle::new_computed(&block_collider, 1.0),
            friction: Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            restitution: Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            collider: block_collider,
            rigid_body: RigidBody::Dynamic,
            shape_caster: ShapeCaster::new(caster_shape, Vec3::ZERO, Quat::IDENTITY, Vec3::NEG_Y)
                .with_max_time_of_impact(0.2)
                .with_query_filter(SpatialQueryFilter::new().with_masks([RigidLayer::Ground])),
            collision_layers: CollisionLayers::new(
                [RigidLayer::FallingBlock],
                [RigidLayer::Ground],
            ),
            locked_axes: LockedAxes::ROTATION_LOCKED
                .lock_translation_x()
                .lock_translation_z(),
            pbr_bundle: PbrBundle {
                mesh: mesh_handle,
                material: material_handle,
                transform: Transform::from_translation(global_block_pos_to_block_trans(
                    global_pos,
                    VOXEL_DIMS.into(),
                    CHUNK_DIMS,
                )),
                ..Default::default()
            },
        }
    }
}

pub struct Sand;

pub struct Grass;

pub struct Dirt;

pub struct Stone;

impl Block for Sand {
    fn get_name() -> &'static str {
        "Sand"
    }

    fn get_mesh() -> BlockMesh {
        generate_cube_mesh(
            VOXEL_DIMS,
            TEXTURE_ATLAS_DIMS,
            CubeTextureCords::uniform([6, 0]),
            VOXEL_CENTER,
            PADDING,
            Some(DEFAULT_COLOR_INTENSITY),
            ALPHA,
        )
    }
}

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

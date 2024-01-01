use crate::{player::PhysicalPlayer, Blocks, BlocksMut, CHUNK_DIMS};
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::{
    CoefficientCombine, Collider, CollisionLayers, Friction, GravityScale, LinearVelocity,
    LockedAxes, MassPropertiesBundle, Restitution, RigidBody, ShapeCaster, ShapeHits,
    SpatialQueryFilter,
};
use moxi_bpt::prelude::{app::MoxiApp, *};
use moxi_mesh_utils::prelude::*;
use moxi_physics::MoxiCollisionLayer;
use moxi_utils::prelude::{
    global_block_pos_to_block_trans, neighbor_pos, point_to_global_block_pos, BlockGlobalPos,
    BlockPos, Face,
};

// Defining some constants for the meshes of the blocks.
/// The dimensions of the voxel, this is used to generate the mesh.
const VOXEL_DIMS: [f32; 3] = [1.0, 1.0, 1.0];
/// The dimensions of the texture atlas, this is for the UV cords of the mesh.
const TEXTURE_ATLAS_DIMS: [u32; 2] = [10, 10];
/// The center of the voxel.
const VOXEL_CENTER: [f32; 3] = [0.0, 0.0, 0.0];
/// The padding for the texture.
const PADDING: f32 = 1.0 / 16.0;
/// The default color intensity for the block mesh.
const DEFAULT_COLOR_INTENSITY: f32 = 1.0;
/// The alpha value for the block mesh.
const ALPHA: f32 = 1.0;
/// The block update type for when a player steps on a block.
const PLAYER_STEPPED_ON_BLOCK: BlockUpdateType = BlockUpdateType::from_u128(0x12222AA);

pub struct BlocksPlugin;

impl Plugin for BlocksPlugin {
    fn build(&self, app: &mut App) {
        app.init_block::<Grass>()
            .with_block_actions(trigger_if_block_above_isnt_air, (), transform_into::<Dirt>)
            .with_block_actions(trigger_if_player_stepped_on, (), transform_into::<Stone>)
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

fn trigger_if_block_above_isnt_air(
    world_update: In<BlockWorldUpdateEvent>,
    blocks: Blocks,
) -> bool {
    neighbor_pos(world_update.0.block_pos(), Face::Top, CHUNK_DIMS).map_or(
        false,
        |block_above_pos| {
            blocks.block_name_at(world_update.0.chunk_cords(), block_above_pos) != "Air"
        },
    )
}

fn trigger_if_player_stepped_on(block_world_update: In<BlockWorldUpdateEvent>) -> bool {
    block_world_update
        .0
        .block_update()
        .is_pure_and(|block_update| block_update == PLAYER_STEPPED_ON_BLOCK)
}

fn spawn_falling_block(
    block_world_update: In<BlockWorldUpdateEvent>,
    mut commands: Commands,
    mut blocks: BlocksMut,
    mesh_registry: Res<MeshReg>,
    cube_mesh_material: Res<CubeMeshMaterial>,
) {
    let world_updte = block_world_update.0;
    let sand_block_id = block_id!("Sand");

    blocks.set_block_at_name(world_updte.chunk_cords(), world_updte.block_pos(), "Air");

    commands.spawn(FallingBlockBundle::new(
        world_updte.global_block_pos(),
        mesh_registry.get_block_mesh_handle(&sand_block_id),
        cube_mesh_material.0.clone(),
    ));
}

fn trigger_falling_block(block_world_update: In<BlockWorldUpdateEvent>, blocks: Blocks) -> bool {
    let block_pos = block_world_update.0.block_pos();
    let chunk_cords = block_world_update.0.chunk_cords();
    let mut block_below_pos = block_pos;
    block_below_pos.y = block_below_pos.y.checked_sub(1).unwrap_or(0);
    blocks.block_name_at(chunk_cords, block_below_pos) == "Air"
        && !matches!(
            block_world_update.0.block_update(),
            BlockUpdate::Pure(BLOCK_REMOVED)
        )
}

fn follow_falling_block(
    mut blocks: BlocksMut,
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
            block_world_update_events.send(BlockWorldUpdateEvent::new(
                pos,
                cords,
                BlockUpdate::Pure(PLAYER_STEPPED_ON_BLOCK),
            ));
        }
    }
}

fn transform_into<B: Block>(block_world_update: In<BlockWorldUpdateEvent>, mut blocks: BlocksMut) {
    let block_pos = block_world_update.0.block_pos();
    let chunk_cords = block_world_update.0.chunk_cords();

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
                .with_query_filter(
                    SpatialQueryFilter::new().with_masks([MoxiCollisionLayer::Terrain]),
                ),
            collision_layers: CollisionLayers::new(
                [MoxiCollisionLayer::FreeBlock],
                [MoxiCollisionLayer::Terrain],
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

mod config_macro;

use bevy_app::Update;
use bevy_ecs::{
    bundle::Bundle,
    component::Component,
    entity::Entity,
    query::{With, Without},
    system::{Commands, Query, Res, Resource},
};
use bevy_xpbd_3d::{
    math::Vector3,
    parry::{bounding_volume::Aabb, na::Isometry3},
    prelude::{Collider, CollisionLayers, PhysicsLayer, RigidBody, TriMeshFlags},
};
use moxi_bpt::prelude::{Chunk, CurrentChunk, MeshReg, StaticBlockQuery, _Blocks};
use moxi_mesh_utils::prelude::{Aabb as BevyAabb, MeshRegistryCommon};
use moxi_utils::prelude::{chunk_distance, BlockPos, ChunkCords};

#[derive(Default)]
pub enum ColliderComputationMethod {
    #[default]
    ByBlockAabb,
    ByBlockMesh,
}

#[derive(Resource, Clone, Copy)]
pub enum ComputeCollidersFor {
    AllChunks,
    CurrentChunk,
    ChunksNearPlayer { distance: u32 },
    None,
}

impl Default for ComputeCollidersFor {
    fn default() -> Self {
        Self::ChunksNearPlayer { distance: 2 }
    }
}

#[derive(Default)]
pub struct _MoxiPhysicsPlugin<const N: usize> {
    pub collider_computation_method: ColliderComputationMethod,
    pub compute_colliders_for: ComputeCollidersFor,
}

#[derive(Component)]
pub struct BlockNonCollidable;

#[derive(Component)]
pub struct AsyncChunkCollider;

#[derive(PhysicsLayer)]
pub enum MoxiCollisionLayer {
    Terrain,
    FreeBlock,
    Player,
    Other,
}

impl<const N: usize> bevy_app::Plugin for _MoxiPhysicsPlugin<N> {
    fn build(&self, app: &mut bevy_app::App) {
        println!("Inserted Physics Plugin");
        app.insert_resource(self.compute_colliders_for);

        match self.collider_computation_method {
            ColliderComputationMethod::ByBlockAabb => {
                app.add_systems(Update, generate_collider_for_chunks_from_block_aabb::<N>);
            }
            ColliderComputationMethod::ByBlockMesh => {
                app.add_systems(Update, generate_collider_for_chunks_from_block_mesh::<N>);
            }
        }

        app.add_systems(Update, insert_async_collider_for_chunks);
    }
}

fn insert_async_collider_for_chunks(
    mut commands: Commands,
    chunks: Query<(&Chunk, Entity), (Without<Collider>, Without<AsyncChunkCollider>)>,
    compute_colliders_for: Res<ComputeCollidersFor>,
    current_chunk: Res<CurrentChunk>,
) {
    fn build_chunk_filter(
        distance: i32,
        current_chunk: ChunkCords,
    ) -> impl Fn(&ChunkCords) -> bool {
        move |chunk_cords: &ChunkCords| chunk_distance(*chunk_cords, current_chunk) <= distance
    }

    let current_chunk = current_chunk.0;
    let chunk_filter = match compute_colliders_for.into_inner() {
        ComputeCollidersFor::AllChunks => build_chunk_filter(i32::MAX, current_chunk),
        ComputeCollidersFor::CurrentChunk => build_chunk_filter(0, current_chunk),
        ComputeCollidersFor::ChunksNearPlayer { distance } => {
            build_chunk_filter(*distance as i32, current_chunk)
        }
        ComputeCollidersFor::None => return,
    };

    chunks
        .iter()
        .filter_map(|(chunk, entity)| chunk_filter(&chunk.cords).then(|| entity))
        .for_each(|chunk_entity| {
            commands
                .entity(chunk_entity)
                .insert(AsyncChunkColliderBundle::default());
        });
}

fn generate_collider_for_chunks_from_block_aabb<const N: usize>(
    mut commands: Commands,
    non_collidable_blocks_query: StaticBlockQuery<&BlockNonCollidable>,
    mesh_registry: Res<MeshReg>,
    chunks_to_generate_collider_for: Query<(Entity, &Chunk), With<AsyncChunkCollider>>,
    blocks: _Blocks<N>,
) {
    for (chunk_entity, Chunk { cords: chunk_cords }) in chunks_to_generate_collider_for.iter() {
        if let Some(chunk_grid) = blocks.get_chunk_grid(*chunk_cords) {
            let mut chunk_tri_mesh = ChunkTriMesh::new();
            for (block_pos, block_id) in chunk_grid.enumerate_blocks() {
                if non_collidable_blocks_query
                    .get_static_property(block_id)
                    .is_some()
                {
                    continue;
                }
                let block_aabb = mesh_registry.get_block_mesh_aabb(&block_id);
                chunk_tri_mesh.append_block_aabb(block_aabb, block_pos);
            }
            if chunk_tri_mesh.vertices.is_empty() {
                continue;
            }
            let chunk_collider = Collider::trimesh_with_config(
                chunk_tri_mesh.vertices,
                chunk_tri_mesh.indices,
                TriMeshFlags::MERGE_DUPLICATE_VERTICES,
            );

            commands
                .entity(chunk_entity)
                .insert(chunk_collider)
                .remove::<AsyncChunkCollider>();
        }
    }
}

fn generate_collider_for_chunks_from_block_mesh<const N: usize>() {
    unimplemented!()
}

#[derive(Bundle)]
pub struct AsyncChunkColliderBundle {
    pub async_marker: AsyncChunkCollider,
    pub rigid_body: RigidBody,
    pub collision_layers: CollisionLayers,
}

impl Default for AsyncChunkColliderBundle {
    fn default() -> Self {
        Self {
            async_marker: AsyncChunkCollider,
            rigid_body: RigidBody::Static,
            collision_layers: CollisionLayers::new(
                [MoxiCollisionLayer::Terrain],
                [MoxiCollisionLayer::Player, MoxiCollisionLayer::Other],
            ),
        }
    }
}

struct ChunkTriMesh {
    vertices: Vec<Vector3>,
    indices: Vec<[u32; 3]>,
}

impl ChunkTriMesh {
    fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    fn append_block_aabb(&mut self, aabb: BevyAabb, block_pos: BlockPos) {
        let p_aabb = bevy_aabb_to_parry_aabb(aabb);
        p_aabb.transform_by(&Isometry3::translation(
            block_pos.x as f32,
            block_pos.y as f32,
            block_pos.z as f32,
        ));
        let (vers, inds) = p_aabb.to_trimesh();
        let vers: Vec<Vector3> = vers
            .iter()
            .map(|v| Vector3::new(v.x, v.y, v.z))
            .collect::<Vec<Vector3>>();
        let inds = inds
            .iter()
            .map(|[a, b, c]| {
                [
                    *a + self.vertices.len() as u32,
                    *b + self.vertices.len() as u32,
                    *c + self.vertices.len() as u32,
                ]
            })
            .collect::<Vec<_>>();
        self.vertices.extend(&vers);
        self.indices.extend_from_slice(inds.as_slice());
    }
}

fn bevy_aabb_to_parry_aabb(aabb: BevyAabb) -> Aabb {
    Aabb::from_half_extents(aabb.center.into(), aabb.half_extents.into())
}

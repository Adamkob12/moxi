use std::sync::Arc;

use crate::{
    blockreg::meshreg::MeshReg,
    chunk::{
        chunkbuilder::{BoxedBuilder, ChunkBuilder},
        components::{
            ChildMeshChunks, Chunk, ChunkGrid, CubeMeshChunk, CustomMeshChunk, MeshChunk,
            ToIntroduce, XSpriteMeshChunk,
        },
        meshmd::ChunkMeshMd,
        resources::{ChunkMap, ChunkQueue, CurrentChunk},
        CubeMeshMaterial, CustomMeshMaterial, XSpriteMeshMaterial,
    },
    plugin::RENDER_DISTANCE,
};
use bevy_asset::Assets;
use bevy_ecs::prelude::*;
use bevy_hierarchy::{BuildChildren, DespawnRecursiveExt};
use bevy_math::prelude::Vec3;
use bevy_pbr::PbrBundle;
use bevy_render::mesh::Mesh;
use bevy_render::prelude::SpatialBundle;
use bevy_tasks::{prelude::AsyncComputeTaskPool, Task};
use bevy_transform::prelude::Transform;
use moxi_mesh_utils::prelude::{
    meshify_cubic_voxels, meshify_custom_voxels, meshify_xsprite_voxels, MeshingAlgorithm,
};
use moxi_utils::prelude::{chunk_distance, ChunkCords, Face};

const CHUNK_TRANSLATION_OFFSET: Vec3 = Vec3::splat(0.5);

#[derive(Component)]
pub struct ComputeChunk<const N: usize>(Task<Option<ChunkGenResult<N>>>);

#[derive(Component)]
pub struct ChunkGenResult<const N: usize> {
    pub cords: ChunkCords,
    pub cube_mesh: Mesh,
    pub cube_mesh_md: ChunkMeshMd,
    pub xsprite_mesh: Mesh,
    pub xsprite_mesh_md: ChunkMeshMd,
    pub custom_mesh: Mesh,
    pub custom_mesh_md: ChunkMeshMd,
    pub chunk_grid: ChunkGrid<N>,
}

pub fn spawn_chunks<const N: usize>(
    mut commands: Commands,
    mut chunks_tasks_query: Query<(Entity, &mut ComputeChunk<N>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    cube_mesh_material: Res<CubeMeshMaterial>,
    xsprite_mesh_material: Res<XSpriteMeshMaterial>,
    custom_mesh_material: Res<CustomMeshMaterial>,
) {
    chunks_tasks_query
        .iter_mut()
        .for_each(|(entity, mut task)| {
            if let Some(chunk_generation_results) =
                futures_lite::future::block_on(futures_lite::future::poll_once(&mut task.0))
            {
                commands.entity(entity).despawn();

                let ChunkGenResult {
                    cords,
                    cube_mesh,
                    cube_mesh_md,
                    xsprite_mesh,
                    xsprite_mesh_md,
                    custom_mesh,
                    custom_mesh_md,
                    chunk_grid,
                } = chunk_generation_results.unwrap();
                let parent_transform = Transform::from_translation(
                    Vec3 {
                        x: cords.x as f32 * chunk_grid.dims.x as f32,
                        y: 0.0,
                        z: cords.y as f32 * chunk_grid.dims.z as f32,
                    } + CHUNK_TRANSLATION_OFFSET,
                );

                let parent_chunk = commands
                    .spawn((
                        Chunk { cords },
                        chunk_grid,
                        SpatialBundle::from_transform(parent_transform),
                        ToIntroduce::new(cords),
                    ))
                    .id();

                let cube_mesh_chunk = commands
                    .spawn((
                        MeshChunk { parent_chunk },
                        CubeMeshChunk,
                        PbrBundle {
                            mesh: meshes.add(cube_mesh),
                            material: cube_mesh_material.0.clone(),
                            ..Default::default()
                        },
                        cube_mesh_md,
                    ))
                    .id();

                let xsprite_mesh_chunk = commands
                    .spawn((
                        MeshChunk { parent_chunk },
                        XSpriteMeshChunk,
                        PbrBundle {
                            mesh: meshes.add(xsprite_mesh),
                            material: xsprite_mesh_material.0.clone(),
                            ..Default::default()
                        },
                        xsprite_mesh_md,
                    ))
                    .id();

                let custom_mesh_chunk = commands
                    .spawn((
                        MeshChunk { parent_chunk },
                        CustomMeshChunk,
                        PbrBundle {
                            mesh: meshes.add(custom_mesh),
                            material: custom_mesh_material.0.clone(),
                            ..Default::default()
                        },
                        custom_mesh_md,
                    ))
                    .id();

                commands.entity(parent_chunk).insert(ChildMeshChunks {
                    cube_mesh_chunk,
                    xsprite_mesh_chunk,
                    custom_mesh_chunk,
                });

                commands.entity(parent_chunk).push_children(&[
                    cube_mesh_chunk,
                    xsprite_mesh_chunk,
                    custom_mesh_chunk,
                ]);
            }
        });
}

pub fn queue_chunks_to_spawn(
    mut chunk_queue: ResMut<ChunkQueue>,
    chunk_map: Res<ChunkMap>,
    current_chunk: Res<CurrentChunk>,
) {
    let current_chunk = current_chunk.get();
    for x in -RENDER_DISTANCE..=RENDER_DISTANCE {
        for z in -RENDER_DISTANCE..=RENDER_DISTANCE {
            let chunk_cords = ChunkCords::from([x, z]) + current_chunk;
            if !chunk_map.contains_chunk(chunk_cords) {
                chunk_queue.push(chunk_cords);
            }
        }
    }
}

pub fn despawn_chunks(
    mut commands: Commands,
    mut chunk_map: ResMut<ChunkMap>,
    current_chunk: Res<CurrentChunk>,
) {
    let current_chunk = current_chunk.get();
    for chunk_entity in chunk_map
        .extract_if(|cords| chunk_distance(*cords, current_chunk) > RENDER_DISTANCE)
        .into_iter()
    {
        commands.entity(chunk_entity).despawn_recursive();
    }
}

pub fn build_chunks<const N: usize>(
    mut chunk_queue: ResMut<ChunkQueue>,
    chunk_builder: Res<BoxedBuilder<N>>,
    mesh_registry: Res<'static, MeshReg>,
    mut chunk_map: ResMut<ChunkMap>,
    mut commands: Commands,
) {
    let async_task_pool = AsyncComputeTaskPool::get();
    let mesh_registry = Arc::new(mesh_registry.into_inner());
    for chunk_cords in chunk_queue.drain() {
        chunk_map.insert_chunk(chunk_cords, Entity::PLACEHOLDER);
        let new_mesh_reg = Arc::clone(&mesh_registry);
        let chunk_grid = chunk_builder.build_chunk(chunk_cords);
        let task = async_task_pool.spawn(async move {
            let (cube_chunk_mesh, cube_mesh_md) = meshify_cubic_voxels(
                &[Face::Bottom],
                &chunk_grid,
                *new_mesh_reg,
                MeshingAlgorithm::Culling,
                None,
            )?;
            let (xsprite_chunk_mesh, xsprite_mesh_md) =
                meshify_xsprite_voxels(*new_mesh_reg, &chunk_grid);
            let (custom_chunk_mesh, custom_mesh_md) =
                meshify_custom_voxels(*new_mesh_reg, &chunk_grid);

            Some(ChunkGenResult {
                cords: chunk_cords,
                cube_mesh: cube_chunk_mesh,
                cube_mesh_md: ChunkMeshMd::Cube(cube_mesh_md),
                xsprite_mesh: xsprite_chunk_mesh,
                xsprite_mesh_md: ChunkMeshMd::Xsprite(xsprite_mesh_md),
                custom_mesh: custom_chunk_mesh,
                custom_mesh_md: ChunkMeshMd::Custom(custom_mesh_md),
                chunk_grid: ChunkGrid(chunk_grid),
            })
        });
        commands.spawn(ComputeChunk(task));
    }
}

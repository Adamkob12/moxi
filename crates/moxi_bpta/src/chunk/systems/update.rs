use bevy_asset::{Assets, Handle};
use bevy_ecs::prelude::*;
use bevy_render::mesh::Mesh;
use moxi_mesh_utils::prelude::{
    introduce_adjacent_chunks, update_cube_mesh, update_custom_mesh, update_xsprite_mesh,
    EMPTY_AABB,
};
use moxi_utils::prelude::adj_chunk;

use crate::{
    blockreg::meshreg::MeshReg,
    chunk::{
        components::{
            ChildMeshChunks, ChunkGrid, ChunkMeshType, CubeMeshChunk, ToIntroduce, ToUpdate,
        },
        meshmd::ChunkMeshMd,
        resources::ChunkMap,
    },
};

pub fn handle_chunk_updates(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut chunks_to_update: Query<
        (Entity, &ChunkMeshType, &Handle<Mesh>, &mut ChunkMeshMd),
        With<ToUpdate>,
    >,
    mesh_registry: Res<MeshReg>,
) {
    let mesh_registry = mesh_registry.into_inner();
    for (chunk_entity, chunk_mesh_type, mesh_handle, mut chunk_mesh_md) in &mut chunks_to_update {
        let chunk_mesh = meshes.get_mut(mesh_handle).unwrap();
        match (chunk_mesh_type, chunk_mesh_md.as_mut()) {
            (ChunkMeshType::Cube, ChunkMeshMd::Cube(ref mut md)) => {
                update_cube_mesh(chunk_mesh, md, mesh_registry);
            }
            (ChunkMeshType::XSprite, ChunkMeshMd::Xsprite(ref mut md)) => {
                update_xsprite_mesh(mesh_registry, chunk_mesh, md);
            }
            (ChunkMeshType::Custom, ChunkMeshMd::Custom(ref mut md)) => {
                update_custom_mesh(mesh_registry, chunk_mesh, md);
            }
            _ => panic!("Chunk mesh type and mesh meta-data type mismatch"),
        }

        let aabb = chunk_mesh.compute_aabb().unwrap_or(EMPTY_AABB);

        commands
            .entity(chunk_entity)
            .remove::<ToUpdate>()
            .insert(aabb);
    }
}

pub fn introduce_adj_chunks<const N: usize>(
    mut commands: Commands,
    chunk_grids: Query<&ChunkGrid<N>>,
    mut parent_chunks: Query<(Entity, &ChildMeshChunks, &mut ToIntroduce)>,
    chunk_map: Res<ChunkMap>,
    mut cube_mesh_chunks: Query<&mut ChunkMeshMd, With<CubeMeshChunk>>,
    mesh_registry: Res<MeshReg>,
) {
    let mesh_registry = mesh_registry.into_inner();
    for (chunk_entity, child_mesh_chunks, mut to_introduce) in parent_chunks.iter_mut() {
        let mut couldnt_introduce = Vec::new();
        let chunk_cords = to_introduce.cords;
        for connection_face in to_introduce.adj_chunks_to_introduce.drain(..) {
            let adj_chunk_cords = adj_chunk(chunk_cords, connection_face);
            if let Some(adj_chunk_entity) = chunk_map.get_chunk(adj_chunk_cords) {
                let cube_mesh_entity = child_mesh_chunks.cube_mesh_chunk;
                let mut cube_mesh_md = cube_mesh_chunks.get_mut(cube_mesh_entity).unwrap();
                if let Ok(adj_chunk_grid) = chunk_grids.get(adj_chunk_entity) {
                    introduce_adjacent_chunks(
                        mesh_registry,
                        cube_mesh_md.get_cube_md_mut().unwrap(),
                        connection_face,
                        &adj_chunk_grid.0,
                    );
                    commands.entity(cube_mesh_entity).insert(ToUpdate);
                }
            } else {
                couldnt_introduce.push(connection_face);
            }
        }
        if couldnt_introduce.is_empty() {
            commands.entity(chunk_entity).remove::<ToIntroduce>();
        } else {
            to_introduce.adj_chunks_to_introduce = couldnt_introduce;
        }
    }
}

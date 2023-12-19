use bevy_asset::{Assets, Handle};
use bevy_ecs::prelude::*;
use bevy_render::mesh::Mesh;
use moxi_mesh_utils::prelude::{
    update_cube_mesh, update_custom_mesh, update_xsprite_mesh, EMPTY_AABB,
};

use crate::{
    blockreg::meshreg::MeshReg,
    chunk::{
        components::{ChunkMeshType, ToUpdate},
        meshmd::ChunkMeshMd,
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
            (ChunkMeshType::Xsprite, ChunkMeshMd::Xsprite(ref mut md)) => {
                update_xsprite_mesh(mesh_registry, chunk_mesh, md);
            }
            (ChunkMeshType::Custom, ChunkMeshMd::Custom(ref mut md)) => {
                update_custom_mesh(mesh_registry, chunk_mesh, md)
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

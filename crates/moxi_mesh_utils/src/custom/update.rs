use super::md::{CustomMD, CustomVIVI};
use super::*;
use crate::*;

/// Update the [`custom`](`BlockMeshType::Custom`) chunk mesh according to the metadata.
pub fn update_custom_mesh<B: BlockInGrid>(
    reg: &impl MeshRegistry<B>,
    mesh: &mut Mesh,
    md: &mut CustomMD<B>,
) {
    for (change, block, block_pos) in md.log.iter().filter(|(_, block, _)| reg.is_custom(block)) {
        match change {
            BlockMeshChange::Added => add_custom_block(
                mesh,
                &mut md.vivi,
                *block_pos,
                reg.get_block_mesh_ref(block).unwrap(),
                reg.get_block_dims().into(),
            ),
            BlockMeshChange::Broken => {
                remove_custom_voxel(
                    mesh,
                    &mut md.vivi,
                    *block_pos,
                    reg.get_block_mesh_vertex_count(block),
                    reg.get_block_mesh_indices_len(block),
                );
            }
            _ => debug_assert!(
                false,
                "tried using unsupported BlockMeshChange in Custom Mesh"
            ),
        }
    }
    md.log.clear();
}

/// Remove an [`custom`](`BlockMeshType::Custom`) block from the mesh.
fn remove_custom_voxel(
    mesh: &mut Mesh,
    vivi: &mut CustomVIVI,
    block_pos: BlockPos,
    vertex_count: usize,
    indices_count: usize,
) {
    let (vertex_start, index_start) = vivi.remove(&block_pos).unwrap();
    // remove vertices
    for (_, vav) in mesh.attributes_mut() {
        for vertex in (vertex_start as usize..(vertex_start as usize + vertex_count)).rev() {
            vav.remove(vertex);
        }
    }
    // remove indices & offset the rest
    if let Some(Indices::U32(ref mut indices)) = mesh.indices_mut() {
        for index in (index_start as usize..(index_start as usize + indices_count)).rev() {
            indices.remove(index);
        }

        // offset the indices that were affected by removing `vertex_count` amount of vertices
        for index in indices.iter_mut() {
            if *index > vertex_start as u32 {
                *index -= vertex_count as u32;
            }
        }
    }
}

/// Add an [`custom`](`BlockMeshType::custom`) block to the mesh.
fn add_custom_block(
    mesh: &mut Mesh,
    vivi: &mut CustomVIVI,
    block_pos: BlockPos,
    voxel_mesh: &Mesh,
    voxel_dims: Vec3,
) {
    let ver_count = mesh.count_vertices();
    let indices_count;

    let position_offset = voxel_dims * block_pos.as_vec3();
    // add the vertices
    for (id, vav) in mesh.attributes_mut() {
        if id == Mesh::ATTRIBUTE_POSITION.id {
            let vav2 = voxel_mesh
                .attribute(Mesh::ATTRIBUTE_POSITION.id)
                .unwrap()
                .offset_all(position_offset.into());
            vav.extend(&vav2);
        } else {
            let vav2 = voxel_mesh.attribute(id).unwrap();
            vav.extend(vav2);
        }
    }

    // add the indices
    let Some(Indices::U32(ref mut indices)) = mesh.indices_mut() else {
        panic!("Expected U32 indices format");
    };
    indices_count = indices.len();
    if let Some(Indices::U32(voxel_indices)) = voxel_mesh.indices() {
        let indices_offset: Vec<u32> = voxel_indices
            .clone()
            .iter()
            .map(|x| *x + ver_count as u32)
            .collect();
        indices.extend(indices_offset);
    }

    // add the block to the VIVI
    vivi.insert(
        block_pos,
        (ver_count as VertexIndex, indices_count as IndexIndex),
    );
}

use super::md::{XSpriteMD, XSpriteVIVI};
use crate::*;

/// Update the [`xsprite`](`BlockMeshType::XSprite`) chunk mesh according to the metadata.
pub fn update_xsprite_mesh<B: BlockInGrid>(
    reg: &impl MeshRegistry<B>,
    mesh: &mut Mesh,
    md: &mut XSpriteMD<B>,
    dims: Dimensions,
) {
    for (change, block, block_pos) in md.log.iter().filter(|(_, block, _)| reg.is_xsprite(block)) {
        match change {
            BlockMeshChange::Added => add_xsprite_block(
                mesh,
                &mut md.vivi,
                *block_pos,
                reg.get_block_mesh_ref(block).unwrap(),
                reg.get_block_dims().into(),
                dims,
            ),
            BlockMeshChange::Broken => {
                remove_xsprite_voxel(mesh, &mut md.vivi, *block_pos, dims);
            }
            _ => debug_assert!(false, "tried using unsupported VoxelChange in XSpriteMesh"),
        }
    }
    md.log.clear();
}

/// Remove an [`xsprite`](`BlockMeshType::XSprite`) block from the mesh.
fn remove_xsprite_voxel(
    mesh: &mut Mesh,
    md: &mut XSpriteVIVI,
    block_pos: BlockPos,
    dims: Dimensions,
) {
    let block_index = pos_to_index(block_pos, dims).unwrap();
    let (vertex_start, vertex_end, index_start, index_end) = md[block_index];
    let last = vertex_end == mesh.count_vertices();
    if vertex_end - vertex_start > 0 {
        for (_, vav) in mesh.attributes_mut() {
            for vertex in (vertex_start..vertex_end).rev() {
                vav.swap_remove(vertex);
            }
        }
        if let Some(Indices::U32(ref mut indices)) = mesh.indices_mut() {
            for _ in (index_start..index_end).rev() {
                // indices.swap_remove(i as usize);
                indices.pop();
            }
        }

        md[block_index] = (usize::MIN, usize::MIN, u32::MIN, u32::MIN);
        if !last {
            let mut max = (0, 0);
            for (i, (v, _, _, _)) in md.iter().enumerate() {
                if *v > max.1 {
                    max = (i, *v);
                }
            }
            md[max.0] = (vertex_start, vertex_end, index_start, index_end);
        }
    }
}

/// Add an [`xsprite`](`BlockMeshType::XSprite`) block to the mesh.
fn add_xsprite_block(
    mesh: &mut Mesh,
    md: &mut XSpriteVIVI,
    block_pos: BlockPos,
    voxel_mesh: &Mesh,
    voxel_dims: Vec3,
    dims: Dimensions,
) {
    let block_index = pos_to_index(block_pos, dims).unwrap();
    let ver_count = mesh.count_vertices();
    let position_offset = voxel_dims * block_pos.as_vec3();
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

    let ver_count2 = mesh.count_vertices();
    let mut ind_count = 0;
    let mut ind_count2 = 0;
    if let Some(Indices::U32(ref mut indices)) = mesh.indices_mut() {
        ind_count = indices.len();
        if let Some(Indices::U32(voxel_indices)) = voxel_mesh.indices() {
            let indices_offset: Vec<u32> = voxel_indices
                .clone()
                .iter()
                .map(|x| *x + ver_count as u32)
                .collect();
            indices.extend(indices_offset);
            ind_count2 = indices.len();
        }
    }
    md[block_index] = (ver_count, ver_count2, ind_count as u32, ind_count2 as u32);
}

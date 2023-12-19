use super::md::{CustomMD, CustomVIVI};
use super::*;
use crate::*;

/// Meshify all of the [`custom`](`BlockMeshType::Custom`) blocks in a chunk grid.
pub fn meshify_custom_voxels<B: BlockInGrid, const N: usize>(
    reg: &impl MeshRegistry<B>,
    grid: &Grid<B, N>,
) -> (Mesh, CustomMD<B>) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mut indices: Vec<u32> = vec![];
    let mut vertices: Vec<(MeshVertexAttribute, VertexAttributeValues)> = vec![];
    for att in reg.custom_attributes().iter() {
        vertices.push((att.clone(), VertexAttributeValues::new(att.format.clone())));
    }

    // data structure similar to VIVI, to map voxel index
    let mut vivi: CustomVIVI = CustomVIVI::new();
    let voxel_dims = reg.get_block_dims();

    // Filter out non-[`custom`](`BlockMeshType::Custom`) and extract the block's mesh.
    let enumerated_custom_meshes = grid.enumerate_blocks().filter_map(|(pos, block)| {
        if let Some(custom_mesh) = reg.get_block_mesh_ref(&block).get_if(BlockMeshType::Custom) {
            Some((pos, custom_mesh))
        } else {
            None
        }
    });

    for (block_pos, custom_mesh) in enumerated_custom_meshes {
        let total_vertices = vertices[0].1.len();
        let total_indices = indices.len();

        let position_offset = (
            block_pos.x as f32 * voxel_dims[0],
            block_pos.y as f32 * voxel_dims[1],
            block_pos.z as f32 * voxel_dims[2],
        );

        // Offset and add the indices to the total indices.
        let Indices::U32(ind) = custom_mesh.indices().expect("couldn't get indices data") else {
            panic!("Expected U32 indices format");
        };
        let ind: Vec<u32> = ind.iter().map(|i| *i + total_vertices as u32).collect();
        // Add the vertices to the total vertices, offset the position attribute.
        for (id, vals) in vertices.iter_mut() {
            let mut att = custom_mesh
                .attribute(id.id)
                .expect(format!("Couldn't retrieve voxel mesh attribute {:?}.", id).as_str())
                .clone();
            if id.id == Mesh::ATTRIBUTE_POSITION.id {
                att = att.offset_all(position_offset);
            }
            vals.extend(&att);
        }
        indices.extend(ind);

        vivi.insert(
            block_pos,
            (total_vertices as VertexIndex, total_indices as IndexIndex),
        );
    }

    for (att, vals) in vertices {
        mesh.insert_attribute(att, vals);
    }
    mesh.set_indices(Some(Indices::U32(indices)));
    (
        mesh,
        CustomMD {
            vivi,
            log: vec![],
            dims: grid.dims,
        },
    )
}

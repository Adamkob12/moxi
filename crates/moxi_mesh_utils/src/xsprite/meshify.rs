use crate::*;

use super::md::XSpriteMD;

/// Meshify all of the [`xsprite`](`BlockMeshType::XSprite`) blocks in a chunk grid.
pub fn meshify_xsprite_voxels<B: BlockInGrid, const N: usize>(
    reg: &impl MeshRegistry<B>,
    grid: &Grid<B, N>,
) -> (Mesh, XSpriteMD<B>) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mut indices: Vec<u32> = vec![];
    let mut positions: Vec<[f32; 3]> = vec![];
    let mut uvs: Vec<[f32; 2]> = vec![];
    let mut normals: Vec<[f32; 3]> = vec![];

    // data structure similar to VIVI, to map voxel index
    let mut data_structure = vec![(usize::MIN, usize::MIN, u32::MIN, u32::MIN); grid.len()];
    let voxel_dims = reg.get_block_dims();

    for (block_pos, xsprite_mesh) in grid.enumerate_blocks().filter_map(|(pos, block)| {
        if let Some(xsprite_mesh) = reg
            .get_block_mesh_ref(&block)
            .get_if(BlockMeshType::XSprite)
        {
            Some((pos, xsprite_mesh))
        } else {
            None
        }
    }) {
        let position_offset = (
            block_pos.x as f32 * voxel_dims[0],
            block_pos.y as f32 * voxel_dims[1],
            block_pos.z as f32 * voxel_dims[2],
        );
        let pos_attribute = xsprite_mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .expect("couldn't get voxel mesh data");
        let VertexAttributeValues::Float32x3(pos) = pos_attribute else {
            panic!("Unexpected vertex format for position attribute, expected Float32x3.");
        };
        let pos: Vec<[f32; 3]> = pos
            .iter()
            .map(|[x, y, z]| {
                [
                    *x + position_offset.0,
                    *y + position_offset.1,
                    *z + position_offset.2,
                ]
            })
            .collect();

        let VertexAttributeValues::Float32x4(col) = xsprite_mesh
            .attribute(Mesh::ATTRIBUTE_COLOR)
            .expect("couldn't get mesh data")
        else {
            panic!("Incorrect format for colors");
        };
        let VertexAttributeValues::Float32x2(uv) = xsprite_mesh
            .attribute(Mesh::ATTRIBUTE_UV_0)
            .expect("couldn't get mesh data")
        else {
            panic!("Incorrect format for uvs");
        };
        let VertexAttributeValues::Float32x3(nor) = xsprite_mesh
            .attribute(Mesh::ATTRIBUTE_NORMAL)
            .expect("couldn't get mesh data")
        else {
            panic!("Incorrect format for normals");
        };
        let Indices::U32(ind) = xsprite_mesh.indices().expect("couldn't get indices data") else {
            panic!("Expected U32 indices format");
        };
        let ind: Vec<u32> = ind.iter().map(|i| *i + positions.len() as u32).collect();

        let block_index = pos_to_index(block_pos, grid.dims).unwrap();
        data_structure[block_index].0 = positions.len();
        data_structure[block_index].2 = indices.len() as u32;

        positions.extend(pos);
        normals.extend(nor);
        uvs.extend(uv);
        indices.extend(ind);

        data_structure[block_index].1 = positions.len();
        data_structure[block_index].3 = indices.len() as u32;
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_indices(Some(Indices::U32(indices)));

    (
        mesh,
        XSpriteMD {
            vivi: data_structure.to_vec(),
            log: vec![],
            dims: grid.dims,
        },
    )
}

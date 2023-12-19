use crate::*;

/// All the variants for the Meshing algorithm.
#[derive(Debug, Clone, Copy)]
pub enum MeshingAlgorithm {
    Naive,
    Culling,
}

/// Arguments:
/// - [`outer_layer`](&[Face]): The faces of the blocks that are on the outer layer of the grid.
/// - [`grid`](`ChunkGrid`): The grid of the blocks, this is the data structure that all of the
///     information about the blocks is stored in. It is a wrapper around a  3D array of blocks.
/// - [`reg`](`MeshRegistry`): The mesh registry that contains all the meshes of the blocks.
/// - ['ma'](MeshingAlgorithm): The meshing algorithm to use - currently supports Culling and
///     Naive. (Culling is always better than Naive)
/// - ['sl'](`SmoothLightingParameters`): Enable Smooth Lighting (Some ..) or not (None). Smooth Lighting is a technique often used in
///     voxel based games that resembles Ambient Occlusion, but it is static- which means the
///     shadows are computed only once, when the mesh is generated (or updated).
///
/// Returns the mesh and the mesh metadata.
pub fn meshify_cubic_voxels<B: BlockInGrid, const N: usize>(
    outer_layer: &[Face],
    grid: &Grid<B, N>,
    reg: &impl MeshRegistry<B>,
    meshing_algorithm: MeshingAlgorithm,
    smooth_lighting_params: Option<SmoothLightingParameters>,
) -> Option<(Mesh, CubeMD<B>)> {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let total_voxels = grid.len();
    let mut vivi = CubeVIVI::new(total_voxels);
    let outer_layers_to_call = {
        let mut r = [true, true, true, true, true, true];
        for f in outer_layer {
            r[*f as usize] = false;
        }
        r
    };

    let mut indices: Vec<u32> = vec![];
    let mut vertices: Vec<(MeshVertexAttribute, VertexAttributeValues)> = vec![];
    for att in reg.all_attributes().iter() {
        vertices.push((att.clone(), VertexAttributeValues::new(att.format.clone())));
    }
    let voxel_dims = reg.get_block_dims();
    let center = reg.get_block_center();

    for (block_pos, block) in grid.enumerate_blocks().filter(|(_, v)| reg.is_cube(v)) {
        let position_offset = Vec3::from(voxel_dims) * block_pos.as_vec3();

        let sides_to_cull = match meshing_algorithm {
            MeshingAlgorithm::Culling => grid.enumerate_neighbors(block_pos).map(|(f, n)| {
                n.map_or_else(|| outer_layers_to_call[f as usize], |t| reg.is_cube(&t))
            }),
            MeshingAlgorithm::Naive => [true; 6],
        };

        if sides_to_cull == [false; 6] {
            continue;
        }

        add_vertices_normal_cube(
            sides_to_cull,
            &mut indices,
            &mut vertices,
            reg.get_block_mesh_ref(&block).unwrap(),
            &mut vivi,
            block_pos,
            center,
            position_offset.into(),
            grid.dims,
        );
    }

    for (att, vals) in vertices {
        mesh.insert_attribute(att, vals);
    }
    mesh.set_indices(Some(Indices::U32(indices)));

    let d_mesh = CubeMD {
        dims: grid.dims,
        smooth_lighting_params,
        vivi,
        changed_voxels: vec![],
    };

    if let Some(t) = smooth_lighting_params {
        if t.apply_at_gen {
            apply_smooth_lighting(reg, &mut mesh, &d_mesh, grid.dims, 0, total_voxels, grid);
        }
    }
    Some((mesh, d_mesh))
}

/// Important helper function to add the vertices and indices of each voxel into the running count of vertices
/// and indices, preserving their attributes, and (important!) assigning a custom offset to the
/// position attributes, we are assuming this is only needed for the position attributes (because
/// it usually is).
fn add_vertices_normal_cube(
    sides_to_cull: [bool; 6],
    indices_main: &mut Vec<u32>,
    vertices: &mut Vec<(MeshVertexAttribute, VertexAttributeValues)>,
    voxel: &Mesh,
    vivi: &mut CubeVIVI,
    voxel_pos: BlockPos,
    center: [f32; 3],
    position_offset: (f32, f32, f32),
    dims: Dimensions,
) {
    let vertices_count = vertices[0].1.len();
    let pos_attribute = voxel
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .expect("couldn't get voxel mesh data");
    let VertexAttributeValues::Float32x3(positions) = pos_attribute else {
        panic!("Unexpected vertex format for position attribute, expected Float32x3.");
    };
    let Indices::U32(indices) = voxel.indices().expect("couldn't get indices data") else {
        panic!("Expected U32 indices format");
    };
    let triangles = indices
        .chunks(3)
        .map(|chunk| (chunk[0], chunk[1], chunk[2]));

    // define the indices and vertices we want to save of the voxel mesh
    let mut indices_to_save: Vec<u32> = vec![];
    // helper data structure
    let mut vertices_to_save: Vec<(bool, u32, Face)> = vec![(false, 0, Face::Top); positions.len()];
    // sorted vertices by the quad they are in
    let mut sorted_vertices: Vec<Option<Vec<u32>>> = vec![None; 6];
    // the final array of the vertices, it will be sorted, each 4 vertices will be a
    // part of one quad, we sort them this way to efficiently update the vivi.
    let mut final_vertices: Vec<u32> = vec![];

    // iterate over all the triangles in the mesh
    for (a, b, c) in triangles {
        let v1 = positions[a as usize];
        let v2 = positions[b as usize];
        let v3 = positions[c as usize];
        let mut save = (false, Face::Top);

        // see which side of the voxel the triangle belongs to
        for i in 0..3 {
            if v1[i] == v2[i] && v2[i] == v3[i] && v1[i] == v3[i] {
                match (i, center[i] > v1[i]) {
                    (0, true) if sides_to_cull[3] => save = (true, Face::Left),
                    (0, false) if sides_to_cull[2] => save = (true, Face::Right),
                    (1, true) if sides_to_cull[1] => save = (true, Face::Bottom),
                    (1, false) if sides_to_cull[0] => save = (true, Face::Top),
                    (2, true) if sides_to_cull[5] => save = (true, Face::Front),
                    (2, false) if sides_to_cull[4] => save = (true, Face::Back),
                    _ => save = (false, Face::Top),
                }
                break;
            }
        }

        // save the vertices
        if save.0 {
            let quad: usize = save.1.into();
            indices_to_save.push(a);
            indices_to_save.push(b);
            indices_to_save.push(c);
            match sorted_vertices[quad] {
                None => {
                    sorted_vertices[quad] = Some(vec![a, b, c]);
                    vertices_to_save[a as usize].0 = true;
                    vertices_to_save[b as usize].0 = true;
                    vertices_to_save[c as usize].0 = true;
                    vertices_to_save[a as usize].1 = 0;
                    vertices_to_save[b as usize].1 = 1;
                    vertices_to_save[c as usize].1 = 2;
                    vertices_to_save[a as usize].2 = save.1;
                    vertices_to_save[b as usize].2 = save.1;
                    vertices_to_save[c as usize].2 = save.1;
                }
                Some(ref mut v) => {
                    for &i in [a, b, c].iter() {
                        if !vertices_to_save[i as usize].0 {
                            v.push(i);
                            vertices_to_save[i as usize].2 = save.1;
                            vertices_to_save[i as usize].1 = v.len() as u32 - 1;
                            vertices_to_save[i as usize].0 = true;
                        }
                    }
                }
            }
        }
    }

    // The code from now on is a little messy, but it is very simple in actuality. It is mostly
    // just offseting the vertices and indices and formatting them into the right data-structres.

    // offset the vertices, since we won't be using all the vertices of the the mesh,
    // we need to find out which of them we will be using first, and then filter out
    // the ones we dont need.
    let mut offset: u32 = 0;
    for q in sorted_vertices.iter() {
        match q {
            None => offset += 4,
            Some(ref v) => {
                let mut only_first = true;
                for &i in v.iter() {
                    let face = vertices_to_save[i as usize].2;
                    vertices_to_save[i as usize].1 += face as u32 * 4 - offset;
                    final_vertices.push(i);
                    // update the vivi
                    if only_first {
                        vivi.insert_quad(
                            face,
                            pos_to_index(voxel_pos, dims).unwrap(),
                            i + vertices_count as u32 - offset,
                        );
                        only_first = false;
                    }
                }
            }
        }
    }

    // offset the indices, we need to consider the fact that the indices wil be part of a big mesh,
    // with a lot of vertices, so we must the vertices to a running count and offset them accordingly.
    for i in indices_to_save.iter_mut() {
        *i = vertices_to_save[*i as usize].1 + vertices_count as u32;
    }

    for (id, vals) in vertices.iter_mut() {
        let mut att = voxel
            .attribute(id.id)
            .expect(format!("Couldn't retrieve voxel mesh attribute {:?}.", id).as_str())
            .get_needed(&final_vertices);
        if id.id == Mesh::ATTRIBUTE_POSITION.id {
            att = att.offset_all(position_offset);
        }
        vals.extend(&att);
    }
    indices_main.extend(indices_to_save);
}

use crate::*;

pub fn update_cube_mesh<B: BlockInGrid>(
    mesh: &mut Mesh,
    metadata: &mut CubeMD<B>,
    reg: &impl MeshRegistry<B>,
) {
    let mut min = usize::MAX;
    let mut max = usize::MIN;
    let voxel_dims = reg.get_block_dims();
    for (block, block_pos, change, surrounding_blocks) in metadata
        .changed_voxels
        .iter()
        .filter(|(block, ..)| reg.is_cube(block))
    {
        let block_index = pos_to_index(*block_pos, metadata.dims).unwrap();
        if block_index < min {
            min = block_index;
        }
        if block_index > max {
            max = block_index;
        }
        let position_offset = (
            block_pos.x as f32 * voxel_dims[0],
            block_pos.y as f32 * voxel_dims[1],
            block_pos.z as f32 * voxel_dims[2],
        );
        let cube_neighbors: [bool; 6] = FACES
            .iter()
            .map(|face| surrounding_blocks[*face].map_or(false, |b| reg.is_cube(&b)))
            .collect::<Vec<bool>>()
            .try_into()
            .unwrap();

        let surrounding_block_meshes: Vec<(Face, &Mesh)> = {
            let mut r: Vec<(Face, &Mesh)> = vec![];
            for face in FACES.iter() {
                let neighbor = surrounding_blocks[*face];
                match neighbor {
                    None => continue,
                    Some(t) if reg.is_cube(&t) => {
                        r.push((*face, reg.get_block_mesh_ref(&t).unwrap()));
                    }
                    _ => continue,
                }
            }
            r
        };
        let block_mesh = reg.get_block_mesh_ref(block).unwrap();

        match *change {
            BlockMeshChange::Added => {
                remove_voxel(
                    mesh,
                    &mut metadata.vivi,
                    *block_pos,
                    [true; 6],
                    metadata.dims,
                );
                add_voxel_after_gen(
                    cube_neighbors,
                    mesh,
                    block_mesh,
                    &mut metadata.vivi,
                    *block_pos,
                    reg.get_block_center(),
                    position_offset,
                    metadata.dims,
                );
                remove_quads_facing(
                    mesh,
                    &mut metadata.vivi,
                    *block_pos,
                    metadata.dims,
                    cube_neighbors,
                );
            }
            BlockMeshChange::Broken => {
                remove_voxel(
                    mesh,
                    &mut metadata.vivi,
                    *block_pos,
                    [true; 6],
                    metadata.dims,
                );
                add_quads_facing(
                    mesh,
                    &mut metadata.vivi,
                    *block_pos,
                    surrounding_block_meshes,
                    reg.get_block_center(),
                    reg.get_block_dims(),
                    metadata.dims,
                );
            }
            BlockMeshChange::CullFaces => {
                remove_voxel(
                    mesh,
                    &mut metadata.vivi,
                    *block_pos,
                    cube_neighbors,
                    metadata.dims,
                );
            }
            BlockMeshChange::AddFaces => {
                add_voxel_after_gen(
                    cube_neighbors,
                    mesh,
                    block_mesh,
                    &mut metadata.vivi,
                    *block_pos,
                    reg.get_block_center(),
                    position_offset,
                    metadata.dims,
                );
            }
        }
    }

    metadata.changed_voxels.clear();
}

// The function removes all quads facing a voxel.
fn remove_quads_facing(
    mesh: &mut Mesh,
    vivi: &mut CubeVIVI,
    block_pos: BlockPos,
    dims: Dimensions,
    surrounding_block_is_cube: [bool; 6],
) {
    for connecting_face in surrounding_block_is_cube
        .iter()
        .enumerate()
        .filter(|(_, &b)| b)
        .map(|(i, _)| Face::from(i))
    {
        let neighbor_pos = match neighbor_pos(block_pos, connecting_face, dims) {
            None => continue,
            Some(i) => i,
        };
        let mut quad_to_remove = [false; 6];
        quad_to_remove[connecting_face as usize] = true;
        remove_voxel(mesh, vivi, neighbor_pos, quad_to_remove, dims);
    }
}

/// Function removes voxel from the big mesh.
/// `faces_to_remove`: The faces of the block to remove, if the corrresponding value is false,
/// that face's quad will not be removed.
fn remove_voxel(
    mesh: &mut Mesh,
    vivi: &mut CubeVIVI,
    block_pos: BlockPos,
    faces_to_remove: [bool; 6],
    dims: Dimensions,
) {
    let block_index = pos_to_index(block_pos, dims).unwrap();
    for (i, b) in faces_to_remove.iter().enumerate() {
        if !b {
            continue;
        }
        let face = Face::from(i);
        let quad = match vivi.get_quad_index(face, block_index) {
            None => continue,
            Some(i) => i,
        } as usize;
        if quad + 25 >= mesh.count_vertices() {
            for (_, vals) in mesh.attributes_mut() {
                vals.remove(quad + 3);
                vals.remove(quad + 2);
                vals.remove(quad + 1);
                vals.remove(quad + 0);
            }
            vivi.remove_quad(quad);
            let mut tmp = quad;
            while tmp != mesh.count_vertices() {
                vivi.change_quad_index(tmp + 4, tmp);
                tmp += 4;
            }
        } else {
            for (_, vals) in mesh.attributes_mut() {
                vals.swap_remove(quad + 3);
                vals.swap_remove(quad + 2);
                vals.swap_remove(quad + 1);
                vals.swap_remove(quad + 0);
            }
            let ver_count = mesh.count_vertices();
            vivi.remove_quad(quad);
            vivi.change_quad_index(ver_count, quad);
        }

        let Indices::U32(indices) = mesh.indices_mut().expect("couldn't get indices data") else {
            panic!("Expected U32 indices format");
        };
        for _ in 0..6 {
            indices.pop();
        }
    }
}

/// Function adds quads facing voxel.
/// `neighboring_cube_meshes`: The mesh of neighboring *cube* blocks.
pub(crate) fn add_quads_facing(
    mesh: &mut Mesh,
    vivi: &mut CubeVIVI,
    block_pos: BlockPos,
    neighboring_cube_meshes: Vec<(Face, &Mesh)>,
    center: [f32; 3],
    voxel_dims: [f32; 3],
    dims: Dimensions,
) {
    for &(face, vmesh) in neighboring_cube_meshes.iter() {
        let mut neig = [false; 6];
        neig[face.opposite() as usize] = true;
        let neighbor_pos = match neighbor_pos(block_pos, face, dims) {
            None => continue,
            Some(j) => j,
        };
        let position_offset = (Vec3::from(voxel_dims) * neighbor_pos.as_vec3()).into();
        add_voxel_after_gen(
            neig,
            mesh,
            vmesh,
            vivi,
            neighbor_pos,
            center,
            position_offset,
            dims,
        )
    }
}

/// This function adds a block to the mesh after it has been generated. The function wil autmoatically
/// cull the new unneeded faces.
/// `quads_to_keep`:
/// The faces of the block that are not covered by other blocks.
/// Will not add a face if its corresponding value is true.
fn add_voxel_after_gen(
    mut quads_to_keep: [bool; 6],
    main_mesh: &mut Mesh,
    block_mesh: &Mesh,
    vivi: &mut CubeVIVI,
    block_pos: BlockPos,
    center: [f32; 3],
    position_offset: (f32, f32, f32),
    dims: Dimensions,
) {
    let block_index = pos_to_index(block_pos, dims).unwrap();
    // Make sure we are not adding quads that already exist
    for (i, b) in quads_to_keep.iter_mut().enumerate() {
        let face = Face::from(i);
        if !*b && vivi.get_quad_index(face, block_index).is_some() {
            *b = true;
        }
    }
    let vertices_count = main_mesh.count_vertices();
    let Indices::U32(ref mut indices_main) =
        main_mesh.indices_mut().expect("Couldn't get indices data")
    else {
        panic!("Indices format should be U32");
    };

    let pos_attribute = block_mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .expect("couldn't get voxel mesh data");
    let VertexAttributeValues::Float32x3(positions) = pos_attribute else {
        panic!("Unexpected vertex format for position attribute, expected Float32x3.");
    };
    let Indices::U32(indices) = block_mesh.indices().expect("couldn't get indices data") else {
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
                    (0, true) if !quads_to_keep[3] => save = (true, Face::Left),
                    (0, false) if !quads_to_keep[2] => save = (true, Face::Right),
                    (1, true) if !quads_to_keep[1] => save = (true, Face::Bottom),
                    (1, false) if !quads_to_keep[0] => save = (true, Face::Top),
                    (2, true) if !quads_to_keep[5] => save = (true, Face::Front),
                    (2, false) if !quads_to_keep[4] => save = (true, Face::Back),
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
                        vivi.insert_quad(face, block_index, i + vertices_count as u32 - offset);
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
    indices_main.extend(indices_to_save);

    for (id, vals) in main_mesh.attributes_mut() {
        let mut att = block_mesh
            .attribute(id)
            .expect(format!("Couldn't retrieve voxel mesh attribute {:?}.", id).as_str())
            .get_needed(&final_vertices);
        if id == Mesh::ATTRIBUTE_POSITION.id {
            att = att.offset_all(position_offset);
        }
        vals.extend(&att);
    }
}

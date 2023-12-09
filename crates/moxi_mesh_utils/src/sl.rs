//! This module is responsible for Smooth Lighting. Smooth Lighting is a technique often used in
//! voxel based games that resembles Ambient Occlusion, but it is static- which means the
//! shadows are computed only once, when the mesh is generated (or updated).

use crate::*;

#[derive(Copy, Clone)]
/// Parameters for Smooth Lighting
pub struct SmoothLightingParameters {
    /// How intense the shadow is. 0.0 - 1.0
    pub intensity: f32,
    /// The max intensity value of the shadow (0.0 - 1.0) 1.0 is very dark, and depending on
    /// your intensity it may reach that level. Recommended: 0.6 - 0.8
    pub max: f32,
    /// Smoothing will often lower the overall intensity of the shadowing, but in return
    /// the scene will look more uniform. Recommended: 1.0 - 2.0
    pub smoothing: f32,
    /// True => Apply automatically after generating.
    /// False => The user will apply it manually using the smooth lighting API. (ex: `apply_smooth_lighting`)
    pub apply_at_gen: bool,
}

pub(crate) fn apply_sl_quad(
    mesh: &mut Mesh,
    vivi: &CubeVIVI,
    block_pos: BlockPos,
    face: Face,
    surrounding_blocks: [bool; 3 * 3 * 3],
    slparams: SmoothLightingParameters,
    voxel_dims: [f32; 3],
) {
    let block_index = pos_to_index(block_pos, CHUNK_DIMS).unwrap();
    let quad = vivi
        .get_quad_index(face, block_index)
        .expect("Couldn't find quad in vivi for smooth lighting");

    let positions = mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .expect("Cannot apply proximity-based-shadowing without the color attribute present");
    let VertexAttributeValues::Float32x3(positions) = positions else {
        panic!("Unexpected Format for the position attribute")
    };
    let voxel_center: Vec3 = block_pos.as_vec3() * Vec3::from(voxel_dims);
    let positions = {
        let mut r = vec![];
        for i in quad..(quad + 4) {
            r.push(Vec3::from(positions[i as usize]))
        }
        r
    };
    let colors = mesh
        .attribute_mut(Mesh::ATTRIBUTE_COLOR)
        .expect("Cannot apply proximity-based-shadowing without the color attribute present");
    let VertexAttributeValues::Float32x4(ref mut colors) = colors else {
        panic!("Unexpected Format for the color attribute")
    };

    let og: [i32; 3] = match face {
        Face::Top => [1, 0, 1],
        Face::Bottom => [1, 2, 1],
        Face::Right => [0, 1, 1],
        Face::Left => [2, 1, 1],
        Face::Back => [1, 1, 0],
        Face::Front => [1, 1, 2],
    };
    let grid_dims = (3, 3, 3);
    let [ogx, ogy, ogz] = og;
    for i in 0..4 {
        let ver = i + quad;
        let diff = positions[i as usize] - voxel_center;
        let mut total: f32 = 0.0;
        let (dx, dy, dz) = (
            diff.x.signum() as i32,
            diff.y.signum() as i32,
            diff.z.signum() as i32,
        );
        let nx = (ogx + dx) as u32;
        let ny = (ogy + dy) as u32;
        let nz = (ogz + dz) as u32;

        let tmp: UVec3 = [nx, ny, nz].into();
        if surrounding_blocks[pos_to_index(tmp, grid_dims.into()).unwrap()] {
            total += 0.75;
        }
        let tmp: UVec3 = [nx, ny, ogz as u32].into();
        if surrounding_blocks[pos_to_index(tmp, grid_dims.into()).unwrap()] {
            total += 1.0;
        }
        let tmp: UVec3 = [nx, ogy as u32, nz].into();
        if surrounding_blocks[pos_to_index(tmp, grid_dims.into()).unwrap()] {
            total += 1.0;
        }
        let tmp: UVec3 = [ogx as u32, ny, nz].into();
        if surrounding_blocks[pos_to_index(tmp, grid_dims.into()).unwrap()] {
            total += 1.0;
        }

        total = total.min(2.0);
        let color = total * slparams.intensity;
        let color = (1.0 - color.min(1.0).powf(slparams.smoothing)).max(1.0 - slparams.max);
        colors[ver as usize] = [color, color, color, 1.0]
    }
}

pub fn apply_smooth_lighting<T: BlockInGrid, const N: usize>(
    reg: &impl MeshRegistry<T>,
    mesh: &mut Mesh,
    metadata: &CubeMD<T>,
    dims: Dimensions,
    lower_bound: usize,
    upper_bound: usize,
    this_chunk: &ChunkGrid<T, N>,
) {
    apply_smooth_lighting_with_connected_chunks(
        reg,
        mesh,
        metadata,
        dims,
        lower_bound,
        upper_bound,
        this_chunk,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );
}

pub fn apply_smooth_lighting_with_connected_chunks<'a, T: BlockInGrid, const N: usize>(
    reg: &impl MeshRegistry<T>,
    mesh: &mut Mesh,
    metadata: &CubeMD<T>,
    dims: Dimensions,
    lower_bound: usize,
    upper_bound: usize,
    this_chunk: &'a ChunkGrid<T, N>,
    north_chunk: Option<&'a ChunkGrid<T, N>>,
    south_chunk: Option<&'a ChunkGrid<T, N>>,
    east_chunk: Option<&'a ChunkGrid<T, N>>,
    west_chunk: Option<&'a ChunkGrid<T, N>>,
    no_east_chunk: Option<&'a ChunkGrid<T, N>>,
    no_west_chunk: Option<&'a ChunkGrid<T, N>>,
    so_east_chunk: Option<&'a ChunkGrid<T, N>>,
    so_west_chunk: Option<&'a ChunkGrid<T, N>>,
) {
    if let Some(sl) = metadata.smooth_lighting_params {
        for (block_index, quads) in metadata.vivi.vivi.iter().enumerate().skip(lower_bound) {
            if block_index > upper_bound {
                break;
            }
            let block_pos = index_to_pos(block_index, dims).unwrap();
            for q in quads {
                let mut surrounding_blocks = [false; 3 * 3 * 3];
                let cage_dims = UVec3::new(3, 3, 3);
                let face = face_from_u32(q & REVERSE_OFFSET_CONST);

                if (matches!(face, Face::Bottom) || matches!(face, Face::Top))
                    && is_block_pos_on_edge(block_pos, face, dims)
                {
                    continue;
                }
                let (neighbor_pos, chunk_dir) = {
                    if is_block_pos_on_edge(block_pos, face, dims) {
                        (
                            neighbor_across_chunk(block_pos, face, dims).unwrap(),
                            Some(NDir::from(face)),
                        )
                    } else {
                        (neighbor_pos(block_pos, face, dims).unwrap(), None)
                    }
                };

                let og_index_in_cage: [i32; 3] = match face {
                    Face::Top => [0, -1, 0],
                    Face::Bottom => [0, 1, 0],
                    Face::Right => [-1, 0, 0],
                    Face::Left => [1, 0, 0],
                    Face::Back => [0, 0, -1],
                    Face::Front => [0, 0, 1],
                };
                let [og_x, og_y, og_z] = og_index_in_cage;

                for y in -1..=1 {
                    for z in -1..=1 {
                        for x in -1..=1 {
                            if (og_x == x && og_y == y)
                                || (og_x == x && og_z == z)
                                || (og_y == y && og_z == z)
                            {
                                continue;
                            }
                            if (og_x == x && og_x != 0)
                                || (og_y == y && og_y != 0)
                                || (og_z == z && og_z != 0)
                            {
                                continue;
                            }
                            if (og_x == x + 2 && og_x != 0)
                                || (og_y == y + 2 && og_y != 0)
                                || (og_z == z + 2 && og_z != 0)
                            {
                                continue;
                            }
                            if (og_x == x - 2 && og_x != 0)
                                || (og_y == y - 2 && og_y != 0)
                                || (og_z == z - 2 && og_z != 0)
                            {
                                continue;
                            }

                            let cage_index = pos_to_index(
                                BlockPos::from([(x + 1) as u32, (y + 1) as u32, (z + 1) as u32]),
                                cage_dims,
                            )
                            .unwrap();

                            match get_block_n_away(dims, neighbor_pos, x, y, z) {
                                None => {
                                    continue;
                                }
                                Some((dir, neighbor_block_pos)) => {
                                    let final_dir = NDir::add_direction(chunk_dir, dir);

                                    use NDir::*;
                                    surrounding_blocks[cage_index] = match final_dir {
                                        None => this_chunk
                                            .get_block(neighbor_block_pos)
                                            .map_or(false, |b| reg.is_cube(&b)),
                                        Some(North) if north_chunk.is_some() => north_chunk
                                            .unwrap()
                                            .get_block(neighbor_block_pos)
                                            .map_or(false, |b| reg.is_cube(&b)),
                                        Some(South) if south_chunk.is_some() => south_chunk
                                            .unwrap()
                                            .get_block(neighbor_block_pos)
                                            .map_or(false, |b| reg.is_cube(&b)),
                                        Some(East) if east_chunk.is_some() => east_chunk
                                            .unwrap()
                                            .get_block(neighbor_block_pos)
                                            .map_or(false, |b| reg.is_cube(&b)),
                                        Some(West) if west_chunk.is_some() => west_chunk
                                            .unwrap()
                                            .get_block(neighbor_block_pos)
                                            .map_or(false, |b| reg.is_cube(&b)),
                                        Some(NoEast) if no_east_chunk.is_some() => no_east_chunk
                                            .unwrap()
                                            .get_block(neighbor_block_pos)
                                            .map_or(false, |b| reg.is_cube(&b)),
                                        Some(NoWest) if no_west_chunk.is_some() => no_west_chunk
                                            .unwrap()
                                            .get_block(neighbor_block_pos)
                                            .map_or(false, |b| reg.is_cube(&b)),
                                        Some(SoEast) if so_east_chunk.is_some() => so_east_chunk
                                            .unwrap()
                                            .get_block(neighbor_block_pos)
                                            .map_or(false, |b| reg.is_cube(&b)),
                                        Some(SoWest) if so_west_chunk.is_some() => so_west_chunk
                                            .unwrap()
                                            .get_block(neighbor_block_pos)
                                            .map_or(false, |b| reg.is_cube(&b)),
                                        _ => false,
                                    };
                                }
                            }
                        }
                    }
                }
                apply_sl_quad(
                    mesh,
                    &metadata.vivi,
                    block_pos,
                    face,
                    surrounding_blocks,
                    sl,
                    reg.get_block_dims(),
                )
            }
        }
    }
}

pub fn get_block_n_away(
    dims: Dimensions,
    block_pos: BlockPos,
    x_change: i32,
    y_change: i32,
    z_change: i32,
) -> Option<(Option<NDir>, BlockPos)> {
    if (block_pos.y as i32 + y_change) >= dims.y as i32
        || (block_pos.y as i32 + y_change) < 0
        || x_change.abs() as u32 >= dims.x
        || z_change.abs() as u32 >= dims.z
    {
        return None;
    }

    let new_cords = [
        block_pos.x as i32 + x_change,
        block_pos.y as i32 + y_change,
        block_pos.z as i32 + z_change,
    ];
    let change = [
        (new_cords[0] as f32 / dims.x as f32).floor() as i32,
        (new_cords[2] as f32 / dims.z as f32).floor() as i32,
    ];
    let dir = from_cords_change(change.into());
    let new_cords = [
        new_cords[0].rem_euclid(dims.x as i32),
        new_cords[1],
        new_cords[2].rem_euclid(dims.z as i32),
    ];
    let new_cords: UVec3 = [
        new_cords[0] as u32,
        new_cords[1] as u32,
        new_cords[2] as u32,
    ]
    .into();

    Some((dir, new_cords))
}

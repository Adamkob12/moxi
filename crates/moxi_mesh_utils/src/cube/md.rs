//! This module defines metadata Data Structures for the meshing pipeline.
//! These will be used for:
//! - Culling (Static & Dynamic)
//! - Updating (Dynamic)
//! - Applying Smooth Lighting (Static & Dynamic)

use crate::*;

/// A struct data structure specifically for cubic meshes (made up of [`BlockMeshType::Cube`]) that
/// keeps track of which vertices belong to which block. This is used for culling & updating the mesh at run-time.
pub(crate) struct CubeVIVI {
    /// The VIVI data structure. VIVI stands for Voxel Index Vertex Index.
    pub(crate) vivi: Vec<Vec<u32>>,
    /// A map that maps a vertex to the voxel it belongs to.
    pub(crate) map: HashMap<u32, u32>,
}

impl CubeVIVI {
    /// Create a new CubeVIVI with the given voxel count.
    pub(crate) fn new(voxel_count: usize) -> CubeVIVI {
        CubeVIVI {
            vivi: vec![vec![]; voxel_count],
            map: HashMap::new(),
        }
    }

    /// Insert a quad into the CubeVIVI.
    pub(crate) fn insert_quad(&mut self, face: Face, voxel_index: usize, vertex: u32) {
        self.vivi[voxel_index].push((vertex) | face_to_u32(face));
        self.map
            .insert(vertex, voxel_index as u32 | face_to_u32(face));
    }

    /// Get the quad index of a voxel.
    pub(crate) fn get_quad_index(&self, face: Face, voxel_index: usize) -> Option<u32> {
        for quad in self.vivi[voxel_index].iter() {
            let tmp = quad & !OFFSET_CONST;
            if tmp == face_to_u32(face) {
                return Some(quad & (OFFSET_CONST));
            }
        }
        None
    }

    /// Change the quad index of a voxel.
    pub(crate) fn change_quad_index(&mut self, old_vertex: usize, new_vertex: usize) {
        let voxel = self
            .map
            .remove(&(old_vertex as u32))
            .expect(format!("Couldn't find voxel matching vertex {}", old_vertex).as_str());
        let q = voxel & !OFFSET_CONST;
        let v = voxel & OFFSET_CONST;
        let old_vertex = old_vertex as u32 | q;
        for v in self.vivi[v as usize].iter_mut() {
            if *v == old_vertex {
                *v = new_vertex as u32 | q;
                self.map.insert(new_vertex as u32, voxel);
                return;
            }
        }
        panic!("Couldn't find vertex index in VIVI");
    }

    /// Remove a quad from the CubeVIVI.
    pub(crate) fn remove_quad(&mut self, old_vertex: usize) {
        let voxel = self
            .map
            .remove(&(old_vertex as u32))
            .expect("Couldn't find voxel matching vertex");
        let q = voxel & !OFFSET_CONST;
        let v = voxel & OFFSET_CONST;
        let old_vertex = old_vertex as u32 | q;
        let mut r = (false, 0);
        for (i, j) in self.vivi[v as usize].iter().enumerate() {
            if *j == old_vertex {
                r = (true, i);
            }
        }
        if r.0 {
            self.vivi[v as usize].swap_remove(r.1);
        } else {
            panic!("Couldn't find quad from vertex");
        }
    }
}

/// Mesh meta-data struct for cubic meshes (made up of [`BlockMeshType::Cube`]).
/// Holds all the information needed to update the mesh at run-time.
pub struct CubeMD<B: BlockInGrid> {
    pub(crate) vivi: CubeVIVI,
    pub(crate) smooth_lighting_params: Option<SmoothLightingParameters>,
    /// The dimensions of the 3d grid.
    pub dims: Dimensions,
    /// [`B`](`BlockInGrid`): The block
    /// [`BlockPos`]: Th position of the block in the grid
    /// [`BlockMeshChange`]: The change that happened to the block mesh
    /// [`SurroundingBlocks<B>`]: The blocks surrounding the block that changed
    /// in the `Neighbors` data-type, if the voxel is "empty"- None.
    pub(crate) changed_voxels: Vec<(B, BlockPos, BlockMeshChange, SurroundingBlocks<B>)>,
}

impl<B: BlockInGrid> CubeMD<B> {
    /// Log the changes to the voxels.
    /// `voxel_change`: [`BlockMeshChange`], Added or broken.
    /// `voxel_index`: the index of the voxel in the 1-dimensional grid.
    /// `voxel`: The voxel itself, same type as in the voxel registry.
    /// `neighboring_voxels`: Array where each element is the voxel in that direction.
    ///     (see Face from usize to understand which index represents which direction)
    /// Adding a voxel that already exists, or breaking one that doesn't is undefined behaviour.
    pub fn log(
        &mut self,
        block_mesh_change: BlockMeshChange,
        block_pos: BlockPos,
        block: B,
        surrounding_blocks: SurroundingBlocks<B>,
    ) {
        self.changed_voxels
            .push((block, block_pos, block_mesh_change, surrounding_blocks));
    }
    /// Get read only of the `SmoothLightingParameters`
    pub fn get_sl_params(&self) -> Option<SmoothLightingParameters> {
        self.smooth_lighting_params
    }

    pub fn quad_exists(&self, block_pos: BlockPos, quad: Face) -> bool {
        self.vivi
            .get_quad_index(quad, pos_to_index(block_pos, self.dims).unwrap())
            .is_some()
    }
}

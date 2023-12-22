//! Utils that have to do with chunks, and blocks
use crate::prelude::*;
use bevy_math::prelude::*;

/// Dimensions of a chunk, (width, height, length)
pub type Dimensions = UVec3;
/// Chunk coordinates, (x, z)
pub type ChunkCords = IVec2;
/// Chunk grid
pub struct Grid<T: BlockInGrid, const N: usize> {
    pub dims: Dimensions,
    grid: [T; N],
}
/// The position of the block in the chunk (x, y, z)
pub type BlockPos = UVec3;
/// The index of the block in the chunk
type BlockIndex = usize;
/// The position of the block in the world
#[derive(Copy, Clone)]
pub struct BlockGlobalPos {
    pub pos: BlockPos,
    pub cords: ChunkCords,
    pub valid: bool,
}
/// The translation of the block in the world
pub type BlockTrans = Vec3;

impl BlockGlobalPos {
    pub fn new(block_pos: BlockPos, chunk_cords: ChunkCords) -> Self {
        Self {
            pos: block_pos,
            cords: chunk_cords,
            valid: true,
        }
    }
}

/// Data type to represent the 6 neighbors of a block, one for each face (side), in the order:
/// top, bottom, right, left, back, front
/// Can also be indexed by a [`Face`]. If the block is on the edge of the chunk, the neighbor
/// towards that direction (out of bounds of the chunk) will be None.
pub type SurroundingBlocks<T> = [Option<T>; 6];

impl<B> std::ops::Index<Face> for SurroundingBlocks<B> {
    type Output = Option<B>;

    fn index(&self, face: Face) -> &Self::Output {
        match face {
            Face::Top => &self[0],
            Face::Bottom => &self[1],
            Face::Right => &self[2],
            Face::Left => &self[3],
            Face::Back => &self[4],
            Face::Front => &self[5],
        }
    }
}

impl<B> std::ops::IndexMut<Face> for SurroundingBlocks<B> {
    fn index_mut(&mut self, face: Face) -> &mut Self::Output {
        match face {
            Face::Top => &mut self[0],
            Face::Bottom => &mut self[1],
            Face::Right => &mut self[2],
            Face::Left => &mut self[3],
            Face::Back => &mut self[4],
            Face::Front => &mut self[5],
        }
    }
}

pub trait SurroundingBlocksCommon<B> {
    fn top(&self) -> &Option<B>;
    fn bottom(&self) -> &Option<B>;
    fn right(&self) -> &Option<B>;
    fn left(&self) -> &Option<B>;
    fn back(&self) -> &Option<B>;
    fn front(&self) -> &Option<B>;
    fn with_face(self, face: Face, block: B) -> Self;
    fn uniform(block: B) -> Self;
}

impl<B: BlockInGrid> SurroundingBlocksCommon<B> for SurroundingBlocks<B> {
    fn top(&self) -> &Option<B> {
        &self[0]
    }

    fn bottom(&self) -> &Option<B> {
        &self[1]
    }

    fn right(&self) -> &Option<B> {
        &self[2]
    }

    fn left(&self) -> &Option<B> {
        &self[3]
    }

    fn back(&self) -> &Option<B> {
        &self[4]
    }

    fn front(&self) -> &Option<B> {
        &self[5]
    }

    fn with_face(mut self, face: Face, block: B) -> Self {
        self[face] = Some(block);
        self
    }

    fn uniform(block: B) -> Self {
        [Some(block); 6]
    }
}

impl<T: BlockInGrid, const N: usize> Grid<T, N> {
    pub fn get_block(&self, block_pos: BlockPos) -> Option<T> {
        pos_to_index(block_pos, self.dims).map(|i| self.grid[i])
    }

    pub fn get_block_mut<'a>(&'a mut self, block_pos: BlockPos) -> Option<&'a mut T> {
        pos_to_index(block_pos, self.dims).map(|i| &mut self.grid[i])
    }

    pub fn get_block_or(&self, block_pos: BlockPos, default: T) -> T {
        pos_to_index(block_pos, self.dims).map_or(default, |i| self.grid[i])
    }

    pub fn get_neighbor_of(&self, block_pos: BlockPos, face: Face) -> Option<T> {
        neighbor_index(block_pos, face, self.dims).map(|i| self.grid[i])
    }

    pub fn get_neighbor_of_or(&self, block_pos: BlockPos, face: Face, default: T) -> T {
        self.get_neighbor_of(block_pos, face).unwrap_or(default)
    }

    pub fn enumerate_blocks_on_edge(&self, edge: Face) -> impl Iterator<Item = (BlockPos, T)> + '_ {
        iter_blocks_on_edge(edge, self.dims).map(|pos| (pos, self.get_block(pos).unwrap()))
    }

    pub fn iter_blocks_on_edge(&self, edge: Face) -> impl Iterator<Item = BlockPos> {
        iter_blocks_on_edge(edge, self.dims)
    }

    pub fn enumerate_blocks(&self) -> impl Iterator<Item = (BlockPos, T)> + '_ {
        (0..(self.dims.x * self.dims.y * self.dims.z))
            .into_iter()
            .map(|i| {
                let pos = index_to_pos(i as BlockIndex, self.dims).unwrap();
                (pos, self.get_block(pos).unwrap())
            })
    }

    pub fn get_neighbors(&self, block_pos: BlockPos) -> SurroundingBlocks<T> {
        let mut neighbors = [None; 6];
        for (i, face) in FACES.iter().enumerate() {
            neighbors[i] = self.get_neighbor_of(block_pos, *face);
        }
        neighbors
    }

    pub fn get_neighbors_or(&self, block_pos: BlockPos, default: T) -> [T; 6] {
        let mut neighbors = [default; 6];
        for (i, face) in FACES.iter().enumerate() {
            neighbors[i] = self.get_neighbor_of_or(block_pos, *face, default);
        }
        neighbors
    }

    pub fn enumerate_neighbors(&self, block_pos: BlockPos) -> [(Face, Option<T>); 6] {
        let mut neighbors = [(Face::Top, None); 6];
        for (i, face) in FACES.iter().enumerate() {
            neighbors[i] = (*face, self.get_neighbor_of(block_pos, *face));
        }
        neighbors
    }

    pub fn len(&self) -> usize {
        (self.dims.x * self.dims.y * self.dims.z) as usize
    }

    pub const fn new(grid: [T; N], dims: Dimensions) -> Self {
        Self { dims, grid }
    }

    pub fn set_block(&mut self, block: T, block_pos: BlockPos) -> Result<(), ()> {
        if let Some(block_index) = pos_to_index(block_pos, self.dims) {
            self.grid[block_index] = block;
            return Ok(());
        }
        Err(())
    }
}

pub fn neighbor_across_chunk(
    mut block_pos: BlockPos,
    face: Face,
    dims: Dimensions,
) -> Option<BlockPos> {
    if is_block_pos_on_edge(block_pos, face, dims) && pos_in_bounds(block_pos, dims) {
        return match face {
            Face::Right => Some({
                block_pos.x = 0;
                block_pos
            }),
            Face::Left => Some({
                block_pos.x = dims.x - 1;
                block_pos
            }),
            Face::Back => Some({
                block_pos.z = 0;
                block_pos
            }),
            Face::Front => Some({
                block_pos.z = dims.z - 1;
                block_pos
            }),
            _ => None,
        };
    }
    None
}

pub fn enumerate_neighbors_across_chunks(
    block_pos: BlockPos,
    dims: Dimensions,
) -> impl Iterator<Item = (Face, BlockPos)> {
    FACES.iter().filter_map(move |&face| {
        neighbor_across_chunk(block_pos, face, dims).map(|pos| (face, pos))
    })
}

pub fn iter_blocks_on_edge(edge: Face, dims: Dimensions) -> impl Iterator<Item = BlockPos> {
    (0..(dims.x * dims.y * dims.z))
        .into_iter()
        .filter_map(move |i| {
            if is_block_index_on_edge(i as BlockIndex, edge, dims) {
                Some(index_to_pos(i as BlockIndex, dims).unwrap())
            } else {
                None
            }
        })
        .into_iter()
}

pub fn is_block_pos_on_edge(block_pos: BlockPos, edge: Face, dims: Dimensions) -> bool {
    neighbor_pos(block_pos, edge, dims).is_none()
}

fn is_block_index_on_edge(block_index: BlockIndex, edge: Face, dims: Dimensions) -> bool {
    index_to_pos(block_index, dims).map_or(false, |pos| is_block_pos_on_edge(pos, edge, dims))
}

pub const fn index_to_pos(block_index: BlockIndex, dims: Dimensions) -> Option<BlockPos> {
    let height = dims.y;
    let length = dims.z;
    let width = dims.x;

    let h = (block_index as u32 / (length * width)) as u32;
    let l = ((block_index as u32 - h * (length * width)) / width) as u32;
    let w = (block_index as u32 - h * (length * width) - l * width) as u32;

    if w > width || h > height || l > length {
        return None;
    }

    Some(UVec3::new(w, h, l))
}

pub const fn pos_in_bounds(block_pos: UVec3, dims: Dimensions) -> bool {
    !(block_pos.x >= dims.x || block_pos.y >= dims.y || block_pos.z >= dims.z)
}

pub const fn pos_to_index(block_pos: UVec3, dims: Dimensions) -> Option<BlockIndex> {
    if block_pos.x >= dims.x || block_pos.y >= dims.y || block_pos.z >= dims.z {
        None
    } else {
        Some((block_pos.y * (dims.x * dims.z) + block_pos.z * dims.x + block_pos.x) as BlockIndex)
    }
}

fn neighbor_index(block_pos: BlockPos, face: Face, dims: Dimensions) -> Option<BlockIndex> {
    neighbor_pos(block_pos, face, dims)
        .map(|pos| pos_to_index(pos, dims))
        .flatten()
}

pub fn neighbor_pos(mut block_pos: BlockPos, face: Face, dims: Dimensions) -> Option<BlockPos> {
    match face {
        Face::Top => block_pos.y += 1,
        Face::Bottom => block_pos.y = block_pos.y.checked_sub(1)?,
        Face::Right => block_pos.x += 1,
        Face::Left => block_pos.x = block_pos.x.checked_sub(1)?,
        Face::Back => block_pos.z += 1,
        Face::Front => block_pos.z = block_pos.z.checked_sub(1)?,
    }
    if pos_in_bounds(block_pos, dims) {
        Some(block_pos)
    } else {
        None
    }
}

/// Assumes each block is [1, 1, 1]
pub fn point_to_chunk_cords(point: Vec3, chunk_dims: Dimensions) -> ChunkCords {
    let chunk_width = chunk_dims.x;
    let chunk_length = chunk_dims.z;
    let x = point.x + 0.5;
    let z = point.z + 0.5;
    [
        (x / chunk_width as f32 + (x.signum() - 1.0) / 2.0) as i32,
        (z / chunk_length as f32 + (z.signum() - 1.0) / 2.0) as i32,
    ]
    .into()
}

/// the bool is for whether or not the pos is within the height bounds
pub fn point_to_global_block_pos(point: Vec3, chunk_dims: Dimensions) -> BlockGlobalPos {
    let chunk_width = chunk_dims.x;
    let chunk_length = chunk_dims.z;
    let chunk_height = chunk_dims.y;
    let chunk_cords = point_to_chunk_cords(point, chunk_dims);

    let x = point.x + 0.5;
    let z = point.z + 0.5;
    let y = point.y + 0.5;

    let block_pos = [
        (x - chunk_cords[0] as f32 * chunk_width as f32) as u32,
        (y as u32).max(0).min(chunk_height),
        (z - chunk_cords[1] as f32 * chunk_length as f32) as u32,
    ];

    let flag = y >= 0.0 && y <= chunk_height as f32;
    BlockGlobalPos {
        pos: block_pos.into(),
        cords: chunk_cords,
        valid: flag,
    }
}

pub fn global_block_pos_to_block_trans(
    global_pos: BlockGlobalPos,
    block_dims: Vec3,
    dims: Dimensions,
) -> BlockTrans {
    let [u, v] = [
        global_pos.cords.x as f32 * dims.x as f32,
        global_pos.cords.y as f32 * dims.z as f32,
    ];

    block_dims
        * Vec3::new(
            u + global_pos.pos.x as f32,
            global_pos.pos.y as f32,
            v + global_pos.pos.z as f32,
        )
}

pub fn global_enumerate_neighboring_blocks(
    global_pos: BlockGlobalPos,
    dims: Dimensions,
) -> impl Iterator<Item = (Face, BlockGlobalPos)> {
    FACES
        .iter()
        .map(move |&face| (face, global_neighbor(global_pos, face, dims)))
}

pub fn global_neighbor(
    mut global_pos: BlockGlobalPos,
    face: Face,
    dims: Dimensions,
) -> BlockGlobalPos {
    if let Some(neighbor_pos) = neighbor_pos(global_pos.pos, face, dims) {
        return BlockGlobalPos {
            pos: neighbor_pos,
            cords: global_pos.cords,
            valid: dims.y > neighbor_pos.y,
        };
    } else if let Some(neighbor_pos) = neighbor_across_chunk(global_pos.pos, face, dims) {
        return BlockGlobalPos {
            pos: neighbor_pos,
            cords: adj_chunk(global_pos.cords, face),
            valid: dims.y > neighbor_pos.y,
        };
    }
    global_pos.valid = false;
    global_pos
}

pub fn adj_chunk(chunk_cords: ChunkCords, face: Face) -> ChunkCords {
    match face {
        Face::Top | Face::Bottom => chunk_cords,
        Face::Back => chunk_cords + IVec2::from([0, 1]),
        Face::Front => chunk_cords - IVec2::from([0, 1]),
        Face::Right => chunk_cords + IVec2::from([1, 0]),
        Face::Left => chunk_cords - IVec2::from([1, 0]),
    }
}

pub fn chunk_distance(chunk1: ChunkCords, chunk2: ChunkCords) -> i32 {
    (chunk1.x - chunk2.x).abs().max((chunk1.y - chunk2.y).abs())
}

use super::*;
use crate::*;

/// A struct data structure specifically for `custom` chunk meshes (made up of [`BlockMeshType::Custom`] blocks) that
/// keeps track of which vertices and indices belong to which block. This is used for updating the mesh at run-time.
pub(crate) type CustomVIVI = HashMap<BlockPos, (VertexIndex, IndexIndex)>;

/// Mesh meta-data struct for xsprite meshes (made up of [`BlockMeshType::XSprite`]).
/// Holds all the information needed to update the mesh at run-time.
pub struct CustomMD<B: BlockInGrid> {
    pub(crate) vivi: CustomVIVI,
    pub(crate) log: Vec<(BlockMeshChange, B, BlockPos)>,
    pub(crate) dims: Dimensions,
}

impl<B: BlockInGrid> CustomMD<B> {
    /// Log a block break
    pub fn log_break(&mut self, block: B, pos: BlockPos) {
        self.log.push((BlockMeshChange::Broken, block, pos));
    }

    /// Log a block add
    pub fn log_add(&mut self, block: B, pos: BlockPos) {
        self.log.push((BlockMeshChange::Added, block, pos));
    }
}

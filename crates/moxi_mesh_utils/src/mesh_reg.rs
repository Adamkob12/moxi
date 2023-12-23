use bevy_asset::Handle;

use crate::{
    block_mesh::{BlockMeshRef, BlockMeshType},
    *,
};

/// A registry for [`Mesh`]s, and their associated [`AssetId`]s.
/// When initializing the world, The [`BPT`] system will use [`Bevy Reflect`](https://docs.rs/bevy/latest/bevy/reflect/index.html)
/// To generate this registry at run time from all of the block's properties.
pub trait MeshRegistry<B: BlockInGrid> {
    /// Returns the a block mesh reference [`BlockMeshRef`] of the block
    fn get_block_mesh_ref(&self, block: &B) -> BlockMeshRef;

    /// Returns the [`AssetId`] of the block's mesh.
    fn get_block_mesh_handle(&self, block: &B) -> Handle<Mesh>;

    /// Return the [`BlockMeshType`] of the block mesh.
    fn get_block_mesh_type(&self, block: &B) -> BlockMeshType;
}

/// Trait of common functions for [`MeshRegistry`]s.
pub trait MeshRegistryCommon<B: BlockInGrid> {
    const DEFAULT_MESH: BlockMeshRef<'static>;
    const BLOCK_DIMS: [f32; 3];
    const BLOCK_CENTER: [f32; 3];
    const ALL_ATTRIBUTES: &'static [MeshVertexAttribute];
    const CUSTOM_ATTRIBUTES: &'static [MeshVertexAttribute];
    /// Return the amount of vertices in a block mesh.
    fn get_block_mesh_vertex_count(&self, block: &B) -> usize;

    /// Return the amount of indices in a block mesh.
    fn get_block_mesh_indices_len(&self, block: &B) -> usize;

    fn get_block_mesh_aabb(&self, block: &B) -> Aabb;

    fn is_air(&self, block: &B) -> bool;

    fn is_custom(&self, block: &B) -> bool;

    fn is_xsprite(&self, block: &B) -> bool;

    fn is_cube(&self, block: &B) -> bool;

    fn get_default_mesh(&self) -> BlockMeshRef<'static> {
        Self::DEFAULT_MESH
    }

    fn get_block_dims(&self) -> [f32; 3] {
        Self::BLOCK_DIMS
    }

    fn get_block_center(&self) -> [f32; 3] {
        Self::BLOCK_CENTER
    }

    fn custom_attributes(&self) -> &'static [MeshVertexAttribute] {
        Self::CUSTOM_ATTRIBUTES
    }

    fn all_attributes(&self) -> &'static [MeshVertexAttribute] {
        Self::ALL_ATTRIBUTES
    }
}

impl<B: BlockInGrid, M: MeshRegistry<B>> MeshRegistryCommon<B> for M {
    const DEFAULT_MESH: BlockMeshRef<'static> = BlockMeshRef::Air;
    const BLOCK_DIMS: [f32; 3] = [1.0, 1.0, 1.0];
    const BLOCK_CENTER: [f32; 3] = [0.0, 0.0, 0.0];
    const ALL_ATTRIBUTES: &'static [MeshVertexAttribute] = &[
        Mesh::ATTRIBUTE_POSITION,
        Mesh::ATTRIBUTE_NORMAL,
        Mesh::ATTRIBUTE_UV_0,
        Mesh::ATTRIBUTE_COLOR,
    ];
    const CUSTOM_ATTRIBUTES: &'static [MeshVertexAttribute] = &[
        Mesh::ATTRIBUTE_POSITION,
        Mesh::ATTRIBUTE_NORMAL,
        Mesh::ATTRIBUTE_COLOR,
    ];

    fn get_block_mesh_aabb(&self, block: &B) -> Aabb {
        self.get_block_mesh_ref(block).get_aabb()
    }

    fn get_block_mesh_indices_len(&self, block: &B) -> usize {
        self.get_block_mesh_ref(block).get_indices_count()
    }

    fn get_block_mesh_vertex_count(&self, block: &B) -> usize {
        self.get_block_mesh_ref(block).get_vertex_count()
    }

    fn is_air(&self, block: &B) -> bool {
        self.get_block_mesh_ref(block).is_air()
    }

    fn is_custom(&self, block: &B) -> bool {
        self.get_block_mesh_ref(block).is_custom()
    }

    fn is_xsprite(&self, block: &B) -> bool {
        self.get_block_mesh_ref(block).is_xsprite()
    }

    fn is_cube(&self, block: &B) -> bool {
        self.get_block_mesh_ref(block).is_cube()
    }
}

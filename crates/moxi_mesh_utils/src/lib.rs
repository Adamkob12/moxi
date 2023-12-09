//! This crate provides is essentially the meshing backend of moxi.
//! It provides:
//! - A meshing pipeline that can be used to generate meshes from block data.
//! - Meshing Algorithms to generate & cull chunk meshes.
//! - A way to update chunk meshes based on block updates.
//! - Static Frustum Culling for chunks.

pub(crate) use bevy_asset::AssetId;
pub(crate) use bevy_math::{UVec3, Vec3};
pub(crate) use bevy_render::mesh::{
    Indices, Mesh, MeshVertexAttribute, PrimitiveTopology, VertexAttributeValues,
};
pub(crate) use bevy_render::{primitives::Aabb, render_resource::VertexFormat};
pub(crate) use bevy_utils::HashMap;
pub(crate) use moxi_utils::prelude::*;

mod adj;
mod block_mesh;
mod cube;
mod custom;
mod mesh_reg;
mod sl;
mod vav_utils;
mod xsprite;

pub(crate) use block_mesh::*;
pub(crate) use cube::*;
pub(crate) use mesh_reg::*;
pub(crate) use sl::*;
pub(crate) use vav_utils::*;

pub const CHUNK_DIMS: Dimensions = Dimensions::new(16, 64, 16);

/// This enum represents all the way a voxel could be changed.
#[derive(Clone, Copy)]
pub enum BlockMeshChange {
    Broken,
    Added,
    CullFaces,
    AddFaces,
}

pub mod prelude {
    pub use super::adj::*;
    pub use super::block_mesh::*;
    pub use super::cube::*;
    pub use super::custom::*;
    pub use super::mesh_reg::*;
    pub use super::sl::*;
    pub use super::xsprite::*;
}

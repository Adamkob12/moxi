//! This crate provides is essentially the meshing backend of moxi.
//! It provides:
//! - A meshing pipeline that can be used to generate meshes from block data.
//! - Meshing Algorithms to generate & cull chunk meshes.
//! - A way to update chunk meshes based on block updates.
//! - Static Frustum Culling for chunks.

pub(crate) use bevy_asset::AssetId;
pub(crate) use bevy_math::{UVec3, Vec3};
pub(crate) use bevy_render::mesh::{Indices, Mesh, PrimitiveTopology, VertexAttributeValues};
pub(crate) use bevy_render::primitives::Aabb;
pub(crate) use bevy_utils::HashMap;
pub(crate) use moxi_utils::prelude::*;

mod block_mesh;
mod cube;
mod custom;
mod mesh_reg;
mod sl;
mod xsprite;

pub(crate) use block_mesh::*;
pub(crate) use cube::*;
pub(crate) use custom::*;
pub(crate) use mesh_reg::*;
pub(crate) use sl::*;
pub(crate) use xsprite::*;

pub const CHUNK_DIMS: Dimensions = Dimensions::new(16, 64, 16);

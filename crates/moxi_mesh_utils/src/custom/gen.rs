//! This module contains the function that generates a cubic mesh.

use crate::{block_mesh::BlockMesh, *};

/// In order to wrap a custom mesh that implements [`Into<Mesh>`] in a [`BlockMesh`]
/// The user must first wrap the mesh in this struct, and call [`into()`](`Into::into`)
pub struct MeshCaster<T: Into<Mesh>>(pub T);

impl<T: Into<Mesh>> Into<BlockMesh> for MeshCaster<T> {
    fn into(self) -> BlockMesh {
        BlockMesh::Custom(self.0.into())
    }
}

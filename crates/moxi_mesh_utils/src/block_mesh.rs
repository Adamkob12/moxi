pub use bevy_math::Vec3A;

use crate::*;

const EMPTY_AABB: Aabb = Aabb {
    center: Vec3A::ZERO,
    half_extents: Vec3A::ZERO,
};

/// An enum that represents the different types of meshes that can be used to represent a block.
/// This enum will not be in any SDK, because the user will be able to define incompatible
/// BlockMeshes, for example, a custom mesh inside a [`BlockMeshType::Cube`].
#[derive(Reflect)]
pub enum BlockMesh {
    /// [`BlockMeshType::Cube`]
    Cube(Mesh),
    /// [`BlockMeshType::Custom`]
    Custom(Mesh),
    /// [`BlockMeshType::XSprite`]
    XSprite(Mesh),
    /// [`BlockMeshType::Air`]
    Air,
}

impl BlockMesh {
    pub fn from_custom_mesh<T: Into<Mesh>>(mesh: T) -> Self {
        BlockMesh::Custom(mesh.into())
    }

    pub fn xsprite_from_texture_cords() -> Self {
        unimplemented!()
    }

    pub fn cube_from_texture_cords() -> Self {
        unimplemented!()
    }

    pub fn as_ref<'a>(&'a self) -> BlockMeshRef<'a> {
        match self {
            BlockMesh::Cube(mesh) => BlockMeshRef::Cube(mesh),
            BlockMesh::Custom(mesh) => BlockMeshRef::Custom(mesh),
            BlockMesh::XSprite(mesh) => BlockMeshRef::XSprite(mesh),
            BlockMesh::Air => BlockMeshRef::Air,
        }
    }

    pub fn get_type(&self) -> BlockMeshType {
        match self {
            BlockMesh::Cube(_) => BlockMeshType::Cube,
            BlockMesh::Custom(_) => BlockMeshType::Custom,
            BlockMesh::XSprite(_) => BlockMeshType::XSprite,
            BlockMesh::Air => BlockMeshType::Air,
        }
    }
}

/// An average intelligence pointer to a [`BlockMesh`].
#[derive(Reflect)]
pub enum BlockMeshRef<'a> {
    /// [`BlockMeshType::Cube`]
    Cube(&'a Mesh),
    /// [`BlockMeshType::Custom`]
    Custom(&'a Mesh),
    /// [`BlockMeshType::XSprite`]
    XSprite(&'a Mesh),
    /// [`BlockMeshType::Air`]
    Air,
}

/// An enum that represents the different types of meshes that can be used to represent a block.
#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockMeshType {
    /// A cube mesh. The most normal type of block mesh. Most blocks will use this.
    Cube,
    /// A custom mesh. This is a mesh that the user can express in code, meaning not
    /// an imported 3d model. For example, a custom mesh could be a sphere or a cylinder
    /// that the user can easily define using [`Bevy's shapes`](https://docs.rs/bevy/latest/bevy/prelude/shape/index.html).
    Custom,
    /// An xsprite is the result of two (double sided) sprites being placed in an X shape.
    /// For example, a flower in Minecraft is an xsprite.
    XSprite,
    /// Air is a special type of block mesh that is used to represent empty space.
    Air,
}

impl<'a> BlockMeshRef<'a> {
    /// Returns the type of the mesh.
    pub fn get_type(&self) -> BlockMeshType {
        match self {
            BlockMeshRef::Cube(_) => BlockMeshType::Cube,
            BlockMeshRef::Custom(_) => BlockMeshType::Custom,
            BlockMeshRef::XSprite(_) => BlockMeshType::XSprite,
            BlockMeshRef::Air => BlockMeshType::Air,
        }
    }

    /// Returns true if the mesh is air.
    pub fn is_air(&self) -> bool {
        match self {
            BlockMeshRef::Air => true,
            _ => false,
        }
    }

    /// Returns true if the mesh is a cube.
    pub fn is_cube(&self) -> bool {
        match self {
            BlockMeshRef::Cube(_) => true,
            _ => false,
        }
    }

    /// Returns true if the mesh is a custom mesh.
    pub fn is_custom(&self) -> bool {
        match self {
            BlockMeshRef::Custom(_) => true,
            _ => false,
        }
    }

    /// Returns true if the mesh is an xsprite.
    pub fn is_xsprite(&self) -> bool {
        match self {
            BlockMeshRef::XSprite(_) => true,
            _ => false,
        }
    }

    pub fn get_if(&self, mesh_type: BlockMeshType) -> Option<&'a Mesh> {
        match self {
            BlockMeshRef::Cube(mesh) if mesh_type == BlockMeshType::Cube => Some(mesh),
            BlockMeshRef::Custom(mesh) if mesh_type == BlockMeshType::Custom => Some(mesh),
            BlockMeshRef::XSprite(mesh) if mesh_type == BlockMeshType::XSprite => Some(mesh),
            _ => None,
        }
    }

    /// Returns the AABB of the mesh. If air, returns an empty AABB.
    pub fn get_aabb(&self) -> Aabb {
        match self {
            BlockMeshRef::Cube(mesh) => mesh
                .compute_aabb()
                .expect("Failed to compute AABB for cube"),
            BlockMeshRef::Custom(mesh) => mesh
                .compute_aabb()
                .expect("Failed to compute AABB for custom mesh"),
            BlockMeshRef::XSprite(mesh) => mesh
                .compute_aabb()
                .expect("Failed to compute AABB for xsprite"),
            BlockMeshRef::Air => EMPTY_AABB,
        }
    }

    /// Get vertices count
    pub fn get_vertex_count(&self) -> usize {
        match self {
            BlockMeshRef::Cube(mesh) => mesh.count_vertices(),
            BlockMeshRef::Custom(mesh) => mesh.count_vertices(),
            BlockMeshRef::XSprite(mesh) => mesh.count_vertices(),
            BlockMeshRef::Air => 0,
        }
    }

    /// Get indices count (not number of traingles, number of indices)
    pub fn get_indices_count(&self) -> usize {
        match self {
            BlockMeshRef::Cube(mesh) => mesh.indices().map_or(0, |i| i.len()),
            BlockMeshRef::Custom(mesh) => mesh.indices().map_or(0, |i| i.len()),
            BlockMeshRef::XSprite(mesh) => mesh.indices().map_or(0, |i| i.len()),
            BlockMeshRef::Air => 0,
        }
    }

    /// Get the mesh as an option.
    pub fn as_option(self) -> Option<&'a Mesh> {
        match self {
            BlockMeshRef::Cube(mesh) => Some(mesh),
            BlockMeshRef::Custom(mesh) => Some(mesh),
            BlockMeshRef::XSprite(mesh) => Some(mesh),
            BlockMeshRef::Air => None,
        }
    }

    /// Unwrap the mesh.
    pub fn unwrap(self) -> &'a Mesh {
        match self {
            BlockMeshRef::Cube(mesh) => mesh,
            BlockMeshRef::Custom(mesh) => mesh,
            BlockMeshRef::XSprite(mesh) => mesh,
            BlockMeshRef::Air => panic!("Called unwrap on air mesh"),
        }
    }

    /// Unwrap the mesh with a custom error message.
    pub fn expect(self, msg: &str) -> &'a Mesh {
        match self {
            BlockMeshRef::Cube(mesh) => mesh,
            BlockMeshRef::Custom(mesh) => mesh,
            BlockMeshRef::XSprite(mesh) => mesh,
            BlockMeshRef::Air => panic!("{}", msg),
        }
    }
}

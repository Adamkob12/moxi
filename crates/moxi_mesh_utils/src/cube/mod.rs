mod gen;
mod md;
mod meshify;
mod update;

pub use gen::*;
pub use md::*;
pub use meshify::*;
pub use update::*;

use crate::Face;

pub struct CubeTextureCords {
    pub top: AtlasCords,
    pub bottom: AtlasCords,
    pub right: AtlasCords,
    pub left: AtlasCords,
    pub back: AtlasCords,
    pub front: AtlasCords,
}

use crate::AtlasCords;
impl CubeTextureCords {
    pub const fn uniform(dims: AtlasCords) -> Self {
        Self {
            top: dims,
            bottom: dims,
            right: dims,
            left: dims,
            back: dims,
            front: dims,
        }
    }

    pub fn with_face(mut self, face: Face, dims: AtlasCords) -> Self {
        match face {
            Face::Top => self.top = dims,
            Face::Bottom => self.bottom = dims,
            Face::Right => self.right = dims,
            Face::Left => self.left = dims,
            Face::Back => self.back = dims,
            Face::Front => self.front = dims,
        }
        self
    }
}

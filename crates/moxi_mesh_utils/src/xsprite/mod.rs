mod gen;
mod md;
mod meshify;
mod update;

pub use gen::*;
pub use md::*;
pub use meshify::*;
pub use update::*;

use crate::AtlasCords;
impl XSpriteTextureCords {
    pub const fn uniform(dims: AtlasCords) -> Self {
        Self { sprite: dims }
    }
}

pub struct XSpriteTextureCords {
    pub sprite: AtlasCords,
}

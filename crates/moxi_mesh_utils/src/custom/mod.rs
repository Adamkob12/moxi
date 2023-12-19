mod gen;
mod md;
mod meshify;
mod update;

pub(self) type VertexIndex = u32;
pub(self) type IndexIndex = u32;

pub use gen::*;
pub use md::*;
pub use meshify::*;
pub use update::*;

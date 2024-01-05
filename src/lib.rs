pub mod prelude {
    pub use moxi_bpta::prelude::app::MoxiApp;
    pub use moxi_bpta::prelude::*;
    pub use moxi_derive::config_from_dimensions;
    pub use moxi_utils::prelude::*;
}

pub mod physics {
    pub use moxi_physics::*;
}

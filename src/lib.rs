mod minimal_sdk {}

mod standard_sdk {}

mod complete_sdk {}

pub mod prelude {
    #[cfg(feature = "complete_sdk")]
    pub use crate::complete_sdk::*;
    #[cfg(feature = "minimal_sdk")]
    pub use crate::minimal_sdk::*;
    #[cfg(feature = "standard_sdk")]
    pub use crate::standard_sdk::*;
}

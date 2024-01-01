/// A macro to configure a lot of types that take a generic const using user defined dimensions.
#[macro_export]
macro_rules! config_from_dimensions {
    ($dims:ident) => {
        pub const _CHUNK_DIMS: Dimensions = $dims;
        pub const BLOCKS_IN_CHUNK: usize =
            _CHUNK_DIMS.x as usize * _CHUNK_DIMS.y as usize * _CHUNK_DIMS.z as usize;
        pub type Blocks<'w, 's> = moxi::prelude::_Blocks<'w, 's, BLOCKS_IN_CHUNK>;
        pub type BlocksMut<'w, 's> = moxi::prelude::_BlocksMut<'w, 's, BLOCKS_IN_CHUNK>;
        pub struct MoxiBptPlugin(moxi::prelude::_MoxiBptPlugin<BLOCKS_IN_CHUNK>);
        impl Plugin for MoxiBptPlugin {
            fn build(&self, app: &mut App) {
                self.0.build(app);
                unsafe {
                    PLACEHOLDER_DIMS = _CHUNK_DIMS;
                }
            }
        }
        impl Default for MoxiBptPlugin {
            fn default() -> Self {
                Self(moxi::prelude::_MoxiBptPlugin::<BLOCKS_IN_CHUNK>)
            }
        }
        impl std::ops::Deref for MoxiBptPlugin {
            type Target = moxi::prelude::_MoxiBptPlugin<BLOCKS_IN_CHUNK>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

#[macro_export]
macro_rules! config_physics_from_dimensions {
    ($dims:ident) => {
        pub struct MoxiPhysicsPlugin(moxi::physics::_MoxiPhysicsPlugin<BLOCKS_IN_CHUNK>);
        impl Plugin for MoxiPhysicsPlugin {
            fn build(&self, app: &mut App) {
                self.0.build(app);
            }
        }
        impl Default for MoxiPhysicsPlugin {
            fn default() -> Self {
                Self(moxi::physics::_MoxiPhysicsPlugin::<BLOCKS_IN_CHUNK>::default())
            }
        }
        impl std::ops::Deref for MoxiPhysicsPlugin {
            type Target = moxi::physics::_MoxiPhysicsPlugin<BLOCKS_IN_CHUNK>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl std::ops::DerefMut for MoxiPhysicsPlugin {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

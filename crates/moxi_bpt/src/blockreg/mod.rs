pub(crate) mod meshreg;
use bevy_ecs::prelude::Resource;
use std::collections::HashSet;

#[derive(Resource, Default)]
pub struct BlockRegistry {
    pub(crate) names: HashSet<&'static str>,
}

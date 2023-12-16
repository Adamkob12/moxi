pub(crate) mod meshreg;
use bevy_ecs::prelude::Resource;
use moxi_utils::prelude::BlockId;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct BlockRegistry(pub(crate) HashMap<&'static str, BlockId>);

impl BlockRegistry {
    pub fn new(map: HashMap<&'static str, BlockId>) -> Self {
        Self(map)
    }
}

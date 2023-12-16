pub(crate) mod meshreg;
use bevy_ecs::prelude::Resource;
use moxi_utils::prelude::BlockId;
use std::collections::HashMap;

#[derive(Resource)]
pub struct BlockRegistry {
    pub(crate) block_name_to_id: HashMap<&'static str, BlockId>,
}

impl BlockRegistry {
    pub fn new(map: HashMap<&'static str, BlockId>) -> Self {
        Self {
            block_name_to_id: map,
        }
    }
}

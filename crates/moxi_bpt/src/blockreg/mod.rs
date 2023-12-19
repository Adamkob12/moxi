pub(crate) mod meshreg;
use bevy_ecs::prelude::Resource;
use moxi_utils::prelude::BlockId;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct BlockRegistry {
    pub(crate) name_to_id: HashMap<&'static str, BlockId>,
    pub(crate) id_to_name: HashMap<BlockId, &'static str>,
}

impl BlockRegistry {
    pub fn new(
        name_to_id: HashMap<&'static str, BlockId>,
        id_to_name: HashMap<BlockId, &'static str>,
    ) -> Self {
        Self {
            name_to_id,
            id_to_name,
        }
    }
}

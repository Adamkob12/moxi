pub(crate) mod meshreg;
use bevy_ecs::{
    prelude::{Res, Resource},
    query::{ROQueryItem, WorldQuery},
    system::{Query, SystemParam},
};
pub use meshreg::*;
use moxi_utils::prelude::BlockId;
use std::collections::HashSet;

use crate::prelude::BlockIdtoEnt;

#[derive(Resource, Default)]
pub struct BlockRegistry {
    pub(crate) names: HashSet<&'static str>,
}

#[derive(SystemParam)]
pub struct StaticBlockQuery<'w, 's, Q: WorldQuery + 'static> {
    block_id_to_ent: Res<'w, BlockIdtoEnt>,
    query: Query<'w, 's, Q>,
}

impl<'w, 's, Q: WorldQuery + 'static> StaticBlockQuery<'w, 's, Q> {
    pub fn get_static_property(&self, block_id: BlockId) -> Option<ROQueryItem<'_, Q>> {
        self.block_id_to_ent
            .0
            .get(&block_id)
            .and_then(|ent| self.query.get(*ent).ok())
    }
}

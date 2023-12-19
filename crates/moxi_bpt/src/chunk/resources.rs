use std::collections::hash_map::HashMap;

use bevy_ecs::{entity::Entity, system::Resource};
use moxi_utils::prelude::ChunkCords;

#[derive(Resource)]
pub struct ChunkMap(HashMap<ChunkCords, Entity>);

#[derive(Resource)]
pub struct ChunkQueue(Vec<ChunkCords>);

impl Default for ChunkMap {
    fn default() -> Self {
        Self(HashMap::with_capacity(100))
    }
}

impl ChunkMap {
    pub fn get_chunk(&self, cords: ChunkCords) -> Option<Entity> {
        self.0
            .get(&cords)
            .copied()
            .map_or(None, |e| (e != Entity::PLACEHOLDER).then_some(e))
    }
    pub fn insert_chunk(&mut self, cords: ChunkCords, entity: Entity) {
        self.0.insert(cords, entity);
    }
}

impl Default for ChunkQueue {
    fn default() -> Self {
        Self(Vec::with_capacity(100))
    }
}

impl ChunkQueue {
    pub fn push(&mut self, cords: ChunkCords) {
        self.0.push(cords);
    }

    pub fn drain(&mut self) -> impl Iterator<Item = ChunkCords> + '_ {
        self.0.drain(..)
    }
}

#[cfg(test)]
mod tests {
    use super::ChunkMap;
    use bevy_ecs::entity::Entity;

    #[test]
    fn test_chunk_map() {
        let mut map = ChunkMap::default();
        map.insert_chunk([0, 0].into(), Entity::PLACEHOLDER);
        map.insert_chunk([0, 1].into(), Entity::from_raw(1));
        assert_eq!(map.get_chunk([0, 0].into()), None);
        assert!(map
            .get_chunk([0, 1].into())
            .map_or(false, |e| e.index() == 1));
    }

    #[test]
    fn test_chunk_queue() {
        let mut queue = super::ChunkQueue::default();
        queue.push([0, 0].into());
        queue.push([0, 1].into());
        assert_eq!(
            queue.drain().collect::<Vec<_>>(),
            vec![[0, 0].into(), [0, 1].into()]
        );
    }
}
use std::collections::hash_map::HashMap;

use bevy_ecs::{entity::Entity, system::Resource};
use moxi_utils::prelude::ChunkCords;

/// Resource that stores all the chunks in the world.
/// The key is the chunk's cords, the value is the chunk's entity.
/// If the chunk is currently being loaded, the value will be `Entity::PLACEHOLDER`.
#[derive(Resource)]
pub struct ChunkMap(HashMap<ChunkCords, Entity>);

#[derive(Resource)]
pub(crate) struct ChunkQueue(Vec<ChunkCords>);

/// The current chunk is a resource that the plugin will refer to for the player's position.
/// Chunks will be loaded and unloaded based on the `CurrentChunk` resource.
#[derive(Resource)]
pub struct CurrentChunk(pub ChunkCords);

impl Default for CurrentChunk {
    fn default() -> Self {
        Self([0, 0].into())
    }
}

impl CurrentChunk {
    pub fn get(&self) -> ChunkCords {
        self.0
    }

    pub fn set(&mut self, cords: ChunkCords) {
        self.0 = cords;
    }
}

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

    pub fn contains_chunk(&self, cords: ChunkCords) -> bool {
        self.0.contains_key(&cords)
    }

    pub fn insert_chunk(&mut self, cords: ChunkCords, entity: Entity) {
        self.0.insert(cords, entity);
    }

    pub fn iter(&self) -> impl Iterator<Item = (ChunkCords, Entity)> + '_ {
        self.0.iter().map(|(cords, entity)| (*cords, *entity))
    }

    pub fn extract_if(&mut self, mut predicate: impl FnMut(&ChunkCords) -> bool) -> Vec<Entity> {
        let mut entities = Vec::new();
        self.0.retain(|cords, entity| {
            if predicate(cords) && *entity != Entity::PLACEHOLDER {
                entities.push(*entity);
                false
            } else {
                true
            }
        });
        entities
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

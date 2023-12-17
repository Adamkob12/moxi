pub(crate) mod block_commands;
pub(crate) mod blockworld;
pub(crate) mod update_event;

pub use update_event::*;

#[cfg(test)]
mod tests {
    use super::blockworld::*;
    use crate::prelude::*;
    use bevy_ecs::prelude::*;
    use moxi_mesh_utils::prelude::*;

    struct Block1;

    impl Block for Block1 {
        fn get_name() -> &'static str {
            "Block1"
        }

        fn get_mesh() -> BlockMesh {
            BlockMesh::Air
        }
    }

    fn trigger_always_true(_: In<BlockWorldUpdateEvent>) -> bool {
        true
    }

    fn action_print_hello_world_w_input(_: In<BlockWorldUpdateEvent>, _world: &mut World) {
        println!("Hello world!");
    }

    fn action_print_hello_world(_world: &mut World) {
        println!("Hello world!");
    }

    #[test]
    fn test_blockworld1() {
        let mut world = World::default();

        let mut initer = BlockInitiallizer::new(&mut world);

        initer
            .init_block::<Block1>()
            .add_static_properties(())
            .add_block_actions(trigger_always_true, (), ());

        world
            .query::<&BlockMarker>()
            .for_each(&world, |BlockMarker(id)| {
                assert_eq!(*id, 0);
            });
    }

    #[test]
    fn test_blockworld2() {
        let mut world = World::default();

        let mut initer = BlockInitiallizer::new(&mut world);

        initer
            .init_block::<Block1>()
            .add_static_properties(())
            .add_block_actions(
                trigger_always_true,
                action_print_hello_world,
                action_print_hello_world_w_input,
            );

        world
            .query::<&BlockMarker>()
            .for_each(&world, |BlockMarker(id)| {
                assert_eq!(*id, 0);
            });
    }
}

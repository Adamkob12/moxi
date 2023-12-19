pub(crate) mod block_commands;
pub(crate) mod blocks_param;
pub(crate) mod blockworld;
pub(crate) mod update_event;

pub use update_event::*;

#[cfg(test)]
mod tests {
    use super::blockworld::*;
    use crate::{blockreg::meshreg::MeshReg, prelude::*};
    use bevy_ecs::prelude::*;
    use defs::*;
    use moxi_mesh_utils::prelude::{BlockMeshType, MeshRegistry};

    // different module so I can fold it neetly in the editor
    mod defs {
        use crate::prelude::*;
        use bevy_ecs::prelude::*;
        use moxi_mesh_utils::prelude::*;
        use moxi_utils::prelude::Face;
        pub struct Block1;
        pub struct Block2;

        impl Block for Block1 {
            fn get_name() -> &'static str {
                "Block1"
            }

            fn get_mesh() -> BlockMesh {
                BlockMesh::Air
            }
        }

        impl Block for Block2 {
            fn get_name() -> &'static str {
                "Block2"
            }

            fn get_mesh() -> BlockMesh {
                generate_cube_mesh(
                    [1.0; 3],
                    [10, 10],
                    [
                        (Face::Top, [0, 0]),
                        (Face::Bottom, [0, 0]),
                        (Face::Left, [0, 0]),
                        (Face::Right, [0, 0]),
                        (Face::Front, [0, 0]),
                        (Face::Back, [0, 0]),
                    ],
                    [0.0; 3],
                    0.0,
                    Some(1.0),
                    1.0,
                )
            }
        }

        #[derive(Resource)]
        pub struct Counter(pub usize);

        pub fn increment_counter(mut counter: ResMut<Counter>) {
            counter.0 += 1;
        }

        pub fn trigger_always_true() -> bool {
            true
        }

        pub fn action_print_hello_world_w_input(_: In<BlockWorldUpdateEvent>, _world: &mut World) {
            println!("Hello world!");
        }

        pub fn action_print_hello_world(_world: &mut World) {
            println!("Hello world!");
        }
    }

    /// Test simply adding a block
    #[test]
    fn test_blockworld1() {
        let mut world = World::default();

        world
            .init_block::<Block1>()
            .with_static_properties(())
            .with_block_actions(trigger_always_true, (), ());

        world
            .query::<&BlockMarker>()
            .for_each(&world, |BlockMarker(id)| {
                assert_eq!(*id, 0);
            });
    }

    /// Test adding multiple block actions with input and without
    #[test]
    fn test_blockworld2() {
        let mut world = World::default();

        world
            .init_block::<Block1>()
            .with_static_properties(())
            .with_block_actions(
                trigger_always_true,
                (action_print_hello_world, action_print_hello_world),
                action_print_hello_world_w_input,
            );

        world
            .query::<&BlockMarker>()
            .for_each(&world, |BlockMarker(id)| {
                assert_eq!(*id, 0);
            });
    }

    /// Test adding multiple blocks and small test of the mesh registry
    #[test]
    fn test_blockworld3() {
        let mut world = World::default();

        world
            .init_block::<Block1>()
            .with_static_properties(())
            .init_block::<Block2>()
            .with_static_properties(());

        let mesh_ty = world.resource::<MeshReg>().get_block_mesh_type(&1);
        assert_eq!(mesh_ty, BlockMeshType::Cube);
    }

    /// Test the execution of block actions
    #[test]
    fn test_block_actions1() {
        let mut world = World::default();

        world.insert_resource(Counter(0));

        world
            .init_block::<Block1>()
            .with_static_properties(())
            .with_block_actions(
                trigger_always_true,
                (increment_counter, increment_counter, increment_counter),
                (),
            );

        let block_entity = {
            let block_reg = world.resource::<BlockRegistry>();
            let block_id = block_reg.name_to_id.get("Block1").unwrap();
            let block_id_to_ent = world.resource::<BlockIdtoEnt>();
            let block_entity = block_id_to_ent.0.get(block_id).unwrap();
            *block_entity
        };

        unsafe {
            let unsafe_world_cell = world.as_unsafe_world_cell();
            unsafe_world_cell
                .world()
                .get::<BlockActions>(block_entity)
                .unwrap()
                .execute_all_unsafe(unsafe_world_cell, None);
        }

        assert_eq!(world.resource::<Counter>().0, 3);
    }
}

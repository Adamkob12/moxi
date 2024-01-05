use bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell;

use crate::*;

#[derive(Component)]
pub enum Trigger {
    Input(
        bool,
        Box<dyn System<In = BlockWorldUpdateEvent, Out = bool>>,
    ),
    No(bool, Box<dyn System<In = (), Out = bool>>),
}

pub trait IntoTrigger<In, M>: IntoSystem<In, bool, M> {
    fn into_trigger(self) -> Trigger;
}

impl Trigger {
    pub fn evaluate(&mut self, input: Option<BlockWorldUpdateEvent>, world: &mut World) -> bool {
        match self {
            Trigger::Input(initialized, sys) => {
                if !*initialized {
                    *initialized = true;
                    sys.initialize(world);
                }
                sys.run(
                    input.expect("Expected valid input to evaluate Trigger"),
                    world,
                )
            }
            Trigger::No(initialized, sys) => {
                if !*initialized {
                    *initialized = true;
                    sys.initialize(world);
                }
                sys.run((), world)
            }
        }
    }

    pub unsafe fn evaluate_unsafe<'w>(
        &mut self,
        input: Option<BlockWorldUpdateEvent>,
        world: UnsafeWorldCell<'w>,
    ) -> bool {
        match self {
            Trigger::Input(initialized, sys) => {
                if !*initialized {
                    *initialized = true;
                    unsafe {
                        sys.initialize(world.world_mut());
                    }
                }
                sys.run_unsafe(
                    input.expect("Expected valid input to evaluate Trigger"),
                    world,
                )
            }
            Trigger::No(initialized, sys) => {
                if !*initialized {
                    *initialized = true;
                    unsafe {
                        sys.initialize(world.world_mut());
                    }
                }
                sys.run_unsafe((), world)
            }
        }
    }

    pub fn get_id(&self) -> std::any::TypeId {
        match self {
            Trigger::Input(_, sys) => sys.type_id(),
            Trigger::No(_, sys) => sys.type_id(),
        }
    }
}

impl<S, M> IntoTrigger<(), M> for S
where
    S: IntoSystem<(), bool, M>,
{
    fn into_trigger(self) -> Trigger {
        return Trigger::No(false, Box::new(S::into_system(self)));
    }
}

impl<S, M> IntoTrigger<BlockWorldUpdateEvent, M> for S
where
    S: IntoSystem<BlockWorldUpdateEvent, bool, M>,
{
    fn into_trigger(self) -> Trigger {
        return Trigger::Input(false, Box::new(S::into_system(self)));
    }
}

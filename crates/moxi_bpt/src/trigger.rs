use bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell;

use crate::*;

#[derive(Component)]
pub enum Trigger {
    Input(Box<dyn System<In = BlockWorldUpdateEvent, Out = bool>>),
    No(Box<dyn System<In = (), Out = bool>>),
}

pub trait IntoTrigger<In, M>: IntoSystem<In, bool, M> {
    fn into_trigger(self) -> Trigger;
}

impl Trigger {
    pub fn evaluate(&mut self, input: Option<BlockWorldUpdateEvent>, world: &mut World) -> bool {
        match self {
            Trigger::Input(sys) => sys.run(input.unwrap(), world),
            Trigger::No(sys) => sys.run((), world),
        }
    }

    pub unsafe fn evaluate_unsafe<'w>(
        &mut self,
        input: Option<BlockWorldUpdateEvent>,
        world: UnsafeWorldCell<'w>,
    ) -> bool {
        match self {
            Trigger::Input(sys) => sys.run_unsafe(input.unwrap(), world),
            Trigger::No(sys) => sys.run_unsafe((), world),
        }
    }
    pub fn get_id(&self) -> std::any::TypeId {
        match self {
            Trigger::Input(sys) => sys.type_id(),
            Trigger::No(sys) => sys.type_id(),
        }
    }
}

impl<S, M> IntoTrigger<(), M> for S
where
    S: IntoSystem<(), bool, M>,
{
    fn into_trigger(self) -> Trigger {
        return Trigger::No(Box::new(S::into_system(self)));
    }
}

impl<S, M> IntoTrigger<BlockWorldUpdateEvent, M> for S
where
    S: IntoSystem<BlockWorldUpdateEvent, bool, M>,
{
    fn into_trigger(self) -> Trigger {
        return Trigger::Input(Box::new(S::into_system(self)));
    }
}

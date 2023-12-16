use crate::*;

#[derive(Component)]
pub enum Trigger {
    WithInput(Box<dyn System<In = BlockWorldUpdateEvent, Out = bool>>),
    NoInput(Box<dyn System<In = (), Out = bool>>),
}

impl Trigger {
    pub fn evaluate(
        &mut self,
        world: &mut World,
        update_event: Option<BlockWorldUpdateEvent>,
    ) -> bool {
        match self {
            Trigger::WithInput(ref mut sys) => sys.run(
                update_event.expect("Tried to evaluate trigger condition without proper input"),
                world,
            ),
            Trigger::NoInput(ref mut sys) => sys.run((), world),
        }
    }

    pub fn get_id(&self) -> std::any::TypeId {
        match self {
            Trigger::WithInput(sys) => sys.type_id(),
            Trigger::NoInput(sys) => sys.type_id(),
        }
    }
}

pub trait IntoTrigger {
    fn into_trigger(self) -> Trigger;
}

use std::marker::PhantomData;
pub struct BIS<In, M, S: IntoSystem<In, bool, M> + 'static> {
    sys: S,
    marker: PhantomData<(In, M)>,
}

impl<In, M, S: IntoSystem<In, bool, M> + 'static> BIS<In, M, S> {
    pub fn new(sys: S) -> Self {
        Self {
            sys,
            marker: PhantomData,
        }
    }
}

impl<M, S: IntoSystem<(), bool, M> + 'static> IntoTrigger for BIS<(), M, S> {
    fn into_trigger(self) -> Trigger {
        Trigger::NoInput(Box::new(IntoSystem::into_system(self.sys)))
    }
}

impl<M, S: IntoSystem<BlockWorldUpdateEvent, bool, M> + 'static> IntoTrigger
    for BIS<BlockWorldUpdateEvent, M, S>
{
    fn into_trigger(self) -> Trigger {
        Trigger::WithInput(Box::new(IntoSystem::into_system(self.sys)))
    }
}

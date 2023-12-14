use crate::*;

pub enum Trigger {
    WithInput(Box<dyn ReadOnlySystem<In = BlockWorldUpdateEvent, Out = bool>>),
    NoInput(Box<dyn ReadOnlySystem<In = (), Out = bool>>),
}

impl Trigger {
    pub fn evaluate(&mut self, world: &World, update_event: Option<BlockWorldUpdateEvent>) -> bool {
        match self {
            Trigger::WithInput(ref mut sys) => sys.run_readonly(
                update_event.expect("Tried to evaluate trigger condition without proper input"),
                world,
            ),
            Trigger::NoInput(ref mut sys) => sys.run_readonly((), world),
        }
    }
}

pub trait IntoTrigger {
    fn into_trigger(self) -> Trigger;
}

type ConditionSystem<In> = Box<dyn ReadOnlySystem<In = In, Out = bool>>;

impl IntoTrigger for ConditionSystem<()> {
    fn into_trigger(self) -> Trigger {
        Trigger::NoInput(self)
    }
}

impl IntoTrigger for ConditionSystem<BlockWorldUpdateEvent> {
    fn into_trigger(self) -> Trigger {
        Trigger::WithInput(self)
    }
}

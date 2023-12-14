use bevy_ecs::all_tuples;

use super::*;

pub enum Action {
    WithInput(Box<dyn System<In = BlockWorldUpdateEvent, Out = ()>>),
    NoInput(Box<dyn System<In = (), Out = ()>>),
}

impl Action {
    pub fn run_action(&mut self, world: &mut World, update_event: Option<BlockWorldUpdateEvent>) {
        match self {
            Action::WithInput(ref mut sys) => sys.run(
                update_event.expect("Tried to evaluate trigger condition without proper input"),
                world,
            ),
            Action::NoInput(ref mut sys) => sys.run((), world),
        }
    }
}

pub trait IntoAction {
    fn into_action(self) -> Action;
}

type BoxedAction<In> = Box<dyn System<In = In, Out = ()>>;

impl IntoAction for BoxedAction<()> {
    fn into_action(self) -> Action {
        Action::NoInput(self)
    }
}

impl IntoAction for BoxedAction<BlockWorldUpdateEvent> {
    fn into_action(self) -> Action {
        Action::WithInput(self)
    }
}

pub type ActionSet = Vec<Action>;

pub trait IntoActionSet {
    fn into_action_set(self) -> ActionSet;
}

impl<T1: IntoAction> IntoActionSet for T1 {
    fn into_action_set(self) -> ActionSet {
        let mut actions = Vec::new();
        actions.push(self.into_action());
        actions
    }
}

macro_rules! impl_into_action_set {
    ($($T:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($T: IntoAction),*> IntoActionSet for ($($T,)*) {
            fn into_action_set(self) -> ActionSet {
                let ($($T,)*) = self;
                vec![$($T.into_action()),*]
            }
        }
    };
}

all_tuples!(impl_into_action_set, 0, 15, T);

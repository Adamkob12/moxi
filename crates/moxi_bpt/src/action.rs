use bevy_ecs::{all_tuples, world::unsafe_world_cell::UnsafeWorldCell};

use crate::*;

#[derive(Component)]
pub enum Action {
    Input(bool, Box<dyn System<In = BlockWorldUpdateEvent, Out = ()>>),
    No(bool, Box<dyn System<In = (), Out = ()>>),
}

pub trait IntoAction<In, M>: IntoSystem<In, (), M> {
    fn into_action(self) -> Action;
}

impl Action {
    pub unsafe fn apply_deferred_unsafe<'w>(&mut self, world: UnsafeWorldCell<'w>) {
        match self {
            Action::Input(init, sys) if *init => {
                sys.apply_deferred(world.world_mut());
            }
            Action::No(init, sys) if *init => {
                sys.apply_deferred(world.world_mut());
            }
            _ => {}
        }
    }

    pub fn apply_deferred<'w>(&mut self, world: &'w mut World) {
        match self {
            Action::Input(init, sys) if *init => {
                sys.apply_deferred(world);
            }
            Action::No(init, sys) if *init => {
                sys.apply_deferred(world);
            }
            _ => {}
        }
    }

    pub fn run(&mut self, input: Option<BlockWorldUpdateEvent>, world: &mut World) {
        match self {
            Action::Input(initialized, sys) => {
                if !*initialized {
                    *initialized = true;
                    sys.initialize(world);
                }
                sys.run(
                    input.expect("Expected valid input to evaluate Trigger"),
                    world,
                )
            }
            Action::No(initialized, sys) => {
                if !*initialized {
                    *initialized = true;
                    sys.initialize(world);
                }
                sys.run((), world)
            }
        }
    }

    pub unsafe fn run_unsafe<'w>(
        &mut self,
        input: Option<BlockWorldUpdateEvent>,
        world: UnsafeWorldCell<'w>,
    ) {
        match self {
            Action::Input(initialized, sys) => {
                if !*initialized {
                    *initialized = true;
                    unsafe {
                        sys.initialize(world.world_mut());
                    }
                }
                sys.run_unsafe(
                    input.expect("Expected valid input to evaluate Trigger"),
                    world,
                );
            }
            Action::No(initialized, sys) => {
                if !*initialized {
                    *initialized = true;
                    unsafe {
                        sys.initialize(world.world_mut());
                    }
                }
                sys.run_unsafe((), world);
            }
        }
    }

    pub fn get_id(&self) -> std::any::TypeId {
        match self {
            Action::Input(_, sys) => sys.type_id(),
            Action::No(_, sys) => sys.type_id(),
        }
    }
}

impl<S, M> IntoAction<(), M> for S
where
    S: IntoSystem<(), (), M>,
{
    fn into_action(self) -> Action {
        return Action::No(false, Box::new(S::into_system(self)));
    }
}

impl<S, M> IntoAction<BlockWorldUpdateEvent, M> for S
where
    S: IntoSystem<BlockWorldUpdateEvent, (), M>,
{
    fn into_action(self) -> Action {
        return Action::Input(false, Box::new(S::into_system(self)));
    }
}

pub type ActionSet = Vec<Action>;
pub trait CommonActionSet {
    fn get_ids(&self) -> Vec<std::any::TypeId>;
    fn enumerate_ids_and_actions(self) -> impl Iterator<Item = (std::any::TypeId, Action)>;
}

impl CommonActionSet for ActionSet {
    fn get_ids(&self) -> Vec<std::any::TypeId> {
        self.iter().map(|action| action.get_id()).collect()
    }

    fn enumerate_ids_and_actions(self) -> impl Iterator<Item = (std::any::TypeId, Action)> {
        self.into_iter().map(|action| (action.get_id(), action))
    }
}

pub struct TupleMarker<T> {
    _phantom: std::marker::PhantomData<T>,
}
pub struct SingleAndReadyToMingle<T> {
    _phantom: std::marker::PhantomData<T>,
}

pub struct EmptyActionSet;

pub trait IntoActionSet<I: ValidActionInput, M> {
    fn into_action_set(self) -> ActionSet;
}

macro_rules! impl_into_action_set {
    ($($T:ident),*) => {
        #[allow(non_snake_case)]
        impl<M, I: ValidActionInput, $($T: IntoActionSet<I, M>),*> IntoActionSet<I, TupleMarker<M>> for ($($T,)*) {
            fn into_action_set(self) -> ActionSet {
                let ($($T,)*) = self;
                let mut actions = vec![];
                $(actions.extend($T.into_action_set()));*;
                actions
            }
        }
    };
}

all_tuples!(impl_into_action_set, 1, 15, T);

impl IntoActionSet<(), EmptyActionSet> for () {
    fn into_action_set(self) -> ActionSet {
        vec![]
    }
}

impl IntoActionSet<BlockWorldUpdateEvent, EmptyActionSet> for () {
    fn into_action_set(self) -> ActionSet {
        vec![]
    }
}

impl<M, I: ValidActionInput, T> IntoActionSet<I, SingleAndReadyToMingle<M>> for T
where
    T: IntoAction<I, M>,
{
    fn into_action_set(self) -> ActionSet {
        vec![self.into_action()]
    }
}

pub trait ValidActionInput {}

impl ValidActionInput for () {}
impl ValidActionInput for BlockWorldUpdateEvent {}

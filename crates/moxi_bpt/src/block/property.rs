use bevy_ecs::all_tuples;
use bevy_ecs::bundle::{Bundle, DynamicBundle};
use bevy_ecs::component::{Component, TableStorage};

#[derive(Component)]
pub struct DynamicBlock;

#[derive(Component)]
pub struct StaticBlock;

pub trait DynamicProperty: 'static {
    fn encode(&self) -> u8
    where
        Self: Sized + Into<u8> + Copy,
    {
        (*self).into()
    }

    fn decode(value: u8) -> Self
    where
        Self: Sized + From<u8> + Copy,
    {
        Self::from(value)
    }
}

pub struct DynamicProperties(pub Vec<BoxedDynamicProperty>);

impl Default for DynamicProperties {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl DynamicProperties {
    pub fn extend(&mut self, dynamic_properties: Vec<impl DynamicProperty + Sized>) {
        self.0.extend(
            dynamic_properties
                .into_iter()
                .map(|x| Box::new(x) as BoxedDynamicProperty),
        );
    }
}

pub type BoxedDynamicProperty = Box<dyn DynamicProperty>;

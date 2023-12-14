use bevy_ecs::all_tuples;
use bevy_ecs::component::{Component, StorageType};

pub type PropertyType = u8;

pub const STATIC_PROPERTY_TYPE: PropertyType = 0;
pub const DYNAMIC_PROPERTY_TYPE: PropertyType = 1;

#[repr(u8)]
pub enum BlockType {
    Static = STATIC_PROPERTY_TYPE,
    Dynamic = DYNAMIC_PROPERTY_TYPE,
}

pub trait StaticProperty {}
pub trait DynamicProperty {}

pub trait Property<const T: PropertyType>: Component<Storage = StorageType> {
    fn into_boxed_property(self) -> BoxedProperty<T>;
}

impl<S: StaticProperty + Component<Storage = StorageType>> Property<STATIC_PROPERTY_TYPE> for S {
    fn into_boxed_property(self) -> BoxedProperty<STATIC_PROPERTY_TYPE> {
        BoxedProperty::new::<S>(self)
    }
}

impl<D: DynamicProperty + Component<Storage = StorageType>> Property<DYNAMIC_PROPERTY_TYPE> for D {
    fn into_boxed_property(self) -> BoxedProperty<DYNAMIC_PROPERTY_TYPE> {
        BoxedProperty::new::<D>(self)
    }
}

pub struct BoxedProperty<const F: PropertyType>(Box<dyn Property<F>>);

impl<const F: PropertyType> BoxedProperty<F> {
    pub fn new<P: Property<F>>(property: P) -> Self {
        BoxedProperty::<F>(Box::new(property))
    }
}

pub trait PropertyBundle<const F: PropertyType> {
    fn get_properties(self) -> Vec<BoxedProperty<F>>;
}

macro_rules! impl_into_prop_bundle {
    ($($T:ident),*) => {
        #[allow(non_snake_case)]
        impl<const F: PropertyType,$($T: Property<F>),*> PropertyBundle<F> for ($($T,)*) {
            fn get_properties(self) -> Vec<BoxedProperty<F>> {
                let ($($T,)*) = self;
                vec![$($T.into_boxed_property()),*]
            }
        }
    };
}

all_tuples!(impl_into_prop_bundle, 0, 15, T);

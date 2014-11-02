//! Entity Component System Library (ECS)
//!
//! For info about why an ECS may be beneficial, see some of these articles:
//!
//! - http://gameprogrammingpatterns.com/component.html
//! - http://t-machine.org/index.php/2007/09/03/entity-systems-are-the-future-of-mmog-development-part-1/
//! - http://www.gamedev.net/page/resources/_/technical/game-programming/understanding-component-entity-systems-r3013
//! - http://cowboyprogramming.com/2007/01/05/evolve-your-heirachy/
//!
//! There is a large variety of ways an ECS may work. This particular one is similar to
//! [Artemis](http://gamadu.com/artemis/).
//! Although this isn't a port to Rust, most functionality should be similar, and the
//! tutorials/manual there should be able to make up for the current lack of documentation [FIXME]
//!
//! Here's the basic structure:
//!
//! - An `Entity` is just an identifier. It contains no data or logic whatsoever.
//! - A `Component` is a piece of data (eg: Position, Velocity, Colour). While containing logic can
//! sometimes be useful, it's best practice to avoid it wherever possible.
//! - A `System` runs all the logic. Most of the time, it filters out entities based on their
//! components, and only runs it's logic on the entities it's interested in. These filters are
//! called `Aspect`s. Some systems ignore entities, and just apply logic to the world itself.
//! - An `Aspect` is a simple helper to filter entities based on their components.
//! - A `Manager` is simply an object that observes when an entity is
//! added/activated/deactivated/removed. They are used to help 'manage' the entities, rather than
//! define data or logic.
//! - The `World` organises all the above items together to make sure everything runs as it should.

#![crate_name = "ecs"]
#![comment = "Entity Component System Library"]
#![license = "MIT"]
#![crate_type = "lib"]

#![feature(macro_rules, phase, slicing_syntax)]
#![unstable]

extern crate uuid;
#[phase(link, plugin)]
extern crate log;

pub use aspect::Aspect;
pub use component::{Component, ComponentId};
pub use entity::{Entity, EntityBuilder, EntityModifier};
pub use manager::{Manager, MutableManager};
pub use system::{Passive, System};
pub use world::{Components, EntityData, World, WorldBuilder};

pub mod buffer;

pub mod aspect;
pub mod component;
pub mod entity;
pub mod manager;
pub mod system;
pub mod world;

#[macro_escape]
mod macros
{
    #[macro_export]
    /// Define a new component
    macro_rules! component {
        ($($Name:ident { $($field:ident : $ty:ty),+ })+) =>
        {
            $(
                #[deriving(Default, PartialEq, Show)]
                pub struct $Name
                {
                    $(pub $field : $ty),+
                }
            )+
        };
    }

    #[macro_export]
    /// Define a new empty component
    macro_rules! feature {
        ($($Name:ident;)+) =>
        {
            $(
                #[deriving(Default, PartialEq, Show)]
                pub struct $Name;
            )+
        };
    }

    #[macro_export]
    /// Define a new wrapper type
    macro_rules! new_type {
        ($($Name:ident($Type:ty);)+) =>
        {
            $(
                #[deriving(Default, PartialEq, Show)]
                pub struct $Name(pub $Type);

                impl Deref<$Type> for $Name
                {
                    fn deref(&self) -> &$Type
                    {
                        let $Name(ref ret) = *self;
                        ret
                    }
                }
            )+
        };
    }

    #[macro_export]
    /// Get the component ID of a type
    macro_rules! component_id {
        ($ty:ty) =>
        {
            ::std::intrinsics::TypeId::of::<$ty>().hash()
        };
    }

    #[macro_export]
    /// Get the component IDs of types
    macro_rules! component_ids {
        ($($ty:ty),+) =>
        {
            vec![$(::std::intrinsics::TypeId::of::<$ty>().hash(),)+]
        };
    }

    #[macro_export]
    /// Define an aspect using binary ops
    macro_rules! aspect (
        ($($ty:ty)&+) =>
        {
            Aspect::for_all(component_ids!($($ty),+))
        };
        ($($ty:ty)|+) =>
        {
            Aspect::for_one(component_ids!($($ty),+))
        };
        ($(!$ty:ty)&+) =>
        {
            Aspect::for_none(component_ids!($($ty),+))
        };
    )

    #[macro_export]
    /// Construct a world from the components and systems given
    macro_rules! world(
        (components: [$($comp:ty),+], systems: [$($sys:expr),+]) =>
        {
            let mut builder = WorldBuilder::new();
            $( builder.register_component::<$comp>(); )+
            $( builder.register_system(box $sys as Box<System>); )+
            builder.build()
        };
    )
}

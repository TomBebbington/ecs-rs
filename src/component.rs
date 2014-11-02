
//! Store data in parts to allow defining different entities through composition.

use std::collections::Bitv;
use std::intrinsics::TypeId;

use buffer::Buffer;
use Entity;

/// The data for a specific aspect of an entity
pub trait Component: Copy+'static {}

impl<T:Copy+'static> Component for T {}

/// Uniquely identifies each component
pub type ComponentId = u64;

#[doc(hidden)]
pub struct ComponentList
{
    buffer: Buffer,
    enabled: Bitv,
    id: ComponentId,
}

impl ComponentList
{
    pub fn new<T:Component>() -> ComponentList
    {
        ComponentList
        {
            buffer: Buffer::new::<T>(),
            enabled: Bitv::new(),
            id: TypeId::of::<T>().hash(),
        }
    }

    pub fn add<T:Component>(&mut self, entity: &Entity, component: &T) -> bool
    {
        if **entity < self.enabled.len() && self.enabled.get(**entity)
        {
            false
        }
        else if TypeId::of::<T>().hash() != self.id
        {
            false
        }
        else
        {
            self.buffer[**entity] = component;
            if **entity >= self.enabled.len()
            {
                let diff = **entity - self.enabled.len();
                self.enabled.grow(diff+1, false);
            }
            self.enabled.set(**entity, true);
            true
        }
    }

    pub fn set<T:Component>(&mut self, entity: &Entity, component: &T) -> bool
    {
        if **entity >= self.enabled.len() || !self.enabled.get(**entity)
        {
            false
        }
        else if TypeId::of::<T>().hash() != self.id
        {
            false
        }
        else
        {
            self.buffer[**entity] = component;
            true
        }
    }

    pub fn add_or_set<T:Component>(&mut self, entity: &Entity, component: &T) -> bool
    {
        if TypeId::of::<T>().hash() != self.id
        {
            false
        }
        else
        {
            self.buffer[**entity] = component;
            if **entity >= self.enabled.len()
            {
                let diff = **entity - self.enabled.len();
                self.enabled.grow(diff+1, false);
            }
            self.enabled.set(**entity, true);
            true
        }
    }

    pub fn has(&self, entity: &Entity) -> bool
    {
        **entity < self.enabled.len() && self.enabled.get(**entity)
    }

    pub fn get<T:Component>(&self, entity: &Entity) -> Option<T>
    {
        if **entity < self.enabled.len() && self.enabled.get(**entity)
        {
            self.buffer[**entity]
        }
        else
        {
            None
        }
    }

    pub fn borrow<T:Component>(&self, entity: &Entity) -> Option<&T>
    {
        if **entity < self.enabled.len() && self.enabled.get(**entity)
        {
            self.buffer[**entity]
        }
        else
        {
            None
        }
    }

    pub fn borrow_mut<T:Component>(&mut self, entity: &Entity) -> Option<&mut T>
    {
        if **entity < self.enabled.len() && self.enabled.get(**entity)
        {
            Some(&mut self.buffer[**entity])
        }
        else
        {
            None
        }
    }

    pub fn rm(&mut self, entity: &Entity) -> bool
    {
        if **entity < self.enabled.len() && self.enabled.get(**entity)
        {
            self.enabled.set(**entity, false);
            true
        }
        else
        {
            false
        }
    }

    pub fn get_cid(&self) -> ComponentId
    {
        self.id
    }
}

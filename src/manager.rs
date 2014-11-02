
#![experimental]

//! Traits to observe and manage entities as they are changed in the world.

use std::cell::RefCell;
use std::rc::Rc;

use Entity;
use World;

/// A manager which can mutate itself
pub trait MutableManager: 'static
{
    /// Called when an entity is added to the world.
    fn activated(&mut self, &Entity, &World);
    /// Called when an entity is modified in the world.
    fn reactivated(&mut self, &Entity, &World);
    /// Called when an entity is removed from the world.
    fn deactivated(&mut self, &Entity, &World);
}

/// A manager which can't mutate itself
pub trait Manager: 'static
{
    /// Called when an entity is added to the world.
    fn activated(&self, &Entity, &World);
    /// Called when an entity is modified in the world.
    fn reactivated(&self, &Entity, &World);
    /// Called when an entity is removed from the world.
    fn deactivated(&self, &Entity, &World);
}

impl<T: MutableManager> MutableManager for Rc<RefCell<T>>
{
    fn activated(&mut self, e: &Entity, w: &World)
    {
        self.borrow_mut().activated(e, w)
    }

    fn reactivated(&mut self, e: &Entity, w: &World)
    {
        self.borrow_mut().reactivated(e, w)
    }

    fn deactivated(&mut self, e: &Entity, w: &World)
    {
        self.borrow_mut().deactivated(e, w)
    }
}


//! Entity identifier and manager types.

use std::collections::Bitv;
use uuid::Uuid;

use Components;

/// Dual identifier for an entity.
///
/// The first element (uint) is the entity's index, used to locate components.
/// This value can be recycled, so the second element (Uuid) is used as an identifier.
#[stable]
#[deriving(Clone, Eq, PartialEq, Show)]
pub struct Entity {
    /// The entity's index, used to locate elements
    pub index: uint,
    /// The unique identifier of the entity
    pub uuid: Uuid
}

#[stable]
impl Entity
{
    #[inline(always)]
    pub fn new(index: uint, uuid: Uuid) -> Entity
    {
        Entity
        {
            index: index,
            uuid: uuid
        }
    }
    #[inline]
    pub fn nil() -> Entity
    {
        Entity::new(0, Uuid::nil())
    }
}

impl Deref<uint> for Entity
{
    #[inline]
    fn deref(&self) -> &uint
    {
        &self.index
    }
}

pub trait EntityBuilder: 'static
{
    fn build(&mut self, &mut Components, Entity);
}

impl EntityBuilder for |&mut Components, Entity|: 'static
{
    fn build(&mut self, c: &mut Components, e: Entity)
    {
        (*self)(c, e);
    }
}

impl EntityBuilder for () { fn build(&mut self, _: &mut Components, _: Entity) {} }

pub trait EntityModifier: 'static
{
    fn modify(&mut self, &mut Components, Entity);
}

impl EntityModifier for |&mut Components, Entity|: 'static
{
    fn modify(&mut self, c: &mut Components, e: Entity)
    {
        (*self)(c, e);
    }
}

impl EntityModifier for () { fn modify(&mut self, _: &mut Components, _: Entity) {} }

/// Handles creation, activation, and validating of entities.
#[doc(hidden)]
pub struct EntityManager
{
    ids: IdPool,
    entities: Vec<Entity>,
    enabled: Bitv,
}

impl EntityManager
{
    /// Returns a new `EntityManager`
    pub fn new() -> EntityManager
    {
        EntityManager
        {
            ids: IdPool::new(),
            entities: Vec::new(),
            enabled: Bitv::new(),
        }
    }

    /// Creates a new `Entity`, assigning it the first available identifier.
    pub fn create_entity(&mut self) -> Entity
    {
        let ret = Entity::new(self.ids.get_id(), Uuid::new_v4());
        if *ret >= self.entities.len()
        {
            let diff = *ret - self.entities.len();
            self.entities.grow(diff+1, Entity::new(0, Uuid::nil()));
        }
        self.entities[mut][*ret] = ret;

        if *ret >= self.enabled.len()
        {
            let diff = *ret - self.enabled.len();
            self.enabled.grow(diff+1, false);
        }
        self.enabled.set(*ret, true);
        ret
    }

    /// Returns true if an entity is valid (not removed from the manager).
    #[inline]
    pub fn is_valid(&self, entity: &Entity) -> bool
    {
        &self.entities[**entity] == entity && self.enabled[**entity]
    }

    /// Deletes an entity from the manager.
    pub fn delete_entity(&mut self, entity: &Entity)
    {
        self.entities[mut][**entity] = Entity::new(0, Uuid::nil());
        self.enabled.set(**entity, false);
        self.ids.return_id(**entity);
    }
}

struct IdPool
{
    recycled: Vec<uint>,
    next_id: uint,
}

impl IdPool
{
    pub fn new() -> IdPool
    {
        IdPool
        {
            recycled: Vec::new(),
            next_id: 1u,
        }
    }

    pub fn get_id(&mut self) -> uint
    {
        match self.recycled.pop()
        {
            Some(id) => id,
            None => {
                self.next_id += 1;
                self.next_id - 1
            }
        }
    }

    pub fn return_id(&mut self, id: uint)
    {
        self.recycled.push(id);
    }
}

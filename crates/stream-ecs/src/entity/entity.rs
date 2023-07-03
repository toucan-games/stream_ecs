#![allow(clippy::module_inception)]

use derive_more::Display;

use crate::entity::builder::EntityBuilder;

/// Unique key of the entity in ECS.
///
/// Similarly as in arenas, you can store it anywhere
/// to obtain components attached to the entity.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[display(fmt = "{index}v{generation}")]
pub struct Entity {
    index: u32,
    generation: u32,
}

impl Entity {
    /// Creates new entity key with provided index and its generation.
    ///
    /// More detained information about index and generation can be found
    /// in the documentation of [`index`][index] and [`generation`][generation] methods.
    ///
    /// [index]: Entity::index()
    /// [generation]: Entity::generation()
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::Entity;
    ///
    /// let entity = Entity::new(0, 0);
    /// ```
    pub const fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }

    /// Creates the key which doesn't belong to any entity.
    ///
    /// A null key is always invalid, but an invalid key
    /// (that was removed from the world) is not a null key.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::Entity;
    ///
    /// let entity = Entity::null();
    /// assert!(entity.is_null());
    /// ```
    pub const fn null() -> Self {
        Self {
            index: u32::MAX,
            generation: 0,
        }
    }

    /// Creates an empty entity builder to build a new entity with.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::{builder::EntityBuilder, Entity};
    ///
    /// let builder = Entity::builder();
    /// assert_eq!(builder, EntityBuilder::empty());
    /// ```
    pub const fn builder() -> EntityBuilder {
        EntityBuilder::empty()
    }

    /// Checks if the entity key is null.
    ///
    /// Null keys are created through the [`Entity::null()`] method or
    /// by creating default entity key.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::Entity;
    ///
    /// let entity = Entity::new(0, 0);
    /// assert!(!entity.is_null());
    ///
    /// let entity = Entity::null();
    /// assert!(entity.is_null());
    /// ```
    pub const fn is_null(self) -> bool {
        self.index == u32::MAX
    }

    /// Returns a unique index of the entity.
    ///
    /// Index itself is not a key of the entity: the same index cannot be shared
    /// between two alive entities, but it can collide for both alive and dead entities.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::Entity;
    ///
    /// let entity = Entity::new(42, 127);
    /// assert_eq!(entity.index(), 42);
    /// ```
    pub const fn index(self) -> u32 {
        self.index
    }

    /// Returns the generation of the entity.
    ///
    /// When the entity with a given index is removed, its generation is increased.
    /// This allows to solve ABA problem and uniquely identify an entity.
    /// With a generation we can tell how many times some entity has been reused.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::Entity;
    ///
    /// let entity = Entity::new(42, 127);
    /// assert_eq!(entity.generation(), 127);
    /// ```
    pub const fn generation(self) -> u32 {
        self.generation
    }
}

impl Default for Entity {
    /// Creates default entity key, which is null.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::Entity;
    ///
    /// let entity = Entity::default();
    /// assert_eq!(entity, Entity::null());
    /// ```
    fn default() -> Self {
        Self::null()
    }
}

//! Provides utilities for entities in ECS.

use self::builder::EntityBuilder;

pub mod builder;
pub mod entry;
pub mod error;
pub mod registry;

/// Unique key of the entity in ECS.
///
/// Similarly as in arenas, you can store it anywhere
/// to obtain components attached to the entity.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
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
    pub const fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }

    /// Creates the key which doesn't belong to any entity.
    ///
    /// A null key is always invalid, but an invalid key
    /// (that was removed from the world) is not a null key.
    pub const fn null() -> Self {
        Self {
            index: u32::MAX,
            generation: 0,
        }
    }

    /// Creates an empty entity builder to build a new entity with.
    pub const fn builder() -> EntityBuilder {
        EntityBuilder::empty()
    }

    /// Checks if the entity key is null.
    ///
    /// Null keys are created through the [`Entity::null()`] method or
    /// by creating default entity key.
    pub const fn is_null(self) -> bool {
        self.index == u32::MAX
    }

    /// Returns a unique index of the entity.
    ///
    /// Index itself is not a key of the entity: the same index cannot be shared
    /// between two alive entities, but it can collide for both alive and dead entities.
    pub const fn index(self) -> u32 {
        self.index
    }

    /// Returns the generation of the entity.
    ///
    /// When the entity with a given index is removed, its generation is increased.
    /// This allows to solve ABA problem and uniquely identify an entity.
    /// With a generation we can tell how many times some entity has been reused.
    pub const fn generation(self) -> u32 {
        self.generation
    }
}

impl Default for Entity {
    /// Creates default entity key, which is null.
    fn default() -> Self {
        Self::null()
    }
}

impl core::fmt::Display for Entity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}v{}", self.index, self.generation)
    }
}

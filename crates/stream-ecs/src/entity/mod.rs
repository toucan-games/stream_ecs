//! Provides utilities for entities in ECS.

use as_any::AsAny;

pub use self::default::DefaultEntity;

pub mod builder;
pub mod entry;
pub mod registry;

mod default;

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
pub trait Entity: Copy + 'static {
    /// Type of index and generation of the entity.
    type Index: Copy + 'static;

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
    /// use stream_ecs::entity::{DefaultEntity, Entity};
    ///
    /// let entity: DefaultEntity = Entity::with(0, 0);
    /// ```
    fn with(index: Self::Index, generation: Self::Index) -> Self;

    /// Returns a unique index of the entity.
    ///
    /// Index itself is not a key of the entity: the same index cannot be shared
    /// between two alive entities, but it can collide for both alive and dead entities.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::{DefaultEntity, Entity};
    ///
    /// let entity: DefaultEntity = Entity::with(42, 127);
    /// assert_eq!(entity.index(), 42);
    /// ```
    fn index(self) -> Self::Index;

    /// Returns the generation of the entity.
    ///
    /// When the entity with a given index is removed, its generation is increased.
    /// This allows to solve ABA problem and uniquely identify an entity.
    /// With a generation we can tell how many times some entity has been reused.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::{DefaultEntity, Entity};
    ///
    /// let entity: DefaultEntity = Entity::with(42, 127);
    /// assert_eq!(entity.generation(), 127);
    /// ```
    fn generation(self) -> Self::Index;

    /// Creates the key which doesn't belong to any entity.
    ///
    /// A null key is always invalid, but an invalid key
    /// (that was removed from the world) is not a null key.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::{DefaultEntity, Entity};
    ///
    /// let entity: DefaultEntity = Entity::null();
    /// assert!(entity.is_null());
    /// ```
    fn null() -> Self;

    /// Checks if the entity key is null.
    ///
    /// Null keys are created through the [`null`][null] associated function.
    ///
    /// [null]: Entity::null()
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
    fn is_null(self) -> bool;
}

/// Erased variant of entity of some entity type in ECS.
///
/// Compared to [`Entity`] trait, this trait is guaranteed to be object safe, so it can be used as trait object.
/// This trait is implemented for all the entities, so it can be used as trait object for any type of entity.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
pub trait ErasedEntity: AsAny {}

impl<T> ErasedEntity for T where T: Entity {}

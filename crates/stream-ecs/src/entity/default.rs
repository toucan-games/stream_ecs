use core::fmt::Display;

use derive_more::Display;
use num_traits::{bounds::UpperBounded, Unsigned, Zero};

use super::{builder::EntityBuilder, Entity};

/// Default type of the entity used in this crate.
///
/// This type is generic over all unsigned integers of [`core`] Rust library.
/// By default it uses [`u32`], but this could be changed by user at any time.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
#[derive(Debug, Display, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[display(bound = "Index: Display")]
#[display(fmt = "{index}v{generation}")]
pub struct DefaultEntity<Index = u32> {
    index: Index,
    generation: Index,
}

impl<Index> DefaultEntity<Index> {
    /// Creates new entity key with provided index and its generation.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::DefaultEntity;
    ///
    /// let entity = DefaultEntity::new(0, 0);
    /// ```
    pub fn new(index: Index, generation: Index) -> Self {
        Self { index, generation }
    }

    /// Creates an empty entity builder to build a new entity with.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::{builder::EntityBuilder, DefaultEntity};
    ///
    /// let builder = DefaultEntity::builder();
    /// assert_eq!(builder, EntityBuilder::empty());
    /// ```
    pub fn builder() -> EntityBuilder {
        EntityBuilder::empty()
    }

    /// Returns a unique index of the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::DefaultEntity;
    ///
    /// let entity = DefaultEntity::new(42, 127);
    /// assert_eq!(entity.index(), 42);
    /// ```
    pub fn index(self) -> Index {
        self.index
    }

    /// Returns the generation of the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::DefaultEntity;
    ///
    /// let entity = DefaultEntity::new(42, 127);
    /// assert_eq!(entity.generation(), 127);
    /// ```
    pub fn generation(self) -> Index {
        self.generation
    }
}

impl<Index> DefaultEntity<Index>
where
    Index: UpperBounded + Unsigned + Zero,
{
    /// Creates the key which doesn't belong to any entity.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::DefaultEntity;
    ///
    /// let entity = DefaultEntity::null();
    /// assert!(entity.is_null());
    /// ```
    pub fn null() -> Self {
        Self {
            index: Index::max_value(),
            generation: Index::zero(),
        }
    }
}

impl<Index> DefaultEntity<Index>
where
    Index: PartialEq + UpperBounded,
{
    /// Checks if the entity key is null.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::DefaultEntity;
    ///
    /// let entity = DefaultEntity::new(0, 0);
    /// assert!(!entity.is_null());
    ///
    /// let entity = DefaultEntity::null();
    /// assert!(entity.is_null());
    /// ```
    pub fn is_null(self) -> bool {
        self.index == Index::max_value()
    }
}

impl<Index> Entity for DefaultEntity<Index>
where
    Index: Copy + UpperBounded + Unsigned + Zero + PartialEq + 'static,
{
    type Index = Index;

    fn with(index: Self::Index, generation: Self::Index) -> Self {
        DefaultEntity::new(index, generation)
    }

    fn index(self) -> Self::Index {
        DefaultEntity::index(self)
    }

    fn generation(self) -> Self::Index {
        DefaultEntity::generation(self)
    }

    fn null() -> Self {
        DefaultEntity::null()
    }

    fn is_null(self) -> bool {
        DefaultEntity::is_null(self)
    }
}

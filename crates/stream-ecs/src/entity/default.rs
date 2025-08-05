use core::fmt::Display;

use derive_more::Display;
use num_traits::{Unsigned, Zero, bounds::UpperBounded};

use super::Entity;

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
#[display(bound(Index: Display, Generation: Display))]
#[display("{index}v{generation}")]
pub struct DefaultEntity<Index = u32, Generation = Index> {
    index: Index,
    generation: Generation,
}

impl<Index, Generation> DefaultEntity<Index, Generation> {
    /// Creates new entity key with provided index and its generation.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::DefaultEntity;
    ///
    /// let entity = DefaultEntity::new(0, 0);
    /// ```
    pub fn new(index: Index, generation: Generation) -> Self {
        Self { index, generation }
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
        let Self { index, .. } = self;
        index
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
    pub fn generation(self) -> Generation {
        let Self { generation, .. } = self;
        generation
    }
}

impl<Index, Generation> DefaultEntity<Index, Generation>
where
    Index: UpperBounded + Unsigned,
    Generation: Zero + Unsigned,
{
    /// Creates the key which doesn't belong to any entity.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::DefaultEntity;
    ///
    /// let entity: DefaultEntity = DefaultEntity::null();
    /// assert!(entity.is_null());
    /// ```
    pub fn null() -> Self {
        Self {
            index: Index::max_value(),
            generation: Generation::zero(),
        }
    }
}

impl<Index, Generation> DefaultEntity<Index, Generation>
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
    /// let entity: DefaultEntity = DefaultEntity::null();
    /// assert!(entity.is_null());
    /// ```
    pub fn is_null(self) -> bool {
        let Self { index, .. } = self;
        index == Index::max_value()
    }
}

impl<Index, Generation> Entity for DefaultEntity<Index, Generation>
where
    Index: Copy + UpperBounded + Unsigned + PartialEq + 'static,
    Generation: Copy + Zero + Unsigned + 'static,
{
    type Index = Index;
    type Generation = Generation;

    fn with(index: Self::Index, generation: Self::Generation) -> Self {
        DefaultEntity::new(index, generation)
    }

    fn index(self) -> Self::Index {
        DefaultEntity::index(self)
    }

    fn generation(self) -> Self::Generation {
        DefaultEntity::generation(self)
    }

    fn null() -> Self {
        DefaultEntity::null()
    }

    fn is_null(self) -> bool {
        DefaultEntity::is_null(self)
    }
}

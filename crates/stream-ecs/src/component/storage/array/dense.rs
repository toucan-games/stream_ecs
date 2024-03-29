//! Dense component storage implementation backed by an array.

use core::{iter::FusedIterator, mem, slice};

use arrayvec::ArrayVec;

use crate::{
    component::{
        storage::{Storage, TryStorage},
        Component,
    },
    entity::{DefaultEntity, Entity},
};

use super::ArrayStorageError;

#[derive(Debug, Clone)]
struct Dense<T, G>
where
    T: Component,
{
    index: usize,
    generation: G,
    value: T,
}

#[derive(Debug, Clone)]
enum Slot<G> {
    Occupied { dense_index: usize, generation: G },
    Free,
}

/// Dense implementation of the component storage backed by an array.
///
/// This storage stores entities and their components inline, one component after another,
/// compared to [default implementation], which can have holes in it.
/// This feature of dense storage allows to iterate over data *really* fast, as fast as with slice.
/// But it has the cost: additional space required to track indices of dense array in separate sparse array.
///
/// As the [default implementation] of array storage,
/// it can store exactly `N` components of specified type `T`.
///
/// [default implementation]: super::basic::ArrayStorage
///
/// Consider we have component which represents position of an object:
///
/// ```
/// use stream_ecs::component::{storage::array::DenseArrayStorage, Component};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Component)]
/// #[component(storage = DenseArrayStorage<Self, 10>)]
/// # #[component(crate = stream_ecs)]
/// struct Position {
///     x: f32,
///     y: f32,
/// }
/// ```
///
/// Then we can store components of this type in a dense array storage:
///
/// ```
/// # use stream_ecs::component::{storage::array::DenseArrayStorage, Component};
/// use stream_ecs::entity::DefaultEntity;
/// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
/// # #[component(storage = DenseArrayStorage<Self, 10>)]
/// # #[component(crate = stream_ecs)]
/// # struct Position {
/// #     x: f32,
/// #     y: f32,
/// # }
///
/// let mut storage = DenseArrayStorage::new();
/// let entity = DefaultEntity::new(5, 0);
///
/// storage.attach(entity, Position { x: 0.0, y: 0.0 });
/// assert!(storage.is_attached(entity));
/// ```
#[derive(Debug, Clone)]
pub struct DenseArrayStorage<T, const N: usize, E = DefaultEntity>
where
    T: Component<Storage = Self>,
    E: Entity,
{
    dense: ArrayVec<Dense<T, E::Generation>, N>,
    sparse: [Slot<E::Generation>; N],
}

impl<T, E, const N: usize> DenseArrayStorage<T, N, E>
where
    T: Component<Storage = Self>,
    E: Entity,
{
    const FREE_SLOT: Slot<E::Generation> = Slot::Free;
    const FREE_ARRAY: [Slot<E::Generation>; N] = [Self::FREE_SLOT; N];

    /// Creates new empty dense array component storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::component::storage::array::DenseArrayStorage;
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = DenseArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let storage = DenseArrayStorage::<Position, 10>::new();
    /// assert!(storage.is_empty());
    /// ```
    ///
    /// It also can be used to create globally accessible dense component storage of fixed size:
    ///
    /// ```
    /// # use stream_ecs::component::{storage::array::DenseArrayStorage, Component};
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = DenseArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    /// const STORAGE: DenseArrayStorage<Position, 10> = DenseArrayStorage::new();
    /// ```
    pub const fn new() -> Self {
        Self {
            dense: ArrayVec::new_const(),
            sparse: Self::FREE_ARRAY,
        }
    }

    /// Returns count of components which are stored in the dense array storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::{component::storage::array::DenseArrayStorage, entity::DefaultEntity};
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = DenseArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = DenseArrayStorage::new();
    ///
    /// storage.attach(DefaultEntity::new(5, 1), Position { x: 0.0, y: 0.0 });
    /// storage.attach(DefaultEntity::new(9, 6), Position { x: 10.0, y: -10.0 });
    /// assert_eq!(storage.len(), 2);
    /// ```
    pub const fn len(&self) -> usize {
        self.dense.len()
    }

    /// Returns the capacity of the dense array component storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::component::storage::array::DenseArrayStorage;
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = DenseArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let storage = DenseArrayStorage::<Position, 10>::new();
    /// assert_eq!(storage.capacity(), 10);
    /// ```
    pub const fn capacity(&self) -> usize {
        self.dense.capacity()
    }

    /// Checks if the dense array storage is empty, or has no components.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::{component::storage::array::DenseArrayStorage, entity::DefaultEntity};
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = DenseArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = DenseArrayStorage::new();
    /// assert!(storage.is_empty());
    ///
    /// storage.attach(DefaultEntity::new(0, 0), Position { x: 0.0, y: 0.0 });
    /// assert!(!storage.is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears this dense array storage, destroying all components in it.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::{component::storage::array::DenseArrayStorage, entity::DefaultEntity};
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = DenseArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = DenseArrayStorage::new();
    ///
    /// storage.attach(DefaultEntity::new(5, 1), Position { x: 0.0, y: 0.0 });
    /// storage.attach(DefaultEntity::new(9, 6), Position { x: 10.0, y: -10.0 });
    /// assert!(!storage.is_empty());
    ///
    /// storage.clear();
    /// assert!(storage.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.dense.clear();
        self.sparse = Self::FREE_ARRAY;
    }
}

impl<T, E, const N: usize> DenseArrayStorage<T, N, E>
where
    T: Component<Storage = Self>,
    E: Entity,
    E::Generation: PartialOrd,
    usize: TryFrom<E::Index>,
{
    /// Attaches provided component to the entity.
    /// Returns previous component data, or [`None`] if there was no component attached to the entity.
    ///
    /// This method reuses existing entities when provided entity
    /// is newer (its generation is greater) than an actual entity with the same index.
    ///
    /// # Panics
    ///
    /// This function will panic if provided entity index is larger than capacity of the storage.
    ///
    /// If you wish to handle an error rather than panicking,
    /// you should use [`try_attach`][Self::try_attach()] method.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::{component::storage::array::DenseArrayStorage, entity::DefaultEntity};
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = DenseArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = DenseArrayStorage::new();
    ///
    /// let entity = DefaultEntity::new(0, 0);
    /// let component = storage.attach(entity, Position { x: 10.0, y: 12.0 });
    /// assert_eq!(component, None);
    ///
    /// let entity = DefaultEntity::new(0, 1);
    /// let component = storage.attach(entity, Position { x: 0.0, y: 0.0 });
    /// assert_eq!(component, Some(Position { x: 10.0, y: 12.0 }));
    /// ```
    #[track_caller]
    pub fn attach(&mut self, entity: E, component: T) -> Option<T> {
        match self.try_attach(entity, component) {
            Ok(component) => component,
            Err(err) => panic!("{err}"),
        }
    }

    /// Tries to attach provided component to the entity.
    /// Returns previous component data, or [`None`] if there was no component attached to the entity.
    ///
    /// # Errors
    ///
    /// This function will return an error if provided entity index is larger than capacity of the storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::{component::storage::array::DenseArrayStorage, entity::DefaultEntity};
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = DenseArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = DenseArrayStorage::new();
    ///
    /// let entity = DefaultEntity::new(11, 0);
    /// let result = storage.try_attach(entity, Position { x: 0.0, y: 0.0 });
    /// assert!(result.is_err());
    /// ```
    ///
    /// This is the fallible version of [`attach`][Self::attach()] method.
    pub fn try_attach(&mut self, entity: E, component: T) -> Result<Option<T>, ArrayStorageError> {
        let Ok(index) = usize::try_from(entity.index()) else {
            return Err(ArrayStorageError);
        };
        let Some(slot) = self.sparse.get_mut(index) else {
            return Err(ArrayStorageError);
        };
        match slot {
            &mut Slot::Occupied {
                dense_index,
                ref mut generation,
            } => {
                if entity.generation() < *generation {
                    return Ok(None);
                }
                let dense = self
                    .dense
                    .get_mut(dense_index)
                    .expect("dense index should point to the valid item");
                dense.index = entity.index().try_into().map_err(|_| ArrayStorageError)?;
                dense.generation = entity.generation();
                let component = mem::replace(&mut dense.value, component);
                *generation = entity.generation();
                Ok(Some(component))
            }
            Slot::Free => {
                let dense = Dense {
                    index: entity.index().try_into().map_err(|_| ArrayStorageError)?,
                    generation: entity.generation(),
                    value: component,
                };
                if self.dense.try_push(dense).is_err() {
                    return Err(ArrayStorageError);
                }
                *slot = Slot::Occupied {
                    dense_index: self.dense.len() - 1,
                    generation: entity.generation(),
                };
                Ok(None)
            }
        }
    }
}

impl<T, E, const N: usize> DenseArrayStorage<T, N, E>
where
    T: Component<Storage = Self>,
    E: Entity,
    E::Generation: PartialEq,
    usize: TryFrom<E::Index>,
{
    /// Checks if a component is attached to provided entity.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::{component::storage::array::DenseArrayStorage, entity::DefaultEntity};
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = DenseArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = DenseArrayStorage::new();
    /// let entity = DefaultEntity::new(0, 0);
    ///
    /// storage.attach(entity, Position { x: 0.0, y: 0.0 });
    /// assert!(storage.is_attached(entity));
    ///
    /// storage.remove(entity);
    /// assert!(!storage.is_attached(entity));
    /// ```
    pub fn is_attached(&self, entity: E) -> bool {
        let Ok(index) = usize::try_from(entity.index()) else {
            return false;
        };
        let Some(slot) = self.sparse.get(index) else {
            return false;
        };
        let &Slot::Occupied { dense_index, generation } = slot else {
            return false;
        };
        let Some(_) = self.dense.get(dense_index) else {
            return false;
        };
        generation == entity.generation()
    }

    /// Retrieves a reference to the component attached to provided entity.
    /// Returns [`None`] if provided entity does not have component of such type.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::{component::storage::array::DenseArrayStorage, entity::DefaultEntity};
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = DenseArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = DenseArrayStorage::new();
    /// let entity = DefaultEntity::new(9, 12);
    ///
    /// storage.attach(entity, Position { x: 1.0, y: -1.0 });
    /// assert_eq!(storage.get(entity), Some(&Position { x: 1.0, y: -1.0 }));
    ///
    /// storage.remove(entity);
    /// assert_eq!(storage.get(entity), None);
    /// ```
    pub fn get(&self, entity: E) -> Option<&T> {
        let index = usize::try_from(entity.index()).ok()?;
        let slot = self.sparse.get(index)?;
        let &Slot::Occupied { dense_index, generation } = slot else {
            return None;
        };
        if generation != entity.generation() {
            return None;
        }
        let Dense { value, .. } = self.dense.get(dense_index)?;
        Some(value)
    }

    /// Retrieves a mutable reference to the component attached to provided entity.
    /// Returns [`None`] if provided entity does not have component of such type.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::{component::storage::array::DenseArrayStorage, entity::DefaultEntity};
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = DenseArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = DenseArrayStorage::new();
    /// let entity = DefaultEntity::new(9, 12);
    ///
    /// storage.attach(entity, Position { x: 1.0, y: -1.0 });
    /// *storage.get_mut(entity).unwrap() = Position { x: 0.0, y: 2.0 };
    /// assert_eq!(storage.get_mut(entity), Some(&mut Position { x: 0.0, y: 2.0 }));
    ///
    /// storage.remove(entity);
    /// assert_eq!(storage.get_mut(entity), None);
    /// ```
    pub fn get_mut(&mut self, entity: E) -> Option<&mut T> {
        let index = usize::try_from(entity.index()).ok()?;
        let slot = self.sparse.get(index)?;
        let &Slot::Occupied { dense_index, generation } = slot else {
            return None;
        };
        if generation != entity.generation() {
            return None;
        }
        let Dense { value, .. } = self.dense.get_mut(dense_index)?;
        Some(value)
    }

    /// Removes component from provided entity.
    /// Returns previous component data, or [`None`] if there was no component attached to the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::{component::storage::array::DenseArrayStorage, entity::DefaultEntity};
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = DenseArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = DenseArrayStorage::new();
    /// let entity = DefaultEntity::new(0, 0);
    ///
    /// let component = storage.remove(entity);
    /// assert_eq!(component, None);
    ///
    /// storage.attach(entity, Position { x: 0.0, y: -10.0 });
    /// let component = storage.remove(entity);
    /// assert_eq!(component, Some(Position { x: 0.0, y: -10.0 }));
    /// ```
    pub fn remove(&mut self, entity: E) -> Option<T> {
        let index = usize::try_from(entity.index()).ok()?;
        let slot = self.sparse.get_mut(index)?;
        let Slot::Occupied { dense_index, generation } = mem::replace(slot, Slot::Free) else {
            return None;
        };
        if entity.generation() != generation {
            *slot = Slot::Occupied {
                dense_index,
                generation,
            };
            return None;
        }
        let Dense { value, .. } = self
            .dense
            .swap_pop(dense_index)
            .expect("dense index should point to the valid item");
        if let Some(&Dense { index, .. }) = self.dense.get(dense_index) {
            let slot = self
                .sparse
                .get_mut(index)
                .expect("index should point to the valid slot");
            *slot = match slot {
                Slot::Occupied { .. } => Slot::Occupied {
                    dense_index,
                    generation,
                },
                Slot::Free => Slot::Free,
            };
        }
        Some(value)
    }
}

impl<T, E, const N: usize> DenseArrayStorage<T, N, E>
where
    T: Component<Storage = Self>,
    E: Entity,
    E::Index: TryFrom<usize>,
{
    /// Returns an iterator over entity keys with references of components attached to them.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::{component::storage::array::DenseArrayStorage, entity::DefaultEntity};
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = DenseArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = DenseArrayStorage::new();
    /// storage.attach(DefaultEntity::new(1, 0), Position { x: 0.0, y: -10.0 });
    /// storage.attach(DefaultEntity::new(7, 15), Position { x: 10.0, y: 0.0 });
    /// storage.attach(DefaultEntity::new(9, 10), Position { x: 1.0, y: 23.0 });
    ///
    /// let mut iter = storage.iter();
    /// assert_eq!(iter.next(), Some((DefaultEntity::new(1, 0), &Position { x: 0.0, y: -10.0 })));
    /// assert_eq!(iter.next(), Some((DefaultEntity::new(7, 15), &Position { x: 10.0, y: 0.0 })));
    /// assert_eq!(iter.next(), Some((DefaultEntity::new(9, 10), &Position { x: 1.0, y: 23.0 })));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<'_, T, N, E> {
        self.into_iter()
    }

    /// Returns an iterator over entity keys with mutable references of components attached to them.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::{component::storage::array::DenseArrayStorage, entity::DefaultEntity};
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = DenseArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = DenseArrayStorage::new();
    /// storage.attach(DefaultEntity::new(1, 0), Position { x: 0.0, y: -10.0 });
    /// storage.attach(DefaultEntity::new(7, 15), Position { x: 10.0, y: 0.0 });
    /// storage.attach(DefaultEntity::new(9, 10), Position { x: 1.0, y: 23.0 });
    ///
    /// let mut iter = storage.iter_mut();
    /// assert_eq!(iter.next(), Some((DefaultEntity::new(1, 0), &mut Position { x: 0.0, y: -10.0 })));
    /// assert_eq!(iter.next(), Some((DefaultEntity::new(7, 15), &mut Position { x: 10.0, y: 0.0 })));
    /// assert_eq!(iter.next(), Some((DefaultEntity::new(9, 10), &mut Position { x: 1.0, y: 23.0 })));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, T, N, E> {
        self.into_iter()
    }
}

impl<T, E, const N: usize> Default for DenseArrayStorage<T, N, E>
where
    T: Component<Storage = Self>,
    E: Entity,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, E, const N: usize> Storage for DenseArrayStorage<T, N, E>
where
    T: Component<Storage = Self>,
    E: Entity,
    E::Index: TryFrom<usize> + PartialEq,
    E::Generation: PartialOrd,
    usize: TryFrom<E::Index>,
{
    type Item = T;
    type Entity = E;

    fn attach(&mut self, entity: Self::Entity, component: Self::Item) -> Option<Self::Item> {
        DenseArrayStorage::attach(self, entity, component)
    }

    fn is_attached(&self, entity: Self::Entity) -> bool {
        DenseArrayStorage::is_attached(self, entity)
    }

    fn get(&self, entity: Self::Entity) -> Option<&Self::Item> {
        DenseArrayStorage::get(self, entity)
    }

    fn get_mut(&mut self, entity: Self::Entity) -> Option<&mut Self::Item> {
        DenseArrayStorage::get_mut(self, entity)
    }

    fn remove(&mut self, entity: Self::Entity) -> Option<Self::Item> {
        DenseArrayStorage::remove(self, entity)
    }

    fn clear(&mut self) {
        DenseArrayStorage::clear(self)
    }

    fn len(&self) -> usize {
        DenseArrayStorage::len(self)
    }

    fn is_empty(&self) -> bool {
        DenseArrayStorage::is_empty(self)
    }

    type Iter<'me> = Iter<'me, Self::Item, N, Self::Entity>
    where
        Self: 'me;

    fn iter(&self) -> Self::Iter<'_> {
        DenseArrayStorage::iter(self)
    }

    type IterMut<'me> = IterMut<'me, Self::Item, N, Self::Entity>
    where
        Self: 'me;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        DenseArrayStorage::iter_mut(self)
    }
}

impl<T, E, const N: usize> TryStorage for DenseArrayStorage<T, N, E>
where
    T: Component<Storage = Self>,
    E: Entity,
    E::Index: TryFrom<usize> + PartialEq,
    E::Generation: PartialOrd,
    usize: TryFrom<E::Index>,
{
    type Err = ArrayStorageError;

    fn try_attach(
        &mut self,
        entity: Self::Entity,
        component: Self::Item,
    ) -> Result<Option<Self::Item>, Self::Err> {
        DenseArrayStorage::try_attach(self, entity, component)
    }
}

impl<'me, T, E, const N: usize> IntoIterator for &'me DenseArrayStorage<T, N, E>
where
    T: Component<Storage = DenseArrayStorage<T, N, E>>,
    E: Entity,
    E::Index: TryFrom<usize>,
{
    type Item = (E, &'me T);

    type IntoIter = Iter<'me, T, N, E>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.dense.iter();
        Iter { iter }
    }
}

impl<'me, T, E, const N: usize> IntoIterator for &'me mut DenseArrayStorage<T, N, E>
where
    T: Component<Storage = DenseArrayStorage<T, N, E>>,
    E: Entity,
    E::Index: TryFrom<usize>,
{
    type Item = (E, &'me mut T);

    type IntoIter = IterMut<'me, T, N, E>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.dense.iter_mut();
        IterMut { iter }
    }
}

impl<T, E, const N: usize> IntoIterator for DenseArrayStorage<T, N, E>
where
    T: Component<Storage = Self>,
    E: Entity,
    E::Index: TryFrom<usize>,
{
    type Item = (E, T);

    type IntoIter = IntoIter<T, N, E>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.dense.into_iter();
        IntoIter { iter }
    }
}

/// Iterator of entities with references of components attached to them
/// in the dense array storage.
#[derive(Debug, Clone)]
pub struct Iter<'data, T, const N: usize, E = DefaultEntity>
where
    T: Component<Storage = DenseArrayStorage<T, N, E>>,
    E: Entity,
{
    iter: slice::Iter<'data, Dense<T, E::Generation>>,
}

impl<'data, T, E, const N: usize> Iterator for Iter<'data, T, N, E>
where
    T: Component<Storage = DenseArrayStorage<T, N, E>>,
    E: Entity,
    E::Index: TryFrom<usize>,
{
    type Item = (E, &'data T);

    fn next(&mut self) -> Option<Self::Item> {
        let &Dense {
            index,
            generation,
            ref value,
        } = self.iter.next()?;
        let index = index.try_into().ok()?;
        let entity = E::with(index, generation);
        Some((entity, value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T, E, const N: usize> DoubleEndedIterator for Iter<'_, T, N, E>
where
    T: Component<Storage = DenseArrayStorage<T, N, E>>,
    E: Entity,
    E::Index: TryFrom<usize>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let &Dense {
            index,
            generation,
            ref value,
        } = self.iter.next_back()?;
        let index = index.try_into().ok()?;
        let entity = E::with(index, generation);
        Some((entity, value))
    }
}

impl<T, E, const N: usize> ExactSizeIterator for Iter<'_, T, N, E>
where
    T: Component<Storage = DenseArrayStorage<T, N, E>>,
    E: Entity,
    E::Index: TryFrom<usize>,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T, E, const N: usize> FusedIterator for Iter<'_, T, N, E>
where
    T: Component<Storage = DenseArrayStorage<T, N, E>>,
    E: Entity,
    E::Index: TryFrom<usize>,
{
}

/// Iterator of entities with mutable references of components attached to them
/// in the dense array storage.
#[derive(Debug)]
pub struct IterMut<'data, T, const N: usize, E = DefaultEntity>
where
    T: Component<Storage = DenseArrayStorage<T, N, E>>,
    E: Entity,
{
    iter: slice::IterMut<'data, Dense<T, E::Generation>>,
}

impl<'data, T, E, const N: usize> Iterator for IterMut<'data, T, N, E>
where
    T: Component<Storage = DenseArrayStorage<T, N, E>>,
    E: Entity,
    E::Index: TryFrom<usize>,
{
    type Item = (E, &'data mut T);

    fn next(&mut self) -> Option<Self::Item> {
        let &mut Dense {
            index,
            generation,
            ref mut value,
        } = self.iter.next()?;
        let index = index.try_into().ok()?;
        let entity = E::with(index, generation);
        Some((entity, value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T, E, const N: usize> DoubleEndedIterator for IterMut<'_, T, N, E>
where
    T: Component<Storage = DenseArrayStorage<T, N, E>>,
    E: Entity,
    E::Index: TryFrom<usize>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let &mut Dense {
            index,
            generation,
            ref mut value,
        } = self.iter.next_back()?;
        let index = index.try_into().ok()?;
        let entity = E::with(index, generation);
        Some((entity, value))
    }
}

impl<T, E, const N: usize> ExactSizeIterator for IterMut<'_, T, N, E>
where
    T: Component<Storage = DenseArrayStorage<T, N, E>>,
    E: Entity,
    E::Index: TryFrom<usize>,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T, E, const N: usize> FusedIterator for IterMut<'_, T, N, E>
where
    T: Component<Storage = DenseArrayStorage<T, N, E>>,
    E: Entity,
    E::Index: TryFrom<usize>,
{
}

/// Iterator of entities with components attached to them in the dense array storage.
#[derive(Debug, Clone)]
pub struct IntoIter<T, const N: usize, E = DefaultEntity>
where
    T: Component<Storage = DenseArrayStorage<T, N, E>>,
    E: Entity,
{
    iter: arrayvec::IntoIter<Dense<T, E::Generation>, N>,
}

impl<T, E, const N: usize> Iterator for IntoIter<T, N, E>
where
    T: Component<Storage = DenseArrayStorage<T, N, E>>,
    E: Entity,
    E::Index: TryFrom<usize>,
{
    type Item = (E, T);

    fn next(&mut self) -> Option<Self::Item> {
        let Dense {
            index,
            generation,
            value,
        } = self.iter.next()?;
        let index = index.try_into().ok()?;
        let entity = E::with(index, generation);
        Some((entity, value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T, E, const N: usize> DoubleEndedIterator for IntoIter<T, N, E>
where
    T: Component<Storage = DenseArrayStorage<T, N, E>>,
    E: Entity,
    E::Index: TryFrom<usize>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let Dense {
            index,
            generation,
            value,
        } = self.iter.next_back()?;
        let index = index.try_into().ok()?;
        let entity = E::with(index, generation);
        Some((entity, value))
    }
}

impl<T, E, const N: usize> ExactSizeIterator for IntoIter<T, N, E>
where
    T: Component<Storage = DenseArrayStorage<T, N, E>>,
    E: Entity,
    E::Index: TryFrom<usize>,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T, E, const N: usize> FusedIterator for IntoIter<T, N, E>
where
    T: Component<Storage = DenseArrayStorage<T, N, E>>,
    E: Entity,
    E::Index: TryFrom<usize>,
{
}

#[cfg(test)]
mod tests {
    use crate::{component::Component, entity::DefaultEntity as Entity};

    use super::DenseArrayStorage;

    #[derive(Debug, Clone, Copy)]
    struct Marker;

    impl Component for Marker {
        type Storage = DenseArrayStorage<Self, 10>;
    }

    #[test]
    fn new() {
        let storage = DenseArrayStorage::<Marker, 10>::new();
        assert!(storage.is_empty());
    }

    #[test]
    fn attach() {
        let mut storage = DenseArrayStorage::new();
        let entity = Entity::new(0, 0);

        let marker = storage.attach(entity, Marker);
        assert!(marker.is_none());
        assert!(storage.is_attached(entity));
    }

    #[test]
    fn remove() {
        let mut storage = DenseArrayStorage::new();
        let entity = Entity::new(1, 0);

        storage.attach(entity, Marker);
        let marker = storage.remove(entity);
        assert!(marker.is_some());
        assert!(!storage.is_attached(entity));
    }

    #[test]
    fn reattach() {
        let mut storage = DenseArrayStorage::new();
        let entity = Entity::new(2, 0);

        let marker = storage.attach(entity, Marker);
        assert!(marker.is_none());
        let marker = storage.remove(entity);
        assert!(marker.is_some());

        let new_entity = Entity::new(2, 1);
        let marker = storage.attach(new_entity, Marker);
        assert!(marker.is_none());
        assert!(!storage.is_attached(entity));
        assert!(storage.is_attached(new_entity));
    }

    #[test]
    #[should_panic]
    fn too_many() {
        let mut storage = DenseArrayStorage::new();
        let entity = Entity::new(10, 0);
        let _marker = storage.attach(entity, Marker);
    }

    #[test]
    fn iter() {
        let mut storage = DenseArrayStorage::new();
        let _ = storage.attach(Entity::new(0, 0), Marker);
        let _ = storage.attach(Entity::new(1, 0), Marker);
        let _ = storage.attach(Entity::new(2, 0), Marker);
        let _ = storage.attach(Entity::new(3, 0), Marker);
        let _ = storage.attach(Entity::new(4, 0), Marker);
        storage.remove(Entity::new(2, 0));

        let mut iter = storage.iter();
        assert_eq!(iter.len(), 4);

        let entity = iter.find(|(entity, _)| entity.index() == 2);
        assert!(entity.is_none());
    }

    #[test]
    fn into_iter() {
        let mut storage = DenseArrayStorage::new();
        let _ = storage.attach(Entity::new(0, 0), Marker);
        let _ = storage.attach(Entity::new(1, 0), Marker);
        let _ = storage.attach(Entity::new(2, 0), Marker);
        let _ = storage.attach(Entity::new(3, 0), Marker);
        let _ = storage.attach(Entity::new(4, 0), Marker);
        storage.remove(Entity::new(2, 0));

        let mut iter = storage.into_iter();
        assert_eq!(iter.len(), 4);

        let entity = iter.find(|(entity, _)| entity.index() == 2);
        assert!(entity.is_none());
    }
}

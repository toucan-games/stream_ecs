//! Basic component storage implementation backed by an array.

use core::{
    array,
    iter::{Enumerate, FusedIterator},
    mem, slice,
};

use crate::{
    component::{
        storage::{Storage, TryStorage},
        Component,
    },
    entity::Entity,
};

use super::ArrayStorageError;

#[derive(Debug, Clone)]
enum Slot<T> {
    Free,
    Occupied { value: T, generation: u32 },
}

/// Default implementation of the component storage backed by an array.
///
/// It can store exactly `N` components of specified type `T`.
///
/// Consider we have component which represents position of an object:
///
/// ```
/// use stream_ecs::component::{storage::array::ArrayStorage, Component};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Component)]
/// #[component(storage = ArrayStorage<Self, 10>)]
/// # #[component(crate = stream_ecs)]
/// struct Position {
///     x: f32,
///     y: f32,
/// }
/// ```
///
/// Then we can store components of this type in the array storage:
///
/// ```
/// # use stream_ecs::component::{storage::array::ArrayStorage, Component};
/// use stream_ecs::entity::Entity;
/// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
/// # #[component(storage = ArrayStorage<Self, 10>)]
/// # #[component(crate = stream_ecs)]
/// # struct Position {
/// #     x: f32,
/// #     y: f32,
/// # }
///
/// let mut storage = ArrayStorage::new();
/// let entity = Entity::new(5, 0);
///
/// storage.attach(entity, Position { x: 0.0, y: 0.0 });
/// assert!(storage.is_attached(entity));
/// ```
#[derive(Debug, Clone)]
pub struct ArrayStorage<T, const N: usize>
where
    T: Component<Storage = Self>,
{
    slots: [Slot<T>; N],
    len: usize,
}

impl<T, const N: usize> ArrayStorage<T, N>
where
    T: Component<Storage = Self>,
{
    const FREE_SLOT: Slot<T> = Slot::Free;
    const FREE_ARRAY: [Slot<T>; N] = [Self::FREE_SLOT; N];

    /// Creates new empty array component storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::component::storage::array::ArrayStorage;
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = ArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let storage = ArrayStorage::<Position, 10>::new();
    /// assert!(storage.is_empty());
    /// ```
    ///
    /// It also can be used to create globally accessible component storage of fixed size:
    ///
    /// ```
    /// # use stream_ecs::component::{storage::array::ArrayStorage, Component};
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = ArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    /// const STORAGE: ArrayStorage<Position, 10> = ArrayStorage::new();
    /// ```
    pub const fn new() -> Self {
        Self {
            slots: Self::FREE_ARRAY,
            len: 0,
        }
    }

    /// Returns the capacity of the array component storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::component::storage::array::ArrayStorage;
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = ArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let storage = ArrayStorage::<Position, 10>::new();
    /// assert_eq!(storage.capacity(), 10);
    /// ```
    pub const fn capacity(&self) -> usize {
        self.slots.len()
    }

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
    /// use stream_ecs::{component::storage::array::ArrayStorage, entity::Entity};
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = ArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = ArrayStorage::new();
    ///
    /// let entity = Entity::new(0, 0);
    /// let component = storage.attach(entity, Position { x: 10.0, y: 12.0 });
    /// assert_eq!(component, None);
    ///
    /// let entity = Entity::new(0, 1);
    /// let component = storage.attach(entity, Position { x: 0.0, y: 0.0 });
    /// assert_eq!(component, Some(Position { x: 10.0, y: 12.0 }));
    /// ```
    #[track_caller]
    pub fn attach(&mut self, entity: Entity, component: T) -> Option<T> {
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
    /// use stream_ecs::{component::storage::array::ArrayStorage, entity::Entity};
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = ArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = ArrayStorage::new();
    ///
    /// let entity = Entity::new(11, 0);
    /// let result = storage.try_attach(entity, Position { x: 0.0, y: 0.0 });
    /// assert!(result.is_err());
    /// ```
    ///
    /// This is the fallible version of [`attach`][Self::attach()] method.
    pub fn try_attach(
        &mut self,
        entity: Entity,
        component: T,
    ) -> Result<Option<T>, ArrayStorageError> {
        let Ok(index) = usize::try_from(entity.index()) else {
            return Err(ArrayStorageError);
        };
        let Some(slot) = self.slots.get_mut(index) else {
            return Err(ArrayStorageError);
        };
        match slot {
            Slot::Free => {
                *slot = Slot::Occupied {
                    value: component,
                    generation: entity.generation(),
                };
                self.len += 1;
                Ok(None)
            }
            Slot::Occupied { value, generation } => {
                if entity.generation() < *generation {
                    return Ok(None);
                }
                let component = mem::replace(value, component);
                *generation = entity.generation();
                Ok(Some(component))
            }
        }
    }

    /// Checks if a component is attached to provided entity.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::{component::storage::array::ArrayStorage, entity::Entity};
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = ArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = ArrayStorage::new();
    /// let entity = Entity::new(0, 0);
    ///
    /// storage.attach(entity, Position { x: 0.0, y: 0.0 });
    /// assert!(storage.is_attached(entity));
    ///
    /// storage.remove(entity);
    /// assert!(!storage.is_attached(entity));
    /// ```
    pub fn is_attached(&self, entity: Entity) -> bool {
        let Ok(index) = usize::try_from(entity.index()) else {
            return false;
        };
        let Some(slot) = self.slots.get(index) else {
            return false;
        };
        let &Slot::Occupied { generation, .. } = slot else {
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
    /// use stream_ecs::{component::storage::array::ArrayStorage, entity::Entity};
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = ArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = ArrayStorage::new();
    /// let entity = Entity::new(9, 12);
    ///
    /// storage.attach(entity, Position { x: 1.0, y: -1.0 });
    /// assert_eq!(storage.get(entity), Some(&Position { x: 1.0, y: -1.0 }));
    ///
    /// storage.remove(entity);
    /// assert_eq!(storage.get(entity), None);
    /// ```
    pub fn get(&self, entity: Entity) -> Option<&T> {
        let index = usize::try_from(entity.index()).ok()?;
        let slot = self.slots.get(index)?;
        let &Slot::Occupied { generation, ref value } = slot else {
            return None;
        };
        if generation != entity.generation() {
            return None;
        }
        Some(value)
    }

    /// Retrieves a mutable reference to the component attached to provided entity.
    /// Returns [`None`] if provided entity does not have component of such type.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::{component::storage::array::ArrayStorage, entity::Entity};
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = ArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = ArrayStorage::new();
    /// let entity = Entity::new(9, 12);
    ///
    /// storage.attach(entity, Position { x: 1.0, y: -1.0 });
    /// *storage.get_mut(entity).unwrap() = Position { x: 0.0, y: 2.0 };
    /// assert_eq!(storage.get_mut(entity), Some(&mut Position { x: 0.0, y: 2.0 }));
    ///
    /// storage.remove(entity);
    /// assert_eq!(storage.get_mut(entity), None);
    /// ```
    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        let index = usize::try_from(entity.index()).ok()?;
        let slot = self.slots.get_mut(index)?;
        let &mut Slot::Occupied { generation, ref mut value } = slot else {
            return None;
        };
        if generation != entity.generation() {
            return None;
        }
        Some(value)
    }

    /// Removes component from provided entity.
    /// Returns previous component data, or [`None`] if there was no component attached to the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::{component::storage::array::ArrayStorage, entity::Entity};
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = ArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = ArrayStorage::new();
    /// let entity = Entity::new(0, 0);
    ///
    /// let component = storage.remove(entity);
    /// assert_eq!(component, None);
    ///
    /// storage.attach(entity, Position { x: 0.0, y: -10.0 });
    /// let component = storage.remove(entity);
    /// assert_eq!(component, Some(Position { x: 0.0, y: -10.0 }));
    /// ```
    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        let index = usize::try_from(entity.index()).ok()?;
        let slot = self.slots.get_mut(index)?;
        let Slot::Occupied { value, generation } = mem::replace(slot, Slot::Free) else {
            return None;
        };
        if entity.generation() != generation {
            *slot = Slot::Occupied { value, generation };
            return None;
        }
        self.len -= 1;
        Some(value)
    }

    /// Clears this array storage, destroying all components in it.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::{component::storage::array::ArrayStorage, entity::Entity};
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = ArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = ArrayStorage::new();
    ///
    /// storage.attach(Entity::new(5, 1), Position { x: 0.0, y: 0.0 });
    /// storage.attach(Entity::new(9, 6), Position { x: 10.0, y: -10.0 });
    /// assert!(!storage.is_empty());
    ///
    /// storage.clear();
    /// assert!(storage.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.slots = Self::FREE_ARRAY;
        self.len = 0;
    }

    /// Returns count of components which are stored in the array storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::{component::storage::array::ArrayStorage, entity::Entity};
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = ArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = ArrayStorage::new();
    ///
    /// storage.attach(Entity::new(5, 1), Position { x: 0.0, y: 0.0 });
    /// storage.attach(Entity::new(9, 6), Position { x: 10.0, y: -10.0 });
    /// assert_eq!(storage.len(), 2);
    /// ```
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Checks if the array storage is empty, or has no components.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::{component::storage::array::ArrayStorage, entity::Entity};
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = ArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = ArrayStorage::new();
    /// assert!(storage.is_empty());
    ///
    /// storage.attach(Entity::new(0, 0), Position { x: 0.0, y: 0.0 });
    /// assert!(!storage.is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over entity keys with references of components attached to them.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::{component::storage::array::ArrayStorage, entity::Entity};
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = ArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = ArrayStorage::new();
    /// storage.attach(Entity::new(1, 0), Position { x: 0.0, y: -10.0 });
    /// storage.attach(Entity::new(7, 15), Position { x: 10.0, y: 0.0 });
    /// storage.attach(Entity::new(9, 10), Position { x: 1.0, y: 23.0 });
    ///
    /// let mut iter = storage.iter();
    /// assert_eq!(iter.next(), Some((Entity::new(1, 0), &Position { x: 0.0, y: -10.0 })));
    /// assert_eq!(iter.next(), Some((Entity::new(7, 15), &Position { x: 10.0, y: 0.0 })));
    /// assert_eq!(iter.next(), Some((Entity::new(9, 10), &Position { x: 1.0, y: 23.0 })));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        self.into_iter()
    }

    /// Returns an iterator over entity keys with mutable references of components attached to them.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::{component::storage::array::ArrayStorage, entity::Entity};
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = ArrayStorage<Self, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = ArrayStorage::new();
    /// storage.attach(Entity::new(1, 0), Position { x: 0.0, y: -10.0 });
    /// storage.attach(Entity::new(7, 15), Position { x: 10.0, y: 0.0 });
    /// storage.attach(Entity::new(9, 10), Position { x: 1.0, y: 23.0 });
    ///
    /// let mut iter = storage.iter_mut();
    /// assert_eq!(iter.next(), Some((Entity::new(1, 0), &mut Position { x: 0.0, y: -10.0 })));
    /// assert_eq!(iter.next(), Some((Entity::new(7, 15), &mut Position { x: 10.0, y: 0.0 })));
    /// assert_eq!(iter.next(), Some((Entity::new(9, 10), &mut Position { x: 1.0, y: 23.0 })));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.into_iter()
    }
}

impl<T, const N: usize> Default for ArrayStorage<T, N>
where
    T: Component<Storage = Self>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Storage for ArrayStorage<T, N>
where
    T: Component<Storage = Self>,
{
    type Item = T;

    fn attach(&mut self, entity: Entity, component: Self::Item) -> Option<Self::Item> {
        ArrayStorage::attach(self, entity, component)
    }

    fn is_attached(&self, entity: Entity) -> bool {
        ArrayStorage::is_attached(self, entity)
    }

    fn get(&self, entity: Entity) -> Option<&Self::Item> {
        ArrayStorage::get(self, entity)
    }

    fn get_mut(&mut self, entity: Entity) -> Option<&mut Self::Item> {
        ArrayStorage::get_mut(self, entity)
    }

    fn remove(&mut self, entity: Entity) -> Option<Self::Item> {
        ArrayStorage::remove(self, entity)
    }

    fn clear(&mut self) {
        ArrayStorage::clear(self)
    }

    fn len(&self) -> usize {
        ArrayStorage::len(self)
    }

    fn is_empty(&self) -> bool {
        ArrayStorage::is_empty(self)
    }

    type Iter<'me> = Iter<'me, T>
    where
        Self: 'me;

    fn iter(&self) -> Self::Iter<'_> {
        ArrayStorage::iter(self)
    }

    type IterMut<'me> = IterMut<'me, T>
    where
        Self: 'me;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        ArrayStorage::iter_mut(self)
    }
}

impl<T, const N: usize> TryStorage for ArrayStorage<T, N>
where
    T: Component<Storage = Self>,
{
    type Err = ArrayStorageError;

    fn try_attach(
        &mut self,
        entity: Entity,
        component: Self::Item,
    ) -> Result<Option<Self::Item>, Self::Err> {
        ArrayStorage::try_attach(self, entity, component)
    }
}

impl<'me, T, const N: usize> IntoIterator for &'me ArrayStorage<T, N>
where
    T: Component<Storage = ArrayStorage<T, N>>,
{
    type Item = (Entity, &'me T);

    type IntoIter = Iter<'me, T>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.slots.iter().enumerate();
        let num_left = self.len;
        Iter { iter, num_left }
    }
}

impl<'me, T, const N: usize> IntoIterator for &'me mut ArrayStorage<T, N>
where
    T: Component<Storage = ArrayStorage<T, N>>,
{
    type Item = (Entity, &'me mut T);

    type IntoIter = IterMut<'me, T>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.slots.iter_mut().enumerate();
        let num_left = self.len;
        IterMut { iter, num_left }
    }
}

impl<T, const N: usize> IntoIterator for ArrayStorage<T, N>
where
    T: Component<Storage = Self>,
{
    type Item = (Entity, T);

    type IntoIter = IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.slots.into_iter().enumerate();
        let num_left = self.len;
        IntoIter { iter, num_left }
    }
}

/// Iterator of entities with references of components attached to them
/// in the array storage.
#[derive(Debug, Clone)]
pub struct Iter<'data, T>
where
    T: Component,
{
    iter: Enumerate<slice::Iter<'data, Slot<T>>>,
    num_left: usize,
}

impl<'data, T> Iterator for Iter<'data, T>
where
    T: Component,
{
    type Item = (Entity, &'data T);

    fn next(&mut self) -> Option<Self::Item> {
        let item = loop {
            let (index, slot) = self.iter.next()?;
            let &Slot::Occupied { ref value, generation } = slot else {
                continue;
            };
            let index = index.try_into().ok()?;
            let entity = Entity::new(index, generation);
            self.num_left -= 1;
            break (entity, value);
        };
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for Iter<'_, T>
where
    T: Component,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let item = loop {
            let (index, slot) = self.iter.next_back()?;
            let &Slot::Occupied { ref value, generation } = slot else {
                continue;
            };
            let index = index.try_into().ok()?;
            let entity = Entity::new(index, generation);
            self.num_left -= 1;
            break (entity, value);
        };
        Some(item)
    }
}

impl<T> ExactSizeIterator for Iter<'_, T>
where
    T: Component,
{
    fn len(&self) -> usize {
        self.num_left
    }
}

impl<T> FusedIterator for Iter<'_, T> where T: Component {}

/// Iterator of entities with mutable references of components attached to them
/// in the array storage.
#[derive(Debug)]
pub struct IterMut<'data, T>
where
    T: Component,
{
    iter: Enumerate<slice::IterMut<'data, Slot<T>>>,
    num_left: usize,
}

impl<'data, T> Iterator for IterMut<'data, T>
where
    T: Component,
{
    type Item = (Entity, &'data mut T);

    fn next(&mut self) -> Option<Self::Item> {
        let item = loop {
            let (index, slot) = self.iter.next()?;
            let &mut Slot::Occupied { ref mut value, generation } = slot else {
                continue;
            };
            let index = index.try_into().ok()?;
            let entity = Entity::new(index, generation);
            self.num_left -= 1;
            break (entity, value);
        };
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for IterMut<'_, T>
where
    T: Component,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let item = loop {
            let (index, slot) = self.iter.next_back()?;
            let &mut Slot::Occupied { ref mut value, generation } = slot else {
                continue;
            };
            let index = index.try_into().ok()?;
            let entity = Entity::new(index, generation);
            self.num_left -= 1;
            break (entity, value);
        };
        Some(item)
    }
}

impl<T> ExactSizeIterator for IterMut<'_, T>
where
    T: Component,
{
    fn len(&self) -> usize {
        self.num_left
    }
}

impl<T> FusedIterator for IterMut<'_, T> where T: Component {}

/// Iterator of entities with components attached to them in the array storage.
#[derive(Debug, Clone)]
pub struct IntoIter<T, const N: usize>
where
    T: Component,
{
    iter: Enumerate<array::IntoIter<Slot<T>, N>>,
    num_left: usize,
}

impl<T, const N: usize> Iterator for IntoIter<T, N>
where
    T: Component,
{
    type Item = (Entity, T);

    fn next(&mut self) -> Option<Self::Item> {
        let item = loop {
            let (index, slot) = self.iter.next()?;
            let Slot::Occupied { value, generation } = slot else {
                continue;
            };
            let index = index.try_into().ok()?;
            let entity = Entity::new(index, generation);
            self.num_left -= 1;
            break (entity, value);
        };
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T, const N: usize> DoubleEndedIterator for IntoIter<T, N>
where
    T: Component,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let item = loop {
            let (index, slot) = self.iter.next_back()?;
            let Slot::Occupied { value, generation } = slot else {
                continue;
            };
            let index = index.try_into().ok()?;
            let entity = Entity::new(index, generation);
            self.num_left -= 1;
            break (entity, value);
        };
        Some(item)
    }
}

impl<T, const N: usize> ExactSizeIterator for IntoIter<T, N>
where
    T: Component,
{
    fn len(&self) -> usize {
        self.num_left
    }
}

impl<T, const N: usize> FusedIterator for IntoIter<T, N> where T: Component {}

#[cfg(test)]
mod tests {
    use crate::{component::Component, entity::Entity};

    use super::ArrayStorage;

    #[derive(Debug, Clone, Copy)]
    struct Marker;

    impl Component for Marker {
        type Storage = ArrayStorage<Self, 10>;
    }

    #[test]
    fn new() {
        let storage = ArrayStorage::<Marker, 10>::new();
        assert!(storage.is_empty());
    }

    #[test]
    fn attach() {
        let mut storage = ArrayStorage::new();
        let entity = Entity::new(0, 0);

        let marker = storage.attach(entity, Marker);
        assert!(marker.is_none());
        assert!(storage.is_attached(entity));
    }

    #[test]
    fn remove() {
        let mut storage = ArrayStorage::new();
        let entity = Entity::new(1, 0);

        storage.attach(entity, Marker);
        let marker = storage.remove(entity);
        assert!(marker.is_some());
        assert!(!storage.is_attached(entity));
    }

    #[test]
    fn reattach() {
        let mut storage = ArrayStorage::new();
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
        let mut storage = ArrayStorage::new();
        let entity = Entity::new(10, 0);
        let _marker = storage.attach(entity, Marker);
    }

    #[test]
    fn iter() {
        let mut storage = ArrayStorage::new();
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
        let mut storage = ArrayStorage::new();
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

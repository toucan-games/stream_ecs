//! Dense component storage implementation backed by an array.

use core::{iter::FusedIterator, mem, slice};

use arrayvec::ArrayVec;

use crate::{
    component::{
        storage::{Storage, TryStorage},
        Component,
    },
    entity::Entity,
};

use super::ArrayStorageError;

#[derive(Debug, Clone)]
struct Dense<T>
where
    T: Component,
{
    entity: Entity,
    value: T,
}

#[derive(Debug, Clone)]
enum Slot {
    Occupied { dense_index: u32, generation: u32 },
    Free,
}

/// Dense implementation of the component storage backed by an array.
#[derive(Debug, Clone)]
pub struct DenseArrayStorage<T, const N: usize>
where
    T: Component,
{
    dense: ArrayVec<Dense<T>, N>,
    sparse: [Slot; N],
}

impl<T, const N: usize> DenseArrayStorage<T, N>
where
    T: Component,
{
    const FREE_SLOT: Slot = Slot::Free;
    const FREE_ARRAY: [Slot; N] = [Self::FREE_SLOT; N];

    /// Creates new empty dense array component storage.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub const fn new() -> Self {
        Self {
            dense: ArrayVec::new_const(),
            sparse: Self::FREE_ARRAY,
        }
    }

    /// Returns the capacity of the dense array component storage.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub const fn capacity(&self) -> usize {
        self.dense.capacity()
    }

    /// Attaches provided component to the entity.
    /// Returns previous component data, or [`None`] if there was no component attached to the entity.
    ///
    /// This method reuses existing entities when provided entity
    /// is newer (its generation is greater) than an actual entity with the same index.
    ///
    /// # Panics
    ///
    /// This function will panic if the count of components attached to some entities
    /// is the same as the capacity of the storage.
    ///
    /// If you wish to handle an error rather than panicking,
    /// you should use [`try_attach`][Self::try_attach()] method.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
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
    /// This function will return an error if the count of components attached to some entities
    /// is the same as the capacity of the storage.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
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
                    .get_mut(dense_index as usize)
                    .expect("dense index should point to the valid item");
                let component = mem::replace(&mut dense.value, component);
                dense.entity = entity;
                *generation = entity.generation();
                Ok(Some(component))
            }
            Slot::Free => {
                let dense = Dense {
                    entity,
                    value: component,
                };
                if self.dense.try_push(dense).is_err() {
                    return Err(ArrayStorageError);
                }
                *slot = Slot::Occupied {
                    dense_index: self.dense.len() as u32 - 1,
                    generation: entity.generation(),
                };
                Ok(None)
            }
        }
    }

    /// Checks if a component is attached to provided entity.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn is_attached(&self, entity: Entity) -> bool {
        let Ok(index) = usize::try_from(entity.index()) else {
            return false;
        };
        let Some(slot) = self.sparse.get(index) else {
            return false;
        };
        let &Slot::Occupied { dense_index, generation } = slot else {
            return false;
        };
        let Some(_) = self.dense.get(dense_index as usize) else {
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
    /// todo!()
    /// ```
    pub fn get(&self, entity: Entity) -> Option<&T> {
        let index = usize::try_from(entity.index()).ok()?;
        let slot = self.sparse.get(index)?;
        let &Slot::Occupied { dense_index, generation } = slot else {
            return None;
        };
        if generation != entity.generation() {
            return None;
        }
        let Dense { value, .. } = self.dense.get(dense_index as usize)?;
        Some(value)
    }

    /// Retrieves a mutable reference to the component attached to provided entity.
    /// Returns [`None`] if provided entity does not have component of such type.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        let index = usize::try_from(entity.index()).ok()?;
        let slot = self.sparse.get(index)?;
        let &Slot::Occupied { dense_index, generation } = slot else {
            return None;
        };
        if generation != entity.generation() {
            return None;
        }
        let Dense { value, .. } = self.dense.get_mut(dense_index as usize)?;
        Some(value)
    }

    /// Removes component from provided entity.
    /// Returns previous component data, or [`None`] if there was no component attached to the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn remove(&mut self, entity: Entity) -> Option<T> {
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
            .swap_pop(dense_index as usize)
            .expect("dense index should point to the valid item");
        if let Some(Dense { entity, .. }) = self.dense.get(dense_index as usize) {
            let slot = self
                .sparse
                .get_mut(entity.index() as usize)
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

    /// Clears this dense array storage, destroying all components in it.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn clear(&mut self) {
        self.dense.clear();
        self.sparse = Self::FREE_ARRAY;
    }

    /// Returns count of components which are stored in the dense array storage.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub const fn len(&self) -> usize {
        self.dense.len()
    }

    /// Checks if the dense array storage is empty, or has no components.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over entity keys with references of components attached to them.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        self.into_iter()
    }

    /// Returns an iterator over entity keys with mutable references of components attached to them.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.into_iter()
    }
}

impl<T, const N: usize> Default for DenseArrayStorage<T, N>
where
    T: Component,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Storage for DenseArrayStorage<T, N>
where
    T: Component,
{
    type Item = T;

    fn attach(&mut self, entity: Entity, component: Self::Item) -> Option<Self::Item> {
        DenseArrayStorage::attach(self, entity, component)
    }

    fn is_attached(&self, entity: Entity) -> bool {
        DenseArrayStorage::is_attached(self, entity)
    }

    fn get(&self, entity: Entity) -> Option<&Self::Item> {
        DenseArrayStorage::get(self, entity)
    }

    fn get_mut(&mut self, entity: Entity) -> Option<&mut Self::Item> {
        DenseArrayStorage::get_mut(self, entity)
    }

    fn remove(&mut self, entity: Entity) -> Option<Self::Item> {
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

    type Iter<'a> = Iter<'a, T>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        DenseArrayStorage::iter(self)
    }

    type IterMut<'a> = IterMut<'a, T>
    where
        Self: 'a;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        DenseArrayStorage::iter_mut(self)
    }
}

impl<T, const N: usize> TryStorage for DenseArrayStorage<T, N>
where
    T: Component,
{
    type Err = ArrayStorageError;

    fn try_attach(
        &mut self,
        entity: Entity,
        component: Self::Item,
    ) -> Result<Option<Self::Item>, Self::Err> {
        DenseArrayStorage::try_attach(self, entity, component)
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a DenseArrayStorage<T, N>
where
    T: Component,
{
    type Item = (Entity, &'a T);

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.dense.iter();
        Iter { iter }
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut DenseArrayStorage<T, N>
where
    T: Component,
{
    type Item = (Entity, &'a mut T);

    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.dense.iter_mut();
        IterMut { iter }
    }
}

impl<T, const N: usize> IntoIterator for DenseArrayStorage<T, N>
where
    T: Component,
{
    type Item = (Entity, T);

    type IntoIter = IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.dense.into_iter();
        IntoIter { iter }
    }
}

/// Iterator of entities with references of components attached to them
/// in the dense array storage.
#[derive(Debug, Clone)]
pub struct Iter<'a, T>
where
    T: Component,
{
    iter: slice::Iter<'a, Dense<T>>,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Component,
{
    type Item = (Entity, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let &Dense { entity, ref value } = self.iter.next()?;
        Some((entity, value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for Iter<'_, T>
where
    T: Component,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let &Dense { entity, ref value } = self.iter.next_back()?;
        Some((entity, value))
    }
}

impl<T> ExactSizeIterator for Iter<'_, T>
where
    T: Component,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T> FusedIterator for Iter<'_, T> where T: Component {}

/// Iterator of entities with mutable references of components attached to them
/// in the dense array storage.
#[derive(Debug)]
pub struct IterMut<'a, T>
where
    T: Component,
{
    iter: slice::IterMut<'a, Dense<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T>
where
    T: Component,
{
    type Item = (Entity, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        let &mut Dense {
            entity,
            ref mut value,
        } = self.iter.next()?;
        Some((entity, value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for IterMut<'_, T>
where
    T: Component,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let &mut Dense {
            entity,
            ref mut value,
        } = self.iter.next_back()?;
        Some((entity, value))
    }
}

impl<T> ExactSizeIterator for IterMut<'_, T>
where
    T: Component,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T> FusedIterator for IterMut<'_, T> where T: Component {}

/// Iterator of entities with components attached to them in the dense array storage.
#[derive(Debug, Clone)]
pub struct IntoIter<T, const N: usize>
where
    T: Component,
{
    iter: arrayvec::IntoIter<Dense<T>, N>,
}

impl<T, const N: usize> Iterator for IntoIter<T, N>
where
    T: Component,
{
    type Item = (Entity, T);

    fn next(&mut self) -> Option<Self::Item> {
        let Dense { entity, value } = self.iter.next()?;
        Some((entity, value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T, const N: usize> DoubleEndedIterator for IntoIter<T, N>
where
    T: Component,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let Dense { entity, value } = self.iter.next_back()?;
        Some((entity, value))
    }
}

impl<T, const N: usize> ExactSizeIterator for IntoIter<T, N>
where
    T: Component,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T, const N: usize> FusedIterator for IntoIter<T, N> where T: Component {}

#[cfg(test)]
mod tests {
    use crate::{component::Component, entity::Entity};

    use super::DenseArrayStorage;

    #[derive(Debug, Clone, Copy)]
    struct Marker;

    impl Component for Marker {
        type Storage = DenseArrayStorage<Self, 0>;
    }

    #[test]
    fn new() {
        let storage = DenseArrayStorage::<Marker, 10>::new();
        assert!(storage.is_empty());
    }

    #[test]
    fn attach() {
        let mut storage = DenseArrayStorage::<Marker, 10>::new();
        let entity = Entity::new(0, 0);

        let marker = storage.attach(entity, Marker);
        assert!(marker.is_none());
        assert!(storage.is_attached(entity));
    }

    #[test]
    fn remove() {
        let mut storage = DenseArrayStorage::<Marker, 10>::new();
        let entity = Entity::new(1, 0);

        storage.attach(entity, Marker);
        let marker = storage.remove(entity);
        assert!(marker.is_some());
        assert!(!storage.is_attached(entity));
    }

    #[test]
    fn reattach() {
        let mut storage = DenseArrayStorage::<Marker, 10>::new();
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
        let mut storage = DenseArrayStorage::<Marker, 2>::new();
        let entity = Entity::new(2, 0);
        let _marker = storage.attach(entity, Marker);
    }

    #[test]
    fn iter() {
        let mut storage = DenseArrayStorage::<Marker, 10>::new();
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
        let mut storage = DenseArrayStorage::<Marker, 10>::new();
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

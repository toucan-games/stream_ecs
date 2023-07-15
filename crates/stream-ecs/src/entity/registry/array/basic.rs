//! Basic entity registry implementation backed by an array.

use core::{
    iter::{Enumerate, FusedIterator},
    ops::Add,
    slice,
};

use arrayvec::ArrayVec;

use crate::entity::{
    registry::{NotPresentError, Registry, TryRegistry},
    DefaultEntity, Entity,
};

use super::ArrayRegistryError;

#[derive(Debug, Clone)]
enum SlotEntry<T> {
    Free { next_free: usize },
    Occupied { value: T },
}

#[derive(Debug, Clone)]
struct Slot<T, I> {
    entry: SlotEntry<T>,
    generation: I,
}

/// Default implementation of the entity registry backed by an array.
///
/// # Examples
///
/// ```
/// todo!()
/// ```
#[derive(Debug, Clone, Default)]
pub struct ArrayRegistry<const N: usize, E = DefaultEntity>
where
    E: Entity,
{
    slots: ArrayVec<Slot<(), E::Index>, N>,
    free_head: usize,
    len: usize,
}

impl<E, const N: usize> ArrayRegistry<N, E>
where
    E: Entity,
{
    /// Creates new empty array entity registry.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::registry::array::ArrayRegistry;
    ///
    /// let registry = ArrayRegistry::<10>::new();
    /// assert!(registry.is_empty());
    /// ```
    ///
    /// It also can be used to create globally accessible entity registry of fixed size:
    ///
    /// ```
    /// # use stream_ecs::entity::registry::array::ArrayRegistry;
    /// const REGISTRY: ArrayRegistry<1024> = ArrayRegistry::new();
    /// ```
    pub const fn new() -> Self {
        Self {
            slots: ArrayVec::new_const(),
            free_head: 0,
            len: 0,
        }
    }

    /// Returns count of currently alive entities of the array registry.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::registry::array::ArrayRegistry;
    ///
    /// let mut registry = ArrayRegistry::<10>::new();
    /// let _ = registry.create();
    /// let _ = registry.create();
    /// assert_eq!(registry.len(), 2);
    /// ```
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns the capacity of the array registry.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::registry::array::ArrayRegistry;
    ///
    /// let registry = ArrayRegistry::<1024>::new();
    /// assert_eq!(registry.capacity(), 1024);
    /// ```
    pub const fn capacity(&self) -> usize {
        self.slots.capacity()
    }

    /// Checks if the array registry contains no alive entities.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::registry::array::ArrayRegistry;
    ///
    /// let mut registry = ArrayRegistry::<10>::new();
    /// assert!(registry.is_empty());
    ///
    /// let entity = registry.create();
    /// assert!(!registry.is_empty());
    ///
    /// registry.destroy(entity).unwrap();
    /// assert!(registry.is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears the array registry, destroying all the entities in it.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::registry::array::ArrayRegistry;
    ///
    /// let mut registry = ArrayRegistry::<2>::new();
    /// let first = registry.create();
    /// let second = registry.create();
    /// assert!(!registry.is_empty());
    ///
    /// registry.clear();
    /// assert!(registry.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.slots.clear();
        self.free_head = 0;
        self.len = 0;
    }
}

impl<E, const N: usize> ArrayRegistry<N, E>
where
    E: Entity,
    E::Index: TryFrom<usize>,
{
    /// Creates new entity in the array registry.
    ///
    /// This method reuses indices from destroyed entities, but the resulting key is unique.
    ///
    /// # Panics
    ///
    /// This function will panic if the count of already created entities
    /// is the same as the capacity of the registry.
    ///
    /// If you wish to handle an error rather than panicking,
    /// you should use [`try_create`][Self::try_create()] method.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::registry::array::ArrayRegistry;
    ///
    /// let mut registry = ArrayRegistry::<2>::new();
    /// let first = registry.create();
    /// let second = registry.create();
    /// assert_ne!(first, second);
    /// ```
    #[track_caller]
    pub fn create(&mut self) -> E {
        match self.try_create() {
            Ok(entity) => entity,
            Err(err) => panic!("{err}"),
        }
    }

    /// Tries to create new entity in the array registry.
    ///
    /// # Errors
    ///
    /// This function will return an error if the count of already created entities
    /// is the same as the capacity of the registry.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::registry::array::ArrayRegistry;
    ///
    /// let mut registry = ArrayRegistry::<2>::new();
    /// let _ = registry.try_create().unwrap();
    /// let _ = registry.try_create().unwrap();
    /// let entity = registry.try_create();
    /// assert!(entity.is_err());
    /// ```
    ///
    /// This is the fallible version of [`create`][Self::create()] method.
    pub fn try_create(&mut self) -> Result<E, ArrayRegistryError> {
        let entity = if let Some(slot) = self.slots.get_mut(self.free_head) {
            if let SlotEntry::Free { next_free } = slot.entry {
                let index = self.free_head.try_into().map_err(|_| ArrayRegistryError)?;
                let entity = E::with(index, slot.generation);
                self.free_head = next_free;
                slot.entry = SlotEntry::Occupied { value: () };
                entity
            } else {
                unreachable!("free head must not point to the occupied entry")
            }
        } else {
            let index = self.len.try_into().map_err(|_| ArrayRegistryError)?;
            let generation = 0.try_into().map_err(|_| ArrayRegistryError)?;
            let entity = E::with(index, generation);
            let slot = Slot {
                entry: SlotEntry::Occupied { value: () },
                generation,
            };
            if self.slots.try_push(slot).is_err() {
                return Err(ArrayRegistryError);
            }
            self.free_head = self.len + 1;
            entity
        };
        self.len += 1;
        Ok(entity)
    }
}

impl<E, const N: usize> ArrayRegistry<N, E>
where
    E: Entity,
    E::Index: PartialEq,
    usize: TryFrom<E::Index>,
{
    /// Checks if the array registry contains provided entity.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::registry::array::ArrayRegistry;
    ///
    /// let mut registry = ArrayRegistry::<10>::new();
    /// let entity = registry.create();
    /// assert!(registry.contains(entity));
    ///
    /// registry.destroy(entity).unwrap();
    /// assert!(!registry.contains(entity))
    /// ```
    pub fn contains(&self, entity: E) -> bool {
        let Ok(index) = usize::try_from(entity.index()) else {
            return false;
        };
        let Some(slot) = self.slots.get(index) else {
            return false;
        };
        let &Slot {
            ref entry,
            generation,
        } = slot;
        if let SlotEntry::Free { .. } = entry {
            return false;
        }
        generation == entity.generation()
    }
}

impl<E, const N: usize> ArrayRegistry<N, E>
where
    E: Entity,
    E::Index: TryFrom<usize> + PartialEq + Add<Output = E::Index>,
    usize: TryFrom<E::Index>,
{
    /// Destroys provided entity which was previously created in the array registry.
    ///
    /// # Errors
    ///
    /// This function will return an error if provided entity
    /// was destroyed earlier or was not created in the registry.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::registry::array::ArrayRegistry;
    ///
    /// let mut registry = ArrayRegistry::<10>::new();
    /// let entity = registry.create();
    ///
    /// let result = registry.destroy(entity);
    /// assert!(result.is_ok());
    ///
    /// let result = registry.destroy(entity);
    /// assert!(result.is_err());
    /// ```
    pub fn destroy(&mut self, entity: E) -> Result<(), NotPresentError<E>> {
        let Ok(index) = usize::try_from(entity.index()) else {
            return Err(NotPresentError::new(entity));
        };
        let Some(slot) = self.slots.get_mut(index) else {
            return Err(NotPresentError::new(entity));
        };
        let SlotEntry::Occupied { value } = slot.entry else {
            return Err(NotPresentError::new(entity));
        };
        if slot.generation != entity.generation() {
            return Err(NotPresentError::new(entity));
        }
        slot.generation = {
            let Ok(one) = 1.try_into() else {
                return Err(NotPresentError::new(entity));
            };
            slot.generation + one
        };
        slot.entry = SlotEntry::Free {
            next_free: self.free_head,
        };
        self.free_head = index;
        self.len -= 1;
        Ok(value)
    }
}

impl<E, const N: usize> ArrayRegistry<N, E>
where
    E: Entity,
    E::Index: TryFrom<usize>,
{
    /// Returns an iterator of alive entities created by the array registry.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::registry::array::ArrayRegistry;
    ///
    /// let mut registry = ArrayRegistry::<2>::new();
    /// let first = registry.create();
    /// let second = registry.create();
    ///
    /// for entity in registry.iter() {
    ///     println!("entity is {entity}");
    /// }
    /// ```
    pub fn iter(&self) -> Iter<'_, E> {
        self.into_iter()
    }
}

impl<E, const N: usize> Registry for ArrayRegistry<N, E>
where
    E: Entity,
    E::Index: TryFrom<usize> + PartialEq + Add<Output = E::Index>,
    usize: TryFrom<E::Index>,
{
    type Entity = E;

    fn create(&mut self) -> Self::Entity {
        ArrayRegistry::create(self)
    }

    fn contains(&self, entity: Self::Entity) -> bool {
        ArrayRegistry::contains(self, entity)
    }

    fn destroy(&mut self, entity: Self::Entity) -> Result<(), NotPresentError<Self::Entity>> {
        ArrayRegistry::destroy(self, entity)
    }

    fn len(&self) -> usize {
        ArrayRegistry::len(self)
    }

    fn is_empty(&self) -> bool {
        ArrayRegistry::is_empty(self)
    }

    fn clear(&mut self) {
        ArrayRegistry::clear(self)
    }

    type Iter<'me> = Iter<'me, Self::Entity>
    where
        Self: 'me;

    fn iter(&self) -> Self::Iter<'_> {
        ArrayRegistry::iter(self)
    }
}

impl<E, const N: usize> TryRegistry for ArrayRegistry<N, E>
where
    E: Entity,
    E::Index: TryFrom<usize> + PartialEq + Add<Output = E::Index>,
    usize: TryFrom<E::Index>,
{
    type Err = ArrayRegistryError;

    fn try_create(&mut self) -> Result<Self::Entity, Self::Err> {
        ArrayRegistry::try_create(self)
    }
}

impl<'me, E, const N: usize> IntoIterator for &'me ArrayRegistry<N, E>
where
    E: Entity,
    E::Index: TryFrom<usize>,
{
    type Item = E;

    type IntoIter = Iter<'me, E>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.slots.iter().enumerate();
        let num_left = self.len;
        Iter { iter, num_left }
    }
}

impl<E, const N: usize> IntoIterator for ArrayRegistry<N, E>
where
    E: Entity,
    E::Index: TryFrom<usize>,
{
    type Item = E;

    type IntoIter = IntoIter<E, N>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.slots.into_iter().enumerate();
        let num_left = self.len;
        IntoIter { iter, num_left }
    }
}

/// Iterator over alive entities contained in the array registry.
#[derive(Debug, Clone)]
pub struct Iter<'data, E>
where
    E: Entity,
{
    iter: Enumerate<slice::Iter<'data, Slot<(), E::Index>>>,
    num_left: usize,
}

impl<E> Iterator for Iter<'_, E>
where
    E: Entity,
    E::Index: TryFrom<usize>,
{
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        let entity = loop {
            let (index, slot) = self.iter.next()?;
            let index = index.try_into().ok()?;
            let &Slot {
                ref entry,
                generation,
            } = slot;
            if let SlotEntry::Free { .. } = entry {
                continue;
            }
            self.num_left -= 1;
            break E::with(index, generation);
        };
        Some(entity)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.num_left;
        (len, Some(len))
    }
}

impl<E> DoubleEndedIterator for Iter<'_, E>
where
    E: Entity,
    E::Index: TryFrom<usize>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let entity = loop {
            let (index, slot) = self.iter.next_back()?;
            let index = index.try_into().ok()?;
            let &Slot {
                ref entry,
                generation,
            } = slot;
            if let SlotEntry::Free { .. } = entry {
                continue;
            }
            self.num_left -= 1;
            break E::with(index, generation);
        };
        Some(entity)
    }
}

impl<E> ExactSizeIterator for Iter<'_, E>
where
    E: Entity,
    E::Index: TryFrom<usize>,
{
    fn len(&self) -> usize {
        self.num_left
    }
}

impl<E> FusedIterator for Iter<'_, E>
where
    E: Entity,
    E::Index: TryFrom<usize>,
{
}

/// Type of iterator in which array registry can be converted.
#[derive(Debug, Clone)]
pub struct IntoIter<E, const N: usize>
where
    E: Entity,
{
    iter: Enumerate<arrayvec::IntoIter<Slot<(), E::Index>, N>>,
    num_left: usize,
}

impl<E, const N: usize> Iterator for IntoIter<E, N>
where
    E: Entity,
    E::Index: TryFrom<usize>,
{
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        let entity = loop {
            let (index, slot) = self.iter.next()?;
            let index = index.try_into().ok()?;
            let Slot { entry, generation } = slot;
            if let SlotEntry::Free { .. } = entry {
                continue;
            }
            self.num_left -= 1;
            break E::with(index, generation);
        };
        Some(entity)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.num_left;
        (len, Some(len))
    }
}

impl<E, const N: usize> DoubleEndedIterator for IntoIter<E, N>
where
    E: Entity,
    E::Index: TryFrom<usize>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let entity = loop {
            let (index, slot) = self.iter.next_back()?;
            let index = index.try_into().ok()?;
            let Slot { entry, generation } = slot;
            if let SlotEntry::Free { .. } = entry {
                continue;
            }
            self.num_left -= 1;
            break E::with(index, generation);
        };
        Some(entity)
    }
}

impl<E, const N: usize> ExactSizeIterator for IntoIter<E, N>
where
    E: Entity,
    E::Index: TryFrom<usize>,
{
    fn len(&self) -> usize {
        self.num_left
    }
}

impl<E, const N: usize> FusedIterator for IntoIter<E, N>
where
    E: Entity,
    E::Index: TryFrom<usize>,
{
}

#[cfg(test)]
mod tests {
    use super::ArrayRegistry;

    #[test]
    fn new() {
        let registry = ArrayRegistry::<10>::new();
        assert!(registry.is_empty());
    }

    #[test]
    fn create() {
        let mut registry = ArrayRegistry::<10>::new();
        let entity = registry.create();
        assert!(registry.contains(entity));
    }

    #[test]
    fn destroy() {
        let mut registry = ArrayRegistry::<10>::new();
        let entity = registry.create();

        registry.destroy(entity).unwrap();
        assert!(!registry.contains(entity));
    }

    #[test]
    fn recreate() {
        let mut registry = ArrayRegistry::<10>::new();
        let entity = registry.create();
        registry.destroy(entity).unwrap();

        let new_entity = registry.create();
        assert!(!registry.contains(entity));
        assert!(registry.contains(new_entity));
        assert_eq!(new_entity.index(), entity.index());
        assert_eq!(new_entity.generation(), entity.generation() + 1);
    }

    #[test]
    #[should_panic]
    fn too_many() {
        let mut registry = ArrayRegistry::<2>::new();
        for _ in 0..3 {
            registry.create();
        }
    }

    #[test]
    fn iter() {
        let mut registry = ArrayRegistry::<10>::new();
        let _ = registry.create();
        let _ = registry.create();
        let entity = registry.create();
        let _ = registry.create();
        let _ = registry.create();
        registry.destroy(entity).unwrap();

        let mut iter = registry.iter();
        assert_eq!(iter.len(), 4);

        let entity = iter.find(|entity| entity.index() == 2);
        assert!(entity.is_none());
    }

    #[test]
    fn into_iter() {
        let mut registry = ArrayRegistry::<10>::new();
        let _ = registry.create();
        let _ = registry.create();
        let entity = registry.create();
        let _ = registry.create();
        let _ = registry.create();
        registry.destroy(entity).unwrap();

        let mut iter = registry.into_iter();
        assert_eq!(iter.len(), 4);

        let entity = iter.find(|entity| entity.index() == 2);
        assert!(entity.is_none());
    }
}

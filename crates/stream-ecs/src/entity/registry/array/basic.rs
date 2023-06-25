//! Basic entity registry implementation backed by an array.

use core::{
    iter::{Enumerate, FusedIterator},
    slice,
};

use arrayvec::ArrayVec;

use crate::entity::{
    registry::{NotPresentError, Registry, TryRegistry},
    Entity,
};

use super::ArrayRegistryError;

#[derive(Debug, Clone)]
enum SlotEntry<T> {
    Free { next_free: usize },
    Occupied { value: T },
}

#[derive(Debug, Clone)]
struct Slot<T> {
    entry: SlotEntry<T>,
    generation: u32,
}

/// Default implementation of the entity registry backed by an array.
#[derive(Debug, Clone, Default)]
pub struct ArrayRegistry<const N: usize> {
    slots: ArrayVec<Slot<()>, N>,
    free_head: usize,
    len: usize,
}

impl<const N: usize> ArrayRegistry<N> {
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
    pub fn create(&mut self) -> Entity {
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
    pub fn try_create(&mut self) -> Result<Entity, ArrayRegistryError> {
        let entity = if let Some(slot) = self.slots.get_mut(self.free_head) {
            if let SlotEntry::Free { next_free } = slot.entry {
                let index = self.free_head.try_into().map_err(|_| ArrayRegistryError)?;
                let entity = Entity::new(index, slot.generation);
                self.free_head = next_free;
                slot.entry = SlotEntry::Occupied { value: () };
                entity
            } else {
                unreachable!("free head must not point to the occupied entry")
            }
        } else {
            let index = self.len.try_into().map_err(|_| ArrayRegistryError)?;
            let generation = 0;
            let entity = Entity::new(index, generation);
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
    pub fn contains(&self, entity: Entity) -> bool {
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
    pub fn destroy(&mut self, entity: Entity) -> Result<(), NotPresentError> {
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
        slot.entry = SlotEntry::Free {
            next_free: self.free_head,
        };
        slot.generation += 1;
        self.free_head = index;
        self.len -= 1;
        Ok(value)
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
    pub fn iter(&self) -> Iter<'_> {
        self.into_iter()
    }
}

impl<const N: usize> Registry for ArrayRegistry<N> {
    fn create(&mut self) -> Entity {
        ArrayRegistry::create(self)
    }

    fn contains(&self, entity: Entity) -> bool {
        ArrayRegistry::contains(self, entity)
    }

    fn destroy(&mut self, entity: Entity) -> Result<(), NotPresentError> {
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

    type Iter<'me> = Iter<'me>
    where
        Self: 'me;

    fn iter(&self) -> Self::Iter<'_> {
        ArrayRegistry::iter(self)
    }
}

impl<const N: usize> TryRegistry for ArrayRegistry<N> {
    type Err = ArrayRegistryError;

    fn try_create(&mut self) -> Result<Entity, Self::Err> {
        ArrayRegistry::try_create(self)
    }
}

impl<'me, const N: usize> IntoIterator for &'me ArrayRegistry<N> {
    type Item = Entity;

    type IntoIter = Iter<'me>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.slots.iter().enumerate();
        let num_left = self.len;
        Iter { iter, num_left }
    }
}

impl<const N: usize> IntoIterator for ArrayRegistry<N> {
    type Item = Entity;

    type IntoIter = IntoIter<N>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.slots.into_iter().enumerate();
        let num_left = self.len;
        IntoIter { iter, num_left }
    }
}

/// Iterator over alive entities contained in the array registry.
#[derive(Debug, Clone)]
pub struct Iter<'data> {
    iter: Enumerate<slice::Iter<'data, Slot<()>>>,
    num_left: usize,
}

impl Iterator for Iter<'_> {
    type Item = Entity;

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
            break Entity::new(index, generation);
        };
        Some(entity)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.num_left;
        (len, Some(len))
    }
}

impl DoubleEndedIterator for Iter<'_> {
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
            break Entity::new(index, generation);
        };
        Some(entity)
    }
}

impl ExactSizeIterator for Iter<'_> {
    fn len(&self) -> usize {
        self.num_left
    }
}

impl FusedIterator for Iter<'_> {}

/// Type of iterator in which array registry can be converted.
#[derive(Debug, Clone)]
pub struct IntoIter<const N: usize> {
    iter: Enumerate<arrayvec::IntoIter<Slot<()>, N>>,
    num_left: usize,
}

impl<const N: usize> Iterator for IntoIter<N> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        let entity = loop {
            let (index, slot) = self.iter.next()?;
            let index = index.try_into().ok()?;
            let Slot { entry, generation } = slot;
            if let SlotEntry::Free { .. } = entry {
                continue;
            }
            self.num_left -= 1;
            break Entity::new(index, generation);
        };
        Some(entity)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.num_left;
        (len, Some(len))
    }
}

impl<const N: usize> DoubleEndedIterator for IntoIter<N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let entity = loop {
            let (index, slot) = self.iter.next_back()?;
            let index = index.try_into().ok()?;
            let Slot { entry, generation } = slot;
            if let SlotEntry::Free { .. } = entry {
                continue;
            }
            self.num_left -= 1;
            break Entity::new(index, generation);
        };
        Some(entity)
    }
}

impl<const N: usize> ExactSizeIterator for IntoIter<N> {
    fn len(&self) -> usize {
        self.num_left
    }
}

impl<const N: usize> FusedIterator for IntoIter<N> {}

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

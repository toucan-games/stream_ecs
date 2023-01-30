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
    Free { next_free: u32 },
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
    free_head: u32,
    len: u32,
}

impl<const N: usize> ArrayRegistry<N> {
    /// Creates new empty array entity registry.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::registry::{Registry, array::ArrayRegistry};
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
}

impl<const N: usize> Registry for ArrayRegistry<N> {
    #[track_caller]
    fn create(&mut self) -> Entity {
        match self.try_create() {
            Ok(entity) => entity,
            Err(err) => panic!("{err}"),
        }
    }

    fn contains(&self, entity: Entity) -> bool {
        let Ok(index) = usize::try_from(entity.index) else {
            return false;
        };
        let Some(slot) = self.slots.get(index) else {
            return false;
        };
        let Slot { entry, generation } = slot;
        if let SlotEntry::Free { .. } = entry {
            return false;
        }
        *generation == entity.generation
    }

    fn destroy(&mut self, entity: Entity) -> Result<(), NotPresentError> {
        let Ok(index) = usize::try_from(entity.index) else {
            return Err(NotPresentError::new(entity));
        };
        let Some(slot) = self.slots.get_mut(index) else {
            return Err(NotPresentError::new(entity));
        };
        let SlotEntry::Occupied { value } = slot.entry else {
            return Err(NotPresentError::new(entity));
        };
        if slot.generation != entity.generation {
            return Err(NotPresentError::new(entity));
        }
        slot.entry = SlotEntry::Free {
            next_free: self.free_head,
        };
        slot.generation += 1;
        self.free_head = entity.index;
        self.len -= 1;
        Ok(value)
    }

    fn len(&self) -> usize {
        self.len as usize
    }

    fn clear(&mut self) {
        self.slots.clear();
        self.free_head = 0;
        self.len = 0;
    }

    type Iter<'a> = <&'a Self as IntoIterator>::IntoIter
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.into_iter()
    }
}

impl<const N: usize> TryRegistry for ArrayRegistry<N> {
    type Err = ArrayRegistryError;

    fn try_create(&mut self) -> Result<Entity, Self::Err> {
        let new_len = self.len + 1;
        if usize::try_from(new_len).is_err() || new_len == u32::MAX {
            return Err(ArrayRegistryError);
        }

        let entity = if let Some(slot) = self.slots.get_mut(self.free_head as usize) {
            if let SlotEntry::Free { next_free } = slot.entry {
                let entity = Entity::new(self.free_head, slot.generation);
                self.free_head = next_free;
                slot.entry = SlotEntry::Occupied { value: () };
                entity
            } else {
                unreachable!("Free head must not point to the occupied entry")
            }
        } else {
            let generation = 0;
            let entity = Entity::new(self.len, generation);
            let slot = Slot {
                entry: SlotEntry::Occupied { value: () },
                generation,
            };
            if self.slots.try_push(slot).is_err() {
                return Err(ArrayRegistryError);
            }
            self.free_head = entity.index + 1;
            entity
        };
        self.len = new_len;
        Ok(entity)
    }
}

impl<'a, const N: usize> IntoIterator for &'a ArrayRegistry<N> {
    type Item = Entity;

    type IntoIter = Iter<'a>;

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
pub struct Iter<'a> {
    iter: Enumerate<slice::Iter<'a, Slot<()>>>,
    num_left: u32,
}

impl Iterator for Iter<'_> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        let entity = loop {
            let (index, slot) = self.iter.next()?;
            let Slot { entry, generation } = slot;
            if let SlotEntry::Free { .. } = entry {
                continue;
            }
            self.num_left -= 1;
            break Entity::new(index as u32, *generation);
        };
        Some(entity)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.num_left as usize;
        (len, Some(len))
    }
}

impl DoubleEndedIterator for Iter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let entity = loop {
            let (index, slot) = self.iter.next_back()?;
            let Slot { entry, generation } = slot;
            if let SlotEntry::Free { .. } = entry {
                continue;
            }
            self.num_left -= 1;
            break Entity::new(index as u32, *generation);
        };
        Some(entity)
    }
}

impl ExactSizeIterator for Iter<'_> {
    fn len(&self) -> usize {
        self.num_left as usize
    }
}

impl FusedIterator for Iter<'_> {}

/// Type of iterator in which array registry can be converted.
pub struct IntoIter<const N: usize> {
    iter: Enumerate<arrayvec::IntoIter<Slot<()>, N>>,
    num_left: u32,
}

impl<const N: usize> Iterator for IntoIter<N> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        let entity = loop {
            let (index, slot) = self.iter.next()?;
            let Slot { entry, generation } = slot;
            if let SlotEntry::Free { .. } = entry {
                continue;
            }
            self.num_left -= 1;
            break Entity::new(index as u32, generation);
        };
        Some(entity)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.num_left as usize;
        (len, Some(len))
    }
}

impl<const N: usize> DoubleEndedIterator for IntoIter<N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let entity = loop {
            let (index, slot) = self.iter.next_back()?;
            let Slot { entry, generation } = slot;
            if let SlotEntry::Free { .. } = entry {
                continue;
            }
            self.num_left -= 1;
            break Entity::new(index as u32, generation);
        };
        Some(entity)
    }
}

impl<const N: usize> ExactSizeIterator for IntoIter<N> {
    fn len(&self) -> usize {
        self.num_left as usize
    }
}

impl<const N: usize> FusedIterator for IntoIter<N> {}

#[cfg(test)]
mod tests {
    use super::*;

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

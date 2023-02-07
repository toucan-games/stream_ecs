//! Dense entity registry implementation backed by an array.

use core::{
    iter::{Copied, FusedIterator},
    slice,
};

use arrayvec::ArrayVec;

use crate::entity::{
    registry::{NotPresentError, Registry, TryRegistry},
    Entity,
};

use super::ArrayRegistryError;

#[derive(Debug, Clone)]
enum SlotEntry {
    Occupied { dense_index: u32 },
    Free { next_free: u32 },
}

#[derive(Debug, Clone)]
struct Slot {
    entry: SlotEntry,
    generation: u32,
}

/// Implementation of the entity registry backed by an array
/// which stores entities in a dense array.
#[derive(Debug, Clone, Default)]
pub struct DenseArrayRegistry<const N: usize> {
    dense: ArrayVec<Entity, N>,
    sparse: ArrayVec<Slot, N>,
    free_head: u32,
}

impl<const N: usize> DenseArrayRegistry<N> {
    /// Creates new empty dense array registry.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::registry::{Registry, array::DenseArrayRegistry};
    ///
    /// let registry = DenseArrayRegistry::<10>::new();
    /// assert!(registry.is_empty());
    /// ```
    ///
    /// It also can be used to create globally accessible entity registry of fixed size:
    ///
    /// ```
    /// # use stream_ecs::entity::registry::array::DenseArrayRegistry;
    /// const REGISTRY: DenseArrayRegistry<1024> = DenseArrayRegistry::new();
    pub const fn new() -> Self {
        Self {
            dense: ArrayVec::new_const(),
            sparse: ArrayVec::new_const(),
            free_head: 0,
        }
    }

    /// Returns the capacity of the dense array registry.
    ///
    /// # Examples
    ///
    /// ```
    /// use stream_ecs::entity::registry::array::DenseArrayRegistry;
    ///
    /// let registry = DenseArrayRegistry::<1024>::new();
    /// assert_eq!(registry.capacity(), 1024);
    /// ```
    pub const fn capacity(&self) -> usize {
        self.dense.capacity()
    }
}

impl<const N: usize> Registry for DenseArrayRegistry<N> {
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
        let Some(slot) = self.sparse.get(index) else {
            return false;
        };
        let SlotEntry::Occupied { dense_index } = slot.entry else {
            return false;
        };
        let Some(_) = self.dense.get(dense_index as usize) else {
            return false;
        };
        slot.generation == entity.generation
    }

    fn destroy(&mut self, entity: Entity) -> Result<(), NotPresentError> {
        let Ok(index) = usize::try_from(entity.index) else {
            return Err(NotPresentError::new(entity));
        };
        let Some(slot) = self.sparse.get_mut(index) else {
            return Err(NotPresentError::new(entity));
        };
        let SlotEntry::Occupied { dense_index } = slot.entry else {
            return Err(NotPresentError::new(entity));
        };
        let Some(_) = self.dense.get(dense_index as usize) else {
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
        self.dense.swap_remove(dense_index as usize);
        if let Some(entity) = self.dense.get(dense_index as usize) {
            let slot = self
                .sparse
                .get_mut(entity.index as usize)
                .expect("index should point to the valid slot");
            slot.entry = match slot.entry {
                SlotEntry::Occupied { .. } => SlotEntry::Occupied { dense_index },
                SlotEntry::Free { .. } => SlotEntry::Free {
                    next_free: dense_index,
                },
            };
        }
        Ok(())
    }

    fn len(&self) -> usize {
        self.dense.len()
    }

    fn clear(&mut self) {
        self.dense.clear();
        self.sparse.clear();
        self.free_head = 0;
    }

    type Iter<'a> = <&'a Self as IntoIterator>::IntoIter
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.into_iter()
    }
}

impl<const N: usize> TryRegistry for DenseArrayRegistry<N> {
    type Err = ArrayRegistryError;

    fn try_create(&mut self) -> Result<Entity, Self::Err> {
        if self.len() == self.capacity() {
            return Err(ArrayRegistryError);
        }

        let entity = if let Some(slot) = self.sparse.get_mut(self.free_head as usize) {
            if let SlotEntry::Free { next_free } = slot.entry {
                let entity = Entity::new(self.free_head, slot.generation);
                if self.dense.try_push(entity).is_err() {
                    return Err(ArrayRegistryError);
                }
                self.free_head = next_free;
                slot.entry = SlotEntry::Occupied {
                    dense_index: self.dense.len() as u32 - 1,
                };
                entity
            } else {
                unreachable!("Free head must not point to the occupied entry")
            }
        } else {
            let generation = 0;
            let entity = Entity::new(self.free_head, generation);
            let slot = Slot {
                entry: SlotEntry::Occupied {
                    dense_index: self.dense.len() as u32,
                },
                generation,
            };
            if self.dense.try_push(entity).is_err() {
                return Err(ArrayRegistryError);
            }
            if self.sparse.try_push(slot).is_err() {
                return Err(ArrayRegistryError);
            }
            self.free_head = self.sparse.len() as u32;
            entity
        };
        Ok(entity)
    }
}

impl<'a, const N: usize> IntoIterator for &'a DenseArrayRegistry<N> {
    type Item = Entity;

    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.dense.iter().copied();
        Iter { iter }
    }
}

impl<const N: usize> IntoIterator for DenseArrayRegistry<N> {
    type Item = Entity;

    type IntoIter = IntoIter<N>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.dense.into_iter();
        IntoIter { iter }
    }
}

/// Iterator over alive entities contained in the dense array registry.
#[derive(Debug, Clone)]
pub struct Iter<'a> {
    iter: Copied<slice::Iter<'a, Entity>>,
}

impl Iterator for Iter<'_> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl DoubleEndedIterator for Iter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl ExactSizeIterator for Iter<'_> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl FusedIterator for Iter<'_> {}

/// Type of iterator in which dense array registry can be converted.
#[derive(Debug, Clone)]
pub struct IntoIter<const N: usize> {
    iter: arrayvec::IntoIter<Entity, N>,
}

impl<const N: usize> Iterator for IntoIter<N> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<const N: usize> DoubleEndedIterator for IntoIter<N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<const N: usize> ExactSizeIterator for IntoIter<N> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<const N: usize> FusedIterator for IntoIter<N> {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new() {
        let registry = DenseArrayRegistry::<10>::new();
        assert!(registry.is_empty());
    }

    #[test]
    fn create() {
        let mut registry = DenseArrayRegistry::<10>::new();
        let entity = registry.create();
        assert!(registry.contains(entity));
    }

    #[test]
    fn destroy() {
        let mut registry = DenseArrayRegistry::<10>::new();
        let entity = registry.create();

        registry.destroy(entity).unwrap();
        assert!(!registry.contains(entity));
    }

    #[test]
    fn recreate() {
        let mut registry = DenseArrayRegistry::<10>::new();
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
        let mut registry = DenseArrayRegistry::<2>::new();
        for _ in 0..3 {
            registry.create();
        }
    }

    #[test]
    fn iter() {
        let mut registry = DenseArrayRegistry::<10>::new();
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
        let mut registry = DenseArrayRegistry::<10>::new();
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

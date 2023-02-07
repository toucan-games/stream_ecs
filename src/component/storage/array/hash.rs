//! Hash dense component storage implementation backed by an array.

use core::{
    hash::{BuildHasher, Hash, Hasher},
    iter::{self, FusedIterator},
    mem, slice,
};

use arrayvec::ArrayVec;

use crate::{
    component::{
        storage::{Storage, TryStorage},
        Component,
    },
    entity::Entity,
};

use super::ArrayStorageError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HashValue(u64);

impl HashValue {
    fn new<K, S>(key: &K, build_hasher: &S) -> Self
    where
        K: Hash + ?Sized,
        S: BuildHasher,
    {
        let hash = {
            let mut hasher = build_hasher.build_hasher();
            key.hash(&mut hasher);
            hasher.finish()
        };
        Self(hash)
    }

    fn desired_index(self, len: u64) -> u64 {
        let Self(hash) = self;
        hash % len
    }

    fn probe_distance(self, len: u64, current: u64) -> u64 {
        let desired = self.desired_index(len);
        current.wrapping_sub(desired) % len
    }
}

#[derive(Debug, Clone)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}

#[derive(Debug, Clone, Copy)]
enum HashIndex {
    Free,
    Occupied { hash: HashValue, index: u32 },
}

/// Hash implementation of the component storage backed by an array.
#[derive(Debug, Clone)]
pub struct HashArrayStorage<T, S, const N: usize>
where
    T: Component,
{
    buckets: ArrayVec<Bucket<Entity, T>, N>,
    indices: [HashIndex; N],
    build_hasher: S,
}

impl<T, S, const N: usize> HashArrayStorage<T, S, N>
where
    T: Component,
    S: Default,
{
    /// Creates new empty hash array component storage.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn new() -> Self {
        let build_hasher = S::default();
        Self::with_hasher(build_hasher)
    }
}

impl<T, S, const N: usize> HashArrayStorage<T, S, N>
where
    T: Component,
{
    const EMPTY_INDEX: HashIndex = HashIndex::Free;
    const EMPTY_ARRAY: [HashIndex; N] = [Self::EMPTY_INDEX; N];

    /// Creates new empty hash array component storage with provided hasher.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub const fn with_hasher(build_hasher: S) -> Self {
        Self {
            buckets: ArrayVec::new_const(),
            indices: Self::EMPTY_ARRAY,
            build_hasher,
        }
    }

    /// Returns the capacity of the hash array component storage.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub const fn capacity(&self) -> usize {
        self.buckets.capacity()
    }
}

impl<T, S, const N: usize> Default for HashArrayStorage<T, S, N>
where
    T: Component,
    S: Default,
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
struct FindBucket {
    hash_index: u32,
    bucket_index: u32,
}

impl<T, S, const N: usize> HashArrayStorage<T, S, N>
where
    T: Component,
    S: BuildHasher,
{
    fn find_bucket(&self, entity: Entity) -> Option<FindBucket> {
        if self.buckets.is_empty() {
            return None;
        }
        let entity_hash = HashValue::new(&entity.index(), &self.build_hasher);
        let probe_len = self.capacity() as u64;
        let desired_index = entity_hash.desired_index(probe_len) as usize;

        let mut skip = desired_index;
        let mut distances = 0..;
        let item = 'outer: loop {
            let zipped = iter::zip(
                distances.by_ref(),
                self.indices.iter().enumerate().skip(skip),
            );
            for (distance, (current, hash_index)) in zipped {
                let &HashIndex::Occupied { hash, index } = hash_index else {
                    continue;
                };
                let probe_distance = hash.probe_distance(probe_len, current as u64);
                if distance > probe_distance {
                    break 'outer None;
                }
                if hash != entity_hash {
                    continue;
                }
                let &Bucket { key, .. } = self
                    .buckets
                    .get(index as usize)
                    .expect("index should point to the valid bucket");
                if entity != key {
                    continue;
                }
                let find_bucket = FindBucket {
                    hash_index: current as u32,
                    bucket_index: index,
                };
                break 'outer Some(find_bucket);
            }
            skip = 0;
        };
        item
    }
}

impl<T, S, const N: usize> Storage for HashArrayStorage<T, S, N>
where
    T: Component,
    S: BuildHasher + Send + Sync + 'static,
{
    type Item = T;

    #[track_caller]
    fn attach(&mut self, entity: Entity, component: Self::Item) -> Option<Self::Item> {
        match self.try_attach(entity, component) {
            Ok(component) => component,
            Err(err) => panic!("{err}"),
        }
    }

    fn is_attached(&self, entity: Entity) -> bool {
        self.find_bucket(entity).is_some()
    }

    fn get(&self, entity: Entity) -> Option<&Self::Item> {
        let FindBucket { bucket_index, .. } = self.find_bucket(entity)?;
        let Bucket { value, .. } = self
            .buckets
            .get(bucket_index as usize)
            .expect("index should point to the valid bucket");
        Some(value)
    }

    fn get_mut(&mut self, entity: Entity) -> Option<&mut Self::Item> {
        let FindBucket { bucket_index, .. } = self.find_bucket(entity)?;
        let Bucket { value, .. } = self
            .buckets
            .get_mut(bucket_index as usize)
            .expect("index should point to the valid bucket");
        Some(value)
    }

    fn remove(&mut self, entity: Entity) -> Option<Self::Item> {
        let FindBucket {
            hash_index,
            bucket_index,
        } = self.find_bucket(entity)?;

        {
            let hash_index = self
                .indices
                .get_mut(hash_index as usize)
                .expect("index should point to the valid hash index");
            *hash_index = Self::EMPTY_INDEX;
        }

        let bucket = self
            .buckets
            .swap_pop(bucket_index as usize)
            .expect("index should point to the valid bucket");
        if let Some(bucket) = self.buckets.get(bucket_index as usize) {
            let &Bucket { hash, .. } = bucket;
            let probe_len = self.capacity() as u64;
            let desired_index = hash.desired_index(probe_len) as usize;
            let mut skip = desired_index;
            'outer: loop {
                for hash_index in self.indices.iter_mut().skip(skip) {
                    let &mut HashIndex::Occupied { index, .. } = hash_index else {
                        continue;
                    };
                    if (index as usize) < self.buckets.len() {
                        continue;
                    }
                    *hash_index = HashIndex::Occupied {
                        hash,
                        index: bucket_index,
                    };
                    break 'outer;
                }
                skip = 0;
            }
        }

        let mut last_current = hash_index as usize;
        let probe_len = self.capacity() as u64;
        let mut skip = last_current + 1;
        'outer: loop {
            for current in skip..probe_len as usize {
                let HashIndex::Occupied { hash, .. } = self.indices[current] else {
                    break 'outer;
                };
                let probe_distance = hash.probe_distance(probe_len, current as u64);
                if probe_distance == 0 {
                    break 'outer;
                }
                self.indices[last_current] = self.indices[current];
                self.indices[current] = HashIndex::Free;
                last_current = current;
            }
            skip = 0;
        }

        Some(bucket.value)
    }

    fn clear(&mut self) {
        self.buckets.clear();
        self.indices = Self::EMPTY_ARRAY;
    }

    fn len(&self) -> usize {
        self.buckets.len()
    }

    type Iter<'a> = <&'a Self as IntoIterator>::IntoIter
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.into_iter()
    }

    type IterMut<'a> = <&'a mut Self as IntoIterator>::IntoIter
    where
        Self: 'a;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.into_iter()
    }
}

impl<T, S, const N: usize> TryStorage for HashArrayStorage<T, S, N>
where
    T: Component,
    S: BuildHasher + Send + Sync + 'static,
{
    type Err = ArrayStorageError;

    fn try_attach(
        &mut self,
        entity: Entity,
        component: Self::Item,
    ) -> Result<Option<Self::Item>, Self::Err> {
        enum AttachOperation<'a> {
            Replace { hash_index: &'a mut HashIndex },
            TakeFromRich { start_index: usize },
        }

        let entity_hash = HashValue::new(&entity.index(), &self.build_hasher);
        let probe_len = self.capacity() as u64;
        let desired_index = entity_hash.desired_index(probe_len) as usize;

        let mut skip = desired_index;
        let mut distances = 0..;
        let operation = 'outer: loop {
            let zipped = iter::zip(
                distances.by_ref(),
                self.indices.iter_mut().enumerate().skip(skip),
            );
            for (distance, (current, hash_index)) in zipped {
                let &mut HashIndex::Occupied { hash, index } = hash_index else {
                    break 'outer AttachOperation::Replace { hash_index };
                };
                let probe_distance = hash.probe_distance(probe_len, current as u64);
                if distance > probe_distance {
                    break 'outer AttachOperation::TakeFromRich {
                        start_index: current,
                    };
                }
                if hash != entity_hash {
                    continue;
                }
                let &Bucket { key, .. } = self
                    .buckets
                    .get(index as usize)
                    .expect("index should point to the valid bucket");
                if entity.index() != key.index() || entity.generation() < key.generation() {
                    continue;
                }
                break 'outer AttachOperation::Replace { hash_index };
            }
            skip = 0;
        };
        let start_index = match operation {
            AttachOperation::Replace { hash_index } => match hash_index {
                HashIndex::Free => {
                    let bucket = Bucket {
                        hash: entity_hash,
                        key: entity,
                        value: component,
                    };
                    if self.buckets.try_push(bucket).is_err() {
                        return Err(ArrayStorageError);
                    }
                    *hash_index = HashIndex::Occupied {
                        hash: entity_hash,
                        index: self.buckets.len() as u32 - 1,
                    };
                    return Ok(None);
                }
                &mut HashIndex::Occupied { index, .. } => {
                    let Bucket { value, .. } = self
                        .buckets
                        .get_mut(index as usize)
                        .expect("index should point to the valid bucket");
                    let component = mem::replace(value, component);
                    return Ok(Some(component));
                }
            },
            AttachOperation::TakeFromRich { start_index } => start_index,
        };

        let mut hash_index = HashIndex::Occupied {
            hash: entity_hash,
            index: self.buckets.len() as u32,
        };
        skip = start_index;
        loop {
            for next_hash_index in self.indices.iter_mut().skip(skip) {
                if let &mut HashIndex::Free = next_hash_index {
                    hash_index = mem::replace(next_hash_index, hash_index);
                    continue;
                }
                let bucket = Bucket {
                    hash: entity_hash,
                    key: entity,
                    value: component,
                };
                if self.buckets.try_push(bucket).is_err() {
                    return Err(ArrayStorageError);
                }
                *next_hash_index = hash_index;
                return Ok(None);
            }
            skip = 0;
        }
    }
}

impl<'a, T, S, const N: usize> IntoIterator for &'a HashArrayStorage<T, S, N>
where
    T: Component,
{
    type Item = (Entity, &'a T);

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.buckets.iter();
        Iter { iter }
    }
}

impl<'a, T, S, const N: usize> IntoIterator for &'a mut HashArrayStorage<T, S, N>
where
    T: Component,
{
    type Item = (Entity, &'a mut T);

    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.buckets.iter_mut();
        IterMut { iter }
    }
}

impl<T, S, const N: usize> IntoIterator for HashArrayStorage<T, S, N>
where
    T: Component,
{
    type Item = (Entity, T);

    type IntoIter = IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.buckets.into_iter();
        IntoIter { iter }
    }
}

/// Iterator of entities with references of components attached to them
/// in the hash array storage.
#[derive(Debug, Clone)]
pub struct Iter<'a, T>
where
    T: Component,
{
    iter: slice::Iter<'a, Bucket<Entity, T>>,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Component,
{
    type Item = (Entity, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let &Bucket { key, ref value, .. } = self.iter.next()?;
        Some((key, value))
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
        let &Bucket { key, ref value, .. } = self.iter.next_back()?;
        Some((key, value))
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
/// in the hash array storage.
#[derive(Debug)]
pub struct IterMut<'a, T>
where
    T: Component,
{
    iter: slice::IterMut<'a, Bucket<Entity, T>>,
}

impl<'a, T> Iterator for IterMut<'a, T>
where
    T: Component,
{
    type Item = (Entity, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        let &mut Bucket {
            key, ref mut value, ..
        } = self.iter.next()?;
        Some((key, value))
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
        let &mut Bucket {
            key, ref mut value, ..
        } = self.iter.next_back()?;
        Some((key, value))
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

/// Iterator of entities with components attached to them in the hash array storage.
#[derive(Debug, Clone)]
pub struct IntoIter<T, const N: usize>
where
    T: Component,
{
    iter: arrayvec::IntoIter<Bucket<Entity, T>, N>,
}

impl<T, const N: usize> Iterator for IntoIter<T, N>
where
    T: Component,
{
    type Item = (Entity, T);

    fn next(&mut self) -> Option<Self::Item> {
        let Bucket { key, value, .. } = self.iter.next()?;
        Some((key, value))
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
        let Bucket { key, value, .. } = self.iter.next_back()?;
        Some((key, value))
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
    use core::hash::BuildHasherDefault;
    use std::collections::hash_map::DefaultHasher;

    use super::*;

    #[derive(Debug, Clone, Copy)]
    struct Marker;

    type HashArrayStorage<T, const N: usize> =
        super::HashArrayStorage<T, BuildHasherDefault<DefaultHasher>, N>;

    impl Component for Marker {
        type Storage = HashArrayStorage<Self, 0>;
    }

    #[test]
    fn new() {
        let storage = HashArrayStorage::<Marker, 10>::new();
        assert!(storage.is_empty());
    }

    #[test]
    fn attach() {
        let mut storage = HashArrayStorage::<Marker, 10>::new();
        let entity = Entity::new(0, 0);

        let marker = storage.attach(entity, Marker);
        assert!(marker.is_none());
        assert!(storage.is_attached(entity));
    }

    #[test]
    fn attach_many() {
        let mut storage = HashArrayStorage::<Marker, 10>::new();
        for index in 0..storage.capacity() as u32 {
            let entity = Entity::new(index, 0);
            storage.attach(entity, Marker);
            assert!(storage.is_attached(entity));
        }
    }

    #[test]
    fn remove() {
        let mut storage = HashArrayStorage::<Marker, 10>::new();
        let entity = Entity::new(1, 0);

        storage.attach(entity, Marker);
        let marker = storage.remove(entity);
        assert!(marker.is_some());
        assert!(!storage.is_attached(entity));
    }

    #[test]
    fn reattach() {
        let mut storage = HashArrayStorage::<Marker, 10>::new();
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
        let mut storage = HashArrayStorage::<Marker, 2>::new();

        let entity = Entity::new(0, 0);
        storage.attach(entity, Marker);

        let entity = Entity::new(1, 0);
        storage.attach(entity, Marker);

        let entity = Entity::new(2, 0);
        storage.attach(entity, Marker);
    }

    #[test]
    fn iter() {
        let mut storage = HashArrayStorage::<Marker, 10>::new();
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
        let mut storage = HashArrayStorage::<Marker, 10>::new();
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

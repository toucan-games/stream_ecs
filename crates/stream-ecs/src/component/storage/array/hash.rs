//! Hash component storage implementation backed by an array.

use core::{
    hash::{BuildHasher, Hash},
    iter::{self, FusedIterator},
    mem, slice,
};

use arrayvec::ArrayVec;

use crate::{
    component::{
        Component,
        storage::{Storage, TryStorage},
    },
    entity::{DefaultEntity, Entity},
};

use super::ArrayStorageError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HashValue(u64);

impl HashValue {
    fn new<K, S>(build_hasher: &S, key: K) -> Self
    where
        K: Hash,
        S: BuildHasher,
    {
        let hash = build_hasher.hash_one(key);
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
    Occupied { hash: HashValue, index: usize },
}

/// Hash implementation of the component storage backed by an array.
///
/// Main feature of this implementation is hashing of input entities,
/// which allows to store entities with indices which can be *way* bigger
/// than actual capacity of this hash array storage.
///
/// As the [dense implementation], all the data is stored inline, one component after another.
/// Iteration is as fast, so the cost is the same.
///
/// As the [default implementation] of array storage,
/// it can store exactly `N` components of specified type `T`.
///
/// [dense implementation]: super::dense::DenseArrayStorage
/// [default implementation]: super::basic::ArrayStorage
///
/// Consider we have component which represents position of an object:
///
/// ```
/// use std::collections::hash_map::RandomState;
///
/// use stream_ecs::component::{storage::array::HashArrayStorage, Component};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Component)]
/// #[component(storage = HashArrayStorage<Self, RandomState, 10>)]
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
/// # use std::collections::hash_map::RandomState;
/// # use stream_ecs::component::{storage::array::HashArrayStorage, Component};
/// use stream_ecs::entity::DefaultEntity;
/// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
/// # #[component(storage = HashArrayStorage<Self, RandomState, 10>)]
/// # #[component(crate = stream_ecs)]
/// # struct Position {
/// #     x: f32,
/// #     y: f32,
/// # }
///
/// let mut storage = HashArrayStorage::new();
/// let entity = DefaultEntity::new(5, 0);
///
/// storage.attach(entity, Position { x: 0.0, y: 0.0 });
/// assert!(storage.is_attached(entity));
/// ```
#[derive(Debug, Clone)]
pub struct HashArrayStorage<T, S, const N: usize, E = DefaultEntity>
where
    T: Component<Storage = Self>,
    E: Entity,
{
    buckets: ArrayVec<Bucket<E, T>, N>,
    indices: [HashIndex; N],
    build_hasher: S,
}

impl<T, E, S, const N: usize> HashArrayStorage<T, S, N, E>
where
    T: Component<Storage = Self>,
    E: Entity,
    S: Default,
{
    /// Creates new empty hash array component storage.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::collections::hash_map::RandomState;
    /// use stream_ecs::component::storage::array::HashArrayStorage;
    /// # use stream_ecs::component::Component;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = HashArrayStorage<Self, RandomState, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = HashArrayStorage::<Position, _, 10>::new();
    /// assert!(storage.is_empty());
    /// ```
    pub fn new() -> Self {
        let build_hasher = S::default();
        Self::with_hasher(build_hasher)
    }
}

impl<T, E, S, const N: usize> HashArrayStorage<T, S, N, E>
where
    T: Component<Storage = Self>,
    E: Entity,
{
    const EMPTY_INDEX: HashIndex = HashIndex::Free;
    const EMPTY_ARRAY: [HashIndex; N] = [Self::EMPTY_INDEX; N];

    /// Creates new empty hash array component storage with provided hasher.
    ///
    /// # Examples
    ///
    /// Suppose we have component for velocity with custom hasher builder:
    ///
    /// ```
    /// use core::hash::BuildHasher;
    /// use std::collections::hash_map::DefaultHasher;
    ///
    /// use stream_ecs::component::{storage::array::HashArrayStorage, Component};
    ///
    /// struct MyBuildHasher;
    ///
    /// impl BuildHasher for MyBuildHasher {
    ///     type Hasher = DefaultHasher;
    ///     fn build_hasher(&self) -> Self::Hasher { DefaultHasher::new() }
    /// }
    ///
    /// #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// #[component(storage = HashArrayStorage<Self, MyBuildHasher, 42>)]
    /// # #[component(crate = stream_ecs)]
    /// struct Velocity {
    ///     dx: f32,
    ///     dy: f32,
    /// }
    /// ```
    ///
    /// Then we can create hash component storage with provided hasher builder:
    ///
    /// ```
    /// # use core::hash::BuildHasher;
    /// # use std::collections::hash_map::DefaultHasher;
    /// # use stream_ecs::component::{storage::array::HashArrayStorage, Component};
    /// # struct MyBuildHasher;
    /// # impl BuildHasher for MyBuildHasher {
    /// #     type Hasher = DefaultHasher;
    /// #     fn build_hasher(&self) -> Self::Hasher { DefaultHasher::new() }
    /// # }
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = HashArrayStorage<Self, MyBuildHasher, 42>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Velocity {
    /// #     dx: f32,
    /// #     dy: f32,
    /// # }
    /// let storage = HashArrayStorage::<Velocity, _, 42>::with_hasher(MyBuildHasher);
    /// assert!(storage.is_empty());
    /// ```
    ///
    /// It also can be used to create globally accessible hash array component storage of fixed size:
    ///
    /// ```
    /// # use core::hash::BuildHasher;
    /// # use std::collections::hash_map::DefaultHasher;
    /// # use stream_ecs::component::{storage::array::HashArrayStorage, Component};
    /// # struct MyBuildHasher;
    /// # impl BuildHasher for MyBuildHasher {
    /// #     type Hasher = DefaultHasher;
    /// #     fn build_hasher(&self) -> Self::Hasher { DefaultHasher::new() }
    /// # }
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = HashArrayStorage<Self, MyBuildHasher, 42>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Velocity {
    /// #     dx: f32,
    /// #     dy: f32,
    /// # }
    /// const STORAGE: HashArrayStorage<Velocity, MyBuildHasher, 42> =
    ///     HashArrayStorage::with_hasher(MyBuildHasher);
    /// ```
    pub const fn with_hasher(build_hasher: S) -> Self {
        Self {
            buckets: ArrayVec::new_const(),
            indices: Self::EMPTY_ARRAY,
            build_hasher,
        }
    }

    /// Returns count of components which are stored in the hash array storage.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::collections::hash_map::RandomState;
    /// # use stream_ecs::component::Component;
    /// use stream_ecs::{component::storage::array::HashArrayStorage, entity::DefaultEntity};
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = HashArrayStorage<Self, RandomState, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = HashArrayStorage::new();
    ///
    /// storage.attach(DefaultEntity::new(50, 1), Position { x: 0.0, y: 0.0 });
    /// storage.attach(DefaultEntity::new(97, 63), Position { x: 10.0, y: -10.0 });
    /// assert_eq!(storage.len(), 2);
    /// ```
    pub const fn len(&self) -> usize {
        self.buckets.len()
    }

    /// Returns the capacity of the hash array component storage.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::collections::hash_map::RandomState;
    /// # use stream_ecs::component::Component;
    /// use stream_ecs::component::storage::array::HashArrayStorage;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = HashArrayStorage<Self, RandomState, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let storage = HashArrayStorage::<Position, _, 10>::new();
    /// assert_eq!(storage.capacity(), 10);
    /// ```
    pub const fn capacity(&self) -> usize {
        self.buckets.capacity()
    }

    /// Checks if the hash array storage is empty, or has no components.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::collections::hash_map::RandomState;
    /// # use stream_ecs::component::Component;
    /// use stream_ecs::{component::storage::array::HashArrayStorage, entity::DefaultEntity};
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = HashArrayStorage<Self, RandomState, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = HashArrayStorage::new();
    /// assert!(storage.is_empty());
    ///
    /// storage.attach(DefaultEntity::new(0, 0), Position { x: 0.0, y: 0.0 });
    /// assert!(!storage.is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears this hash array storage, destroying all components in it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::collections::hash_map::RandomState;
    /// # use stream_ecs::component::Component;
    /// use stream_ecs::{component::storage::array::HashArrayStorage, entity::DefaultEntity};
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = HashArrayStorage<Self, RandomState, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = HashArrayStorage::new();
    ///
    /// storage.attach(DefaultEntity::new(15, 1), Position { x: 0.0, y: 0.0 });
    /// storage.attach(DefaultEntity::new(8, 6), Position { x: 10.0, y: -10.0 });
    /// assert!(!storage.is_empty());
    ///
    /// storage.clear();
    /// assert!(storage.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.buckets.clear();
        self.indices = Self::EMPTY_ARRAY;
    }

    /// Returns an iterator over entity keys with references of components attached to them.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::collections::hash_map::RandomState;
    /// # use stream_ecs::component::Component;
    /// use stream_ecs::{component::storage::array::HashArrayStorage, entity::DefaultEntity};
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = HashArrayStorage<Self, RandomState, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = HashArrayStorage::new();
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
    pub fn iter(&self) -> Iter<'_, T, S, N, E> {
        self.into_iter()
    }

    /// Returns an iterator over entity keys with mutable references of components attached to them.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::collections::hash_map::RandomState;
    /// # use stream_ecs::component::Component;
    /// use stream_ecs::{component::storage::array::HashArrayStorage, entity::DefaultEntity};
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = HashArrayStorage<Self, RandomState, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = HashArrayStorage::new();
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
    pub fn iter_mut(&mut self) -> IterMut<'_, T, S, N, E> {
        self.into_iter()
    }
}

impl<T, E, S, const N: usize> Default for HashArrayStorage<T, S, N, E>
where
    T: Component<Storage = Self>,
    E: Entity,
    S: Default,
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
struct FindBucket {
    hash_index: usize,
    bucket_index: usize,
}

impl<T, E, S, const N: usize> HashArrayStorage<T, S, N, E>
where
    T: Component<Storage = Self>,
    E: Entity + PartialEq,
    E::Index: Hash + PartialEq,
    E::Generation: PartialOrd,
    S: BuildHasher,
{
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
    /// # use core::hash::BuildHasherDefault;
    /// # use std::collections::hash_map::DefaultHasher;
    /// # use stream_ecs::component::Component;
    /// use stream_ecs::{component::storage::array::HashArrayStorage, entity::DefaultEntity};
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = HashArrayStorage<Self, BuildHasherDefault<DefaultHasher>, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = HashArrayStorage::new();
    ///
    /// let entity = DefaultEntity::new(15, 0);
    /// let component = storage.attach(entity, Position { x: 10.0, y: 12.0 });
    /// assert_eq!(component, None);
    ///
    /// let entity = DefaultEntity::new(15, 1);
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
    /// This function will return an error if the count of components attached to some entities
    /// is the same as the capacity of the storage.
    ///
    /// # Examples
    ///
    /// ```
    /// # use core::hash::BuildHasherDefault;
    /// # use std::collections::hash_map::DefaultHasher;
    /// # use stream_ecs::component::Component;
    /// use stream_ecs::{component::storage::array::HashArrayStorage, entity::DefaultEntity};
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = HashArrayStorage<Self, BuildHasherDefault<DefaultHasher>, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = HashArrayStorage::new();
    /// for i in 0..10 {
    ///     let entity = DefaultEntity::new(i + 10, 0);
    ///     storage.attach(entity, Position { x: 10.0, y: 10.0 });
    /// }
    ///
    /// let entity = DefaultEntity::new(36, 0);
    /// let result = storage.try_attach(entity, Position { x: 0.0, y: 0.0 });
    /// assert!(result.is_err());
    /// ```
    ///
    /// This is the fallible version of [`attach`][Self::attach()] method.
    pub fn try_attach(&mut self, entity: E, component: T) -> Result<Option<T>, ArrayStorageError> {
        enum AttachOperation<'a> {
            Replace { hash_index: &'a mut HashIndex },
            TakeFromRich { start_index: usize },
        }

        let probe_len = self.capacity().try_into().map_err(|_| ArrayStorageError)?;
        let Self {
            buckets,
            indices,
            build_hasher,
        } = self;
        let entity_hash = HashValue::new(build_hasher, entity.index());
        let desired_index = entity_hash
            .desired_index(probe_len)
            .try_into()
            .map_err(|_| ArrayStorageError)?;

        let mut skip = desired_index;
        let mut distances = 0..;
        let operation = 'outer: loop {
            let zipped = iter::zip(
                distances.by_ref(),
                indices.iter_mut().enumerate().skip(skip),
            );
            for (distance, (current, hash_index)) in zipped {
                let &mut HashIndex::Occupied { hash, index } = hash_index else {
                    break 'outer AttachOperation::Replace { hash_index };
                };
                let probe_distance = hash.probe_distance(
                    probe_len,
                    current.try_into().map_err(|_| ArrayStorageError)?,
                );
                if distance > probe_distance {
                    break 'outer AttachOperation::TakeFromRich {
                        start_index: current,
                    };
                }
                if hash != entity_hash {
                    continue;
                }
                let &Bucket { key, .. } = buckets
                    .get(index)
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
                    if buckets.try_push(bucket).is_err() {
                        return Err(ArrayStorageError);
                    }
                    *hash_index = HashIndex::Occupied {
                        hash: entity_hash,
                        index: buckets.len() - 1,
                    };
                    return Ok(None);
                }
                &mut HashIndex::Occupied { index, .. } => {
                    let Bucket { value, .. } = buckets
                        .get_mut(index)
                        .expect("index should point to the valid bucket");
                    let component = mem::replace(value, component);
                    return Ok(Some(component));
                }
            },
            AttachOperation::TakeFromRich { start_index } => start_index,
        };

        let mut hash_index = HashIndex::Occupied {
            hash: entity_hash,
            index: buckets.len(),
        };
        skip = start_index;
        loop {
            for next_hash_index in indices.iter_mut().skip(skip) {
                if let &mut HashIndex::Free = next_hash_index {
                    hash_index = mem::replace(next_hash_index, hash_index);
                    continue;
                }
                let bucket = Bucket {
                    hash: entity_hash,
                    key: entity,
                    value: component,
                };
                if buckets.try_push(bucket).is_err() {
                    return Err(ArrayStorageError);
                }
                *next_hash_index = hash_index;
                return Ok(None);
            }
            skip = 0;
        }
    }
}

impl<T, E, S, const N: usize> HashArrayStorage<T, S, N, E>
where
    T: Component<Storage = Self>,
    E: Entity + PartialEq,
    E::Index: Hash,
    S: BuildHasher,
{
    fn find_bucket(&self, entity: E) -> Option<FindBucket> {
        let Self {
            buckets,
            indices,
            build_hasher,
        } = self;

        if buckets.is_empty() {
            return None;
        }
        let entity_hash = HashValue::new(build_hasher, entity.index());
        let probe_len = self.capacity().try_into().ok()?;
        let desired_index = entity_hash.desired_index(probe_len).try_into().ok()?;

        let mut skip = desired_index;
        let mut distances = 0..;
        'outer: loop {
            let zipped = iter::zip(distances.by_ref(), indices.iter().enumerate().skip(skip));
            for (distance, (current, hash_index)) in zipped {
                let &HashIndex::Occupied { hash, index } = hash_index else {
                    continue;
                };
                let probe_distance = hash.probe_distance(probe_len, current.try_into().ok()?);
                if distance > probe_distance {
                    break 'outer None;
                }
                if hash != entity_hash {
                    continue;
                }
                let &Bucket { key, .. } = buckets
                    .get(index)
                    .expect("index should point to the valid bucket");
                if entity != key {
                    continue;
                }
                let find_bucket = FindBucket {
                    hash_index: current,
                    bucket_index: index,
                };
                break 'outer Some(find_bucket);
            }
            skip = 0;
        }
    }

    /// Checks if a component is attached to provided entity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use core::hash::BuildHasherDefault;
    /// # use std::collections::hash_map::DefaultHasher;
    /// # use stream_ecs::component::Component;
    /// use stream_ecs::{component::storage::array::HashArrayStorage, entity::DefaultEntity};
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = HashArrayStorage<Self, BuildHasherDefault<DefaultHasher>, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = HashArrayStorage::new();
    /// let entity = DefaultEntity::new(100, 0);
    ///
    /// storage.attach(entity, Position { x: 0.0, y: 0.0 });
    /// assert!(storage.is_attached(entity));
    ///
    /// storage.remove(entity);
    /// assert!(!storage.is_attached(entity));
    /// ```
    pub fn is_attached(&self, entity: E) -> bool {
        self.find_bucket(entity).is_some()
    }

    /// Retrieves a reference to the component attached to provided entity.
    /// Returns [`None`] if provided entity does not have component of such type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use core::hash::BuildHasherDefault;
    /// # use std::collections::hash_map::DefaultHasher;
    /// # use stream_ecs::component::Component;
    /// use stream_ecs::{component::storage::array::HashArrayStorage, entity::DefaultEntity};
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = HashArrayStorage<Self, BuildHasherDefault<DefaultHasher>, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = HashArrayStorage::new();
    /// let entity = DefaultEntity::new(90, 12);
    ///
    /// storage.attach(entity, Position { x: 1.0, y: -1.0 });
    /// assert_eq!(storage.get(entity), Some(&Position { x: 1.0, y: -1.0 }));
    ///
    /// storage.remove(entity);
    /// assert_eq!(storage.get(entity), None);
    /// ```
    pub fn get(&self, entity: E) -> Option<&T> {
        let FindBucket { bucket_index, .. } = self.find_bucket(entity)?;
        let Bucket { value, .. } = self
            .buckets
            .get(bucket_index)
            .expect("index should point to the valid bucket");
        Some(value)
    }

    /// Retrieves a mutable reference to the component attached to provided entity.
    /// Returns [`None`] if provided entity does not have component of such type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use core::hash::BuildHasherDefault;
    /// # use std::collections::hash_map::DefaultHasher;
    /// # use stream_ecs::component::Component;
    /// use stream_ecs::{component::storage::array::HashArrayStorage, entity::DefaultEntity};
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = HashArrayStorage<Self, BuildHasherDefault<DefaultHasher>, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = HashArrayStorage::new();
    /// let entity = DefaultEntity::new(96, 12);
    ///
    /// storage.attach(entity, Position { x: 1.0, y: -1.0 });
    /// *storage.get_mut(entity).unwrap() = Position { x: 0.0, y: 2.0 };
    /// assert_eq!(storage.get_mut(entity), Some(&mut Position { x: 0.0, y: 2.0 }));
    ///
    /// storage.remove(entity);
    /// assert_eq!(storage.get_mut(entity), None);
    /// ```
    pub fn get_mut(&mut self, entity: E) -> Option<&mut T> {
        let FindBucket { bucket_index, .. } = self.find_bucket(entity)?;
        let Bucket { value, .. } = self
            .buckets
            .get_mut(bucket_index)
            .expect("index should point to the valid bucket");
        Some(value)
    }

    /// Removes component from provided entity.
    /// Returns previous component data, or [`None`] if there was no component attached to the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use core::hash::BuildHasherDefault;
    /// # use std::collections::hash_map::DefaultHasher;
    /// # use stream_ecs::component::Component;
    /// use stream_ecs::{component::storage::array::HashArrayStorage, entity::DefaultEntity};
    /// # #[derive(Debug, Clone, Copy, PartialEq, Component)]
    /// # #[component(storage = HashArrayStorage<Self, BuildHasherDefault<DefaultHasher>, 10>)]
    /// # #[component(crate = stream_ecs)]
    /// # struct Position {
    /// #     x: f32,
    /// #     y: f32,
    /// # }
    ///
    /// let mut storage = HashArrayStorage::new();
    /// let entity = DefaultEntity::new(127, 0);
    ///
    /// let component = storage.remove(entity);
    /// assert_eq!(component, None);
    ///
    /// storage.attach(entity, Position { x: 0.0, y: -10.0 });
    /// let component = storage.remove(entity);
    /// assert_eq!(component, Some(Position { x: 0.0, y: -10.0 }));
    /// ```
    pub fn remove(&mut self, entity: E) -> Option<T> {
        let FindBucket {
            hash_index,
            bucket_index,
        } = self.find_bucket(entity)?;

        {
            let hash_index = self
                .indices
                .get_mut(hash_index)
                .expect("index should point to the valid hash index");
            *hash_index = Self::EMPTY_INDEX;
        }

        let bucket = self
            .buckets
            .swap_pop(bucket_index)
            .expect("index should point to the valid bucket");
        if let Some(bucket) = self.buckets.get(bucket_index) {
            let &Bucket { hash, .. } = bucket;
            let probe_len = self.capacity().try_into().ok()?;
            let desired_index = hash.desired_index(probe_len).try_into().ok()?;
            let mut skip = desired_index;
            'outer: loop {
                for hash_index in self.indices.iter_mut().skip(skip) {
                    let &mut HashIndex::Occupied { index, .. } = hash_index else {
                        continue;
                    };
                    if index < self.buckets.len() {
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

        let mut last_current = hash_index;
        let probe_len = self.capacity();
        let mut skip = last_current + 1;
        'outer: loop {
            for current in skip..probe_len {
                let HashIndex::Occupied { hash, .. } = self.indices[current] else {
                    break 'outer;
                };
                let probe_len = probe_len.try_into().ok()?;
                let probe_distance = hash.probe_distance(probe_len, current.try_into().ok()?);
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
}

impl<T, E, S, const N: usize> Storage for HashArrayStorage<T, S, N, E>
where
    T: Component<Storage = Self>,
    E: Entity + PartialEq,
    E::Index: Hash + PartialEq,
    E::Generation: PartialOrd,
    S: BuildHasher + 'static,
{
    type Item = T;
    type Entity = E;

    fn attach(&mut self, entity: Self::Entity, component: Self::Item) -> Option<Self::Item> {
        HashArrayStorage::attach(self, entity, component)
    }

    fn is_attached(&self, entity: Self::Entity) -> bool {
        HashArrayStorage::is_attached(self, entity)
    }

    fn get(&self, entity: Self::Entity) -> Option<&Self::Item> {
        HashArrayStorage::get(self, entity)
    }

    fn get_mut(&mut self, entity: Self::Entity) -> Option<&mut Self::Item> {
        HashArrayStorage::get_mut(self, entity)
    }

    fn remove(&mut self, entity: Self::Entity) -> Option<Self::Item> {
        HashArrayStorage::remove(self, entity)
    }

    fn clear(&mut self) {
        HashArrayStorage::clear(self)
    }

    fn len(&self) -> usize {
        HashArrayStorage::len(self)
    }

    fn is_empty(&self) -> bool {
        HashArrayStorage::is_empty(self)
    }

    type Iter<'me>
        = Iter<'me, Self::Item, S, N, Self::Entity>
    where
        Self: 'me;

    fn iter(&self) -> Self::Iter<'_> {
        HashArrayStorage::iter(self)
    }

    type IterMut<'me>
        = IterMut<'me, Self::Item, S, N, Self::Entity>
    where
        Self: 'me;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        HashArrayStorage::iter_mut(self)
    }
}

impl<T, E, S, const N: usize> TryStorage for HashArrayStorage<T, S, N, E>
where
    T: Component<Storage = Self>,
    E: Entity + PartialEq,
    E::Index: Hash + PartialEq,
    E::Generation: PartialOrd,
    S: BuildHasher + 'static,
{
    type Err = ArrayStorageError;

    fn try_attach(
        &mut self,
        entity: Self::Entity,
        component: Self::Item,
    ) -> Result<Option<Self::Item>, Self::Err> {
        HashArrayStorage::try_attach(self, entity, component)
    }
}

impl<'me, T, E, S, const N: usize> IntoIterator for &'me HashArrayStorage<T, S, N, E>
where
    T: Component<Storage = HashArrayStorage<T, S, N, E>>,
    E: Entity,
{
    type Item = (E, &'me T);

    type IntoIter = Iter<'me, T, S, N, E>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.buckets.iter();
        Iter { iter }
    }
}

impl<'me, T, E, S, const N: usize> IntoIterator for &'me mut HashArrayStorage<T, S, N, E>
where
    T: Component<Storage = HashArrayStorage<T, S, N, E>>,
    E: Entity,
{
    type Item = (E, &'me mut T);

    type IntoIter = IterMut<'me, T, S, N, E>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.buckets.iter_mut();
        IterMut { iter }
    }
}

impl<T, E, S, const N: usize> IntoIterator for HashArrayStorage<T, S, N, E>
where
    T: Component<Storage = Self>,
    E: Entity,
{
    type Item = (E, T);

    type IntoIter = IntoIter<T, S, N, E>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.buckets.into_iter();
        IntoIter { iter }
    }
}

/// Iterator of entities with references of components attached to them
/// in the hash array storage.
#[derive(Debug, Clone)]
pub struct Iter<'data, T, S, const N: usize, E = DefaultEntity>
where
    T: Component<Storage = HashArrayStorage<T, S, N, E>>,
    E: Entity,
{
    iter: slice::Iter<'data, Bucket<E, T>>,
}

impl<'data, T, E, S, const N: usize> Iterator for Iter<'data, T, S, N, E>
where
    T: Component<Storage = HashArrayStorage<T, S, N, E>>,
    E: Entity,
{
    type Item = (E, &'data T);

    fn next(&mut self) -> Option<Self::Item> {
        let &Bucket { key, ref value, .. } = self.iter.next()?;
        Some((key, value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T, E, S, const N: usize> DoubleEndedIterator for Iter<'_, T, S, N, E>
where
    T: Component<Storage = HashArrayStorage<T, S, N, E>>,
    E: Entity,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let &Bucket { key, ref value, .. } = self.iter.next_back()?;
        Some((key, value))
    }
}

impl<T, E, S, const N: usize> ExactSizeIterator for Iter<'_, T, S, N, E>
where
    T: Component<Storage = HashArrayStorage<T, S, N, E>>,
    E: Entity,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T, E, S, const N: usize> FusedIterator for Iter<'_, T, S, N, E>
where
    T: Component<Storage = HashArrayStorage<T, S, N, E>>,
    E: Entity,
{
}

/// Iterator of entities with mutable references of components attached to them
/// in the hash array storage.
#[derive(Debug)]
pub struct IterMut<'data, T, S, const N: usize, E = DefaultEntity>
where
    T: Component<Storage = HashArrayStorage<T, S, N, E>>,
    E: Entity,
{
    iter: slice::IterMut<'data, Bucket<E, T>>,
}

impl<'data, T, E, S, const N: usize> Iterator for IterMut<'data, T, S, N, E>
where
    T: Component<Storage = HashArrayStorage<T, S, N, E>>,
    E: Entity,
{
    type Item = (E, &'data mut T);

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

impl<T, E, S, const N: usize> DoubleEndedIterator for IterMut<'_, T, S, N, E>
where
    T: Component<Storage = HashArrayStorage<T, S, N, E>>,
    E: Entity,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let &mut Bucket {
            key, ref mut value, ..
        } = self.iter.next_back()?;
        Some((key, value))
    }
}

impl<T, E, S, const N: usize> ExactSizeIterator for IterMut<'_, T, S, N, E>
where
    T: Component<Storage = HashArrayStorage<T, S, N, E>>,
    E: Entity,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T, E, S, const N: usize> FusedIterator for IterMut<'_, T, S, N, E>
where
    T: Component<Storage = HashArrayStorage<T, S, N, E>>,
    E: Entity,
{
}

/// Iterator of entities with components attached to them in the hash array storage.
#[derive(Debug, Clone)]
pub struct IntoIter<T, S, const N: usize, E = DefaultEntity>
where
    T: Component<Storage = HashArrayStorage<T, S, N, E>>,
    E: Entity,
{
    iter: arrayvec::IntoIter<Bucket<E, T>, N>,
}

impl<T, E, S, const N: usize> Iterator for IntoIter<T, S, N, E>
where
    T: Component<Storage = HashArrayStorage<T, S, N, E>>,
    E: Entity,
{
    type Item = (E, T);

    fn next(&mut self) -> Option<Self::Item> {
        let Bucket { key, value, .. } = self.iter.next()?;
        Some((key, value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T, E, S, const N: usize> DoubleEndedIterator for IntoIter<T, S, N, E>
where
    T: Component<Storage = HashArrayStorage<T, S, N, E>>,
    E: Entity,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let Bucket { key, value, .. } = self.iter.next_back()?;
        Some((key, value))
    }
}

impl<T, E, S, const N: usize> ExactSizeIterator for IntoIter<T, S, N, E>
where
    T: Component<Storage = HashArrayStorage<T, S, N, E>>,
    E: Entity,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T, E, S, const N: usize> FusedIterator for IntoIter<T, S, N, E>
where
    T: Component<Storage = HashArrayStorage<T, S, N, E>>,
    E: Entity,
{
}

#[cfg(test)]
mod tests {
    use core::hash::BuildHasherDefault;
    use std::collections::hash_map::DefaultHasher;

    use crate::{component::Component, entity::DefaultEntity as Entity};

    type HashArrayStorage<T, const N: usize> =
        super::HashArrayStorage<T, BuildHasherDefault<DefaultHasher>, N>;

    #[derive(Debug, Clone, Copy)]
    struct Marker;

    impl Component for Marker {
        type Storage = HashArrayStorage<Self, 10>;
    }

    #[test]
    fn new() {
        let storage = HashArrayStorage::<Marker, 10>::new();
        assert!(storage.is_empty());
    }

    #[test]
    fn attach() {
        let mut storage = HashArrayStorage::new();
        let entity = Entity::new(0, 0);

        let marker = storage.attach(entity, Marker);
        assert!(marker.is_none());
        assert!(storage.is_attached(entity));
    }

    #[test]
    fn attach_many() {
        let mut storage = HashArrayStorage::new();
        for index in 0..storage.capacity().try_into().unwrap() {
            let entity = Entity::new(index, 0);
            storage.attach(entity, Marker);
            assert!(storage.is_attached(entity));
        }
    }

    #[test]
    fn remove() {
        let mut storage = HashArrayStorage::new();
        let entity = Entity::new(1, 0);

        storage.attach(entity, Marker);
        let marker = storage.remove(entity);
        assert!(marker.is_some());
        assert!(!storage.is_attached(entity));
    }

    #[test]
    fn reattach() {
        let mut storage = HashArrayStorage::new();
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
        let mut storage = HashArrayStorage::new();
        for index in 0..11 {
            let entity = Entity::new(index, 0);
            storage.attach(entity, Marker);
        }
    }

    #[test]
    fn iter() {
        let mut storage = HashArrayStorage::new();
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
        let mut storage = HashArrayStorage::new();
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

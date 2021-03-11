use std::{default::default, mem::swap};

use crate::SmallSortedMap;

#[derive(Debug)]
pub(super) struct EntryInner<'m, K, V, const SIZE: usize>
where
    K: Ord + 'm,
    V: 'm,
{
    pub(super) map: &'m mut SmallSortedMap<K, V, SIZE>,
    pub(super) key: K,
    pub(super) pos: usize,
}

#[derive(Debug)]
pub struct OccupiedEntry<'m, K, V, const SIZE: usize>
where
    K: Ord + 'm,
    V: 'm,
{
    pub(super) inner: EntryInner<'m, K, V, SIZE>,
}

impl<'m, K, V, const SIZE: usize> OccupiedEntry<'m, K, V, SIZE>
where
    K: Ord + 'm,
    V: 'm,
{
    #[inline]
    pub fn get_mut(&mut self) -> &mut V {
        let EntryInner { map, pos, .. } = &mut self.inner;

        let (.., value) = &mut map.storage[*pos];
        value
    }

    #[inline]
    pub fn into_mut(self) -> &'m mut V {
        let EntryInner { map, pos, .. } = self.inner;

        let (.., value) = &mut map.storage[pos];
        value
    }

    #[inline]
    pub fn insert(&mut self, mut value: V) -> V {
        let old_value = self.get_mut();

        swap(old_value, &mut value);
        value
    }

    #[inline]
    pub fn remove(self) -> V {
        let EntryInner { map, pos, .. } = self.inner;

        let (.., value) = map.storage.remove(pos);
        value
    }
}

#[derive(Debug)]
pub struct VacantEntry<'m, K, V, const SIZE: usize>
where
    K: Ord + 'm,
    V: 'm,
{
    pub(super) inner: EntryInner<'m, K, V, SIZE>,
}

impl<'m, K, V, const SIZE: usize> VacantEntry<'m, K, V, SIZE>
where
    K: Ord + 'm,
    V: 'm,
{
    #[inline]
    pub fn insert(self, value: V) -> &'m mut V {
        let EntryInner { map, pos, key } = self.inner;

        map.storage.insert(pos, (key, value));

        let (.., value) = &mut map.storage[pos];
        value
    }
}

#[derive(Debug)]
pub enum Entry<'m, K, V, const SIZE: usize>
where
    K: Ord + 'm,
    V: 'm,
{
    Occupied(OccupiedEntry<'m, K, V, SIZE>),
    Vacant(VacantEntry<'m, K, V, SIZE>),
}

impl<'m, K, V, const SIZE: usize> Entry<'m, K, V, SIZE>
where
    K: Ord + 'm,
    V: Default + 'm,
{
    pub fn or_default(self) -> &'m mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default()),
        }
    }
}

pub mod entry;
pub mod iter;
#[cfg(feature = "serde")]
mod serde;

use std::{
    collections::HashMap,
    iter::FromIterator,
    ops::{Index, IndexMut},
};

use smallvec::SmallVec;

use self::entry::{Entry, EntryInner, OccupiedEntry, VacantEntry};
use self::iter::ValuesIter;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SmallSortedMap<K, V, const SIZE: usize>
where
    K: Ord,
{
    storage: SmallVec<[(K, V); SIZE]>,
}

impl<K, V, const SIZE: usize> SmallSortedMap<K, V, SIZE>
where
    K: Ord,
{
    #[inline]
    pub fn new() -> Self {
        Self {
            storage: SmallVec::new(),
        }
    }

    fn binary_search_by_key(&self, key: &K) -> Result<usize, usize> {
        self.storage.binary_search_by(|(other, ..)| other.cmp(key))
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    #[inline]
    pub fn get(&self, key: &K) -> Option<&V> {
        let pos = self.binary_search_by_key(key).ok()?;
        let (.., value) = self.storage.get(pos)?;

        Some(value)
    }

    #[inline]
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let pos = self.binary_search_by_key(key).ok()?;
        let (.., value) = self.storage.get_mut(pos)?;

        Some(value)
    }

    #[inline]
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.entry(key) {
            Entry::Occupied(mut entry) => Some(entry.insert(value)),
            Entry::Vacant(entry) => {
                entry.insert(value);
                None
            }
        }
    }

    #[inline]
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let pos = self.binary_search_by_key(key).ok()?;
        let (.., removed_value) = self.storage.remove(pos);

        Some(removed_value)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.storage.clear();
    }

    #[inline]
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V, SIZE> {
        let pos = self.binary_search_by_key(&key);

        let inner = EntryInner {
            map: self,
            key,
            pos: pos.into_ok_or_err(),
        };

        match pos {
            Ok(..) => Entry::Occupied(OccupiedEntry { inner }),
            Err(..) => Entry::Vacant(VacantEntry { inner }),
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[(K, V)] {
        self.storage.as_slice()
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [(K, V)] {
        self.storage.as_mut_slice()
    }

    #[inline]
    pub fn values(&self) -> ValuesIter<'_, K, V> {
        ValuesIter {
            slice: self.as_slice(),
        }
    }

    #[inline]
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.storage.retain(|(key, value)| f(key, value))
    }
}

impl<K, V, const SIZE: usize> AsRef<[(K, V)]> for SmallSortedMap<K, V, SIZE>
where
    K: Ord,
{
    #[inline]
    fn as_ref(&self) -> &[(K, V)] {
        self.as_slice()
    }
}

impl<K, V, const SIZE: usize> AsMut<[(K, V)]> for SmallSortedMap<K, V, SIZE>
where
    K: Ord,
{
    #[inline]
    fn as_mut(&mut self) -> &mut [(K, V)] {
        self.as_mut_slice()
    }
}

impl<K, V, const SIZE: usize> IntoIterator for SmallSortedMap<K, V, SIZE>
where
    K: Ord,
{
    type Item = (K, V);

    type IntoIter = smallvec::IntoIter<[(K, V); SIZE]>;

    fn into_iter(self) -> Self::IntoIter {
        self.storage.into_iter()
    }
}

impl<K, V, const SIZE: usize> Index<K> for SmallSortedMap<K, V, SIZE>
where
    K: Ord,
{
    type Output = V;

    #[inline]
    fn index(&self, index: K) -> &Self::Output {
        self.get(&index).expect("key not found in map")
    }
}

impl<K, V, const SIZE: usize> IndexMut<K> for SmallSortedMap<K, V, SIZE>
where
    K: Ord,
{
    #[inline]
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        self.get_mut(&index).expect("key not found in map")
    }
}

impl<K, V, const SIZE: usize> Index<&K> for SmallSortedMap<K, V, SIZE>
where
    K: Ord,
{
    type Output = V;

    #[inline]
    fn index(&self, index: &K) -> &Self::Output {
        self.get(index).expect("key not found in map")
    }
}

impl<K, V, const SIZE: usize> IndexMut<&K> for SmallSortedMap<K, V, SIZE>
where
    K: Ord,
{
    #[inline]
    fn index_mut(&mut self, index: &K) -> &mut Self::Output {
        self.get_mut(index).expect("key not found in map")
    }
}

impl<K, V, const SIZE: usize> Extend<(K, V)> for SmallSortedMap<K, V, SIZE>
where
    K: Ord,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|(key, value)| {
            self.insert(key, value);
        })
    }
}

impl<K, V, const SIZE: usize> FromIterator<(K, V)> for SmallSortedMap<K, V, SIZE>
where
    K: Ord,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut map = Self::new();
        map.extend(iter);

        map
    }
}

impl<S, K, V, const SIZE: usize> From<HashMap<K, V, S>> for SmallSortedMap<K, V, SIZE>
where
    K: Ord,
{
    fn from(map: HashMap<K, V, S>) -> Self {
        map.into_iter().collect()
    }
}

impl<K, V, const SIZE: usize> Default for SmallSortedMap<K, V, SIZE>
where
    K: Ord,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::ops::IndexMut;

    use crate::SmallSortedMap;

    #[test]
    fn it_works() {
        let mut map = SmallSortedMap::<usize, usize, 20>::new();

        map.insert(10, 10);
        map.insert(20, 20);

        assert_eq!(*map.get(&10).unwrap(), 10);
        assert_eq!(*map.get(&20).unwrap(), 20);

        assert!(map.get(&15).is_none());
        assert!(map.get(&25).is_none());

        assert_eq!(map[&10], 10);
        assert_eq!(*IndexMut::index_mut(&mut map, &20), 20);
    }

    #[test]
    fn test_values_iter() {
        let mut map = SmallSortedMap::<usize, usize, 20>::new();

        map.insert(20, 20);
        map.insert(10, 10);

        assert_eq!(map.values().copied().collect::<Vec<_>>(), &[10, 20]);
    }

    #[test]
    fn test_remove() {
        let mut map = SmallSortedMap::<usize, usize, 20>::new();

        map.insert(5, 5);
        map.insert(10, 10);
        map.insert(20, 20);
        assert!(map.get(&5).is_some());
        assert!(map.get(&10).is_some());
        assert!(map.get(&20).is_some());

        assert_eq!(map.remove(&10), Some(10));

        assert!(map.get(&5).is_some());
        assert!(map.get(&10).is_none());
        assert!(map.get(&20).is_some());
    }
}

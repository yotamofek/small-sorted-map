#[cfg(feature = "serde")]
mod serde;

use std::{
    collections::HashMap,
    iter::FromIterator,
    ops::{AddAssign as _, Deref, Index},
};

use crate::{Entry, SmallSortedMap, ValuesIter};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SmallCounter<K, const SIZE: usize>
where
    K: Ord,
{
    storage: SmallSortedMap<K, usize, SIZE>,
}

impl<K, const SIZE: usize> SmallCounter<K, SIZE>
where
    K: Ord,
{
    #[inline]
    pub fn new() -> Self {
        Self {
            storage: SmallSortedMap::new(),
        }
    }

    #[inline]
    pub fn add(&mut self, key: K) {
        self.storage.entry(key).or_default().add_assign(1);
    }

    pub fn remove(&mut self, key: K) -> bool {
        match self.storage.entry(key) {
            Entry::Occupied(mut entry) => {
                let count = entry.get_mut();

                if *count == 0 {
                    false
                } else {
                    *count -= 1;
                    true
                }
            }
            Entry::Vacant(..) => false,
        }
    }

    #[inline]
    pub fn get(&self, key: &K) -> &usize {
        self.storage.get(key).unwrap_or(&0)
    }

    #[inline]
    pub fn as_slice(&self) -> &[(K, usize)] {
        self.storage.as_slice()
    }

    #[inline]
    pub fn values(&self) -> ValuesIter<'_, K, usize> {
        self.storage.values()
    }

    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&K, &mut usize) -> bool,
    {
        self.storage.retain(f)
    }
}

impl<K, const SIZE: usize> Default for SmallCounter<K, SIZE>
where
    K: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, const SIZE: usize> Index<K> for SmallCounter<K, SIZE>
where
    K: Ord,
{
    type Output = usize;

    fn index(&self, index: K) -> &Self::Output {
        self.get(&index)
    }
}

impl<K, const SIZE: usize> Index<&K> for SmallCounter<K, SIZE>
where
    K: Ord,
{
    type Output = usize;

    fn index(&self, index: &K) -> &Self::Output {
        self.get(index)
    }
}

impl<K, const SIZE: usize> AsRef<[(K, usize)]> for SmallCounter<K, SIZE>
where
    K: Ord,
{
    fn as_ref(&self) -> &[(K, usize)] {
        self.as_slice()
    }
}

impl<K, const SIZE: usize> Deref for SmallCounter<K, SIZE>
where
    K: Ord,
{
    type Target = [(K, usize)];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<K, const SIZE: usize> IntoIterator for SmallCounter<K, SIZE>
where
    K: Ord,
{
    type Item = (K, usize);

    type IntoIter = smallvec::IntoIter<[(K, usize); SIZE]>;

    fn into_iter(self) -> Self::IntoIter {
        self.storage.into_iter()
    }
}

impl<K, const SIZE: usize> Extend<K> for SmallCounter<K, SIZE>
where
    K: Ord,
{
    fn extend<T: IntoIterator<Item = K>>(&mut self, iter: T) {
        iter.into_iter().for_each(|key| self.add(key));
    }
}

impl<K, const SIZE: usize> FromIterator<K> for SmallCounter<K, SIZE>
where
    K: Ord,
{
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        let mut counter = Self::new();

        counter.extend(iter.into_iter());
        counter
    }
}

impl<K, const SIZE: usize> FromIterator<(K, usize)> for SmallCounter<K, SIZE>
where
    K: Ord,
{
    fn from_iter<T: IntoIterator<Item = (K, usize)>>(iter: T) -> Self {
        Self {
            storage: iter.into_iter().collect(),
        }
    }
}

impl<S, K, const SIZE: usize> From<HashMap<K, usize, S>> for SmallCounter<K, SIZE>
where
    K: Ord,
{
    fn from(map: HashMap<K, usize, S>) -> Self {
        map.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use std::array;

    use crate::SmallCounter;

    #[test]
    fn it_works() {
        let mut counter = SmallCounter::<_, 10>::new();

        counter.extend(array::IntoIter::new([10, 20, 10, 10, 20]));

        assert_eq!(counter.len(), 2);
        assert_eq!(counter[10], 3);
        assert_eq!(counter[20], 2);
        assert_eq!(counter[15], 0);

        assert_eq!(counter.into_iter().collect::<Vec<_>>(), [(10, 3), (20, 2)]);
    }

    #[test]
    fn test_from_iterator() {
        let counter: SmallCounter<_, 10> = array::IntoIter::new([10, 20, 10, 10, 20]).collect();
        assert_eq!(counter.into_iter().collect::<Vec<_>>(), [(10, 3), (20, 2)]);
    }

    #[test]
    fn test_remove() {
        let mut counter: SmallCounter<_, 10> = array::IntoIter::new([10, 20, 10, 10, 20]).collect();

        assert!(counter.remove(10));
        assert!(counter.remove(20));
        assert!(counter.remove(20));
        assert!(!counter.remove(15));
        assert!(!counter.remove(25));

        assert_eq!(counter[10], 2);
        assert_eq!(counter[20], 0);
    }
}

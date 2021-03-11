pub struct ValuesIter<'a, K, V>
where
    K: Ord,
{
    pub(crate) slice: &'a [(K, V)],
}

impl<'a, K, V> Iterator for ValuesIter<'a, K, V>
where
    K: Ord,
{
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        let (.., value) = self.slice.get(0)?;

        self.slice = &self.slice[1..];

        Some(value)
    }
}

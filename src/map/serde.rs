use std::{any::type_name, default::default, fmt::Formatter, iter, marker::PhantomData};

use serde_crate::{
    de::{MapAccess, Visitor},
    ser::SerializeMap,
    Deserialize, Serialize, Serializer,
};

use crate::SmallSortedMap;

impl<K, V, const SIZE: usize> Serialize for SmallSortedMap<K, V, SIZE>
where
    K: Ord + Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut serializer = serializer.serialize_map(Some(self.len()))?;

        for (key, value) in self.as_slice() {
            serializer.serialize_entry(key, value)?;
        }

        serializer.end()
    }
}

struct MapVisitor<K, V, const SIZE: usize> {
    _marker: PhantomData<[(K, V); SIZE]>,
}

impl<'de, K, V, const SIZE: usize> Visitor<'de> for MapVisitor<K, V, SIZE>
where
    K: Ord + Deserialize<'de>,
    V: Deserialize<'de>,
{
    type Value = SmallSortedMap<K, V, SIZE>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "expecting map of {}: {}",
            type_name::<K>(),
            type_name::<V>()
        )
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        iter::from_fn(|| map.next_entry::<K, V>().transpose()).collect()
    }
}

impl<'de, K, V, const SIZE: usize> Deserialize<'de> for SmallSortedMap<K, V, SIZE>
where
    K: Ord + Deserialize<'de>,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde_crate::Deserializer<'de>,
    {
        deserializer.deserialize_map(MapVisitor { _marker: default() })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::SmallSortedMap;

    #[test]
    fn test_deserialize_json() {
        let json = json!({
            "10": 10,
            "20": 20,
        });

        let map: SmallSortedMap<usize, usize, 10> =
            serde_json::from_str(&json.to_string()).unwrap();

        assert_eq!(map[10], 10);
        assert_eq!(map[20], 20);
        assert!(map.get(&30).is_none())
    }
}

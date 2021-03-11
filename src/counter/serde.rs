use serde_crate::{Deserialize, Deserializer, Serialize, Serializer};

use super::SmallCounter;

impl<K, const SIZE: usize> Serialize for SmallCounter<K, SIZE>
where
    K: Ord + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.storage.serialize(serializer)
    }
}

impl<'de, K, const SIZE: usize> Deserialize<'de> for SmallCounter<K, SIZE>
where
    K: Ord + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self {
            storage: Deserialize::deserialize(deserializer)?,
        })
    }
}

use super::super::object_type::ObjectType;
use super::is_sorted;
use super::NodeEntry;
use super::{KeyTraits, ValTraits};
use crate::common::RawTyped;
use std::fmt;
use std::mem;

use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug, Clone)]
pub struct LeafNode<K, V> {
    pub entries: Vec<NodeEntry<K, V>>,
}

impl<K: serde::ser::Serialize, V: serde::ser::Serialize> Serialize for LeafNode<K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("LeafNode", 1)?;
        s.serialize_field("entries", &self.entries)?;
        s.end()
    }
}

impl<'de, K: serde::de::DeserializeOwned, V: serde::de::DeserializeOwned> Deserialize<'de>
    for LeafNode<K, V>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LeafNodeVisitor<K, V> {
            _p: std::marker::PhantomData<(K, V)>,
        }

        impl<'de, K: serde::de::DeserializeOwned, V: serde::de::DeserializeOwned> Visitor<'de>
            for LeafNodeVisitor<K, V>
        {
            type Value = LeafNode<K, V>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an LeafNode")
            }

            fn visit_seq<S>(self, mut seq: S) -> Result<LeafNode<K, V>, S::Error>
            where
                S: SeqAccess<'de>,
            {
                let entries = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                Ok(LeafNode { entries })
            }
        }

        const FIELDS: &[&str] = &["entries"];
        deserializer.deserialize_struct(
            "LeafNode",
            FIELDS,
            LeafNodeVisitor {
                _p: std::marker::PhantomData,
            },
        )
    }
}

impl<K, V> RawTyped for LeafNode<K, V> {
    const RAW_TYPE: ObjectType = ObjectType::LeafNode;
}

impl<K: KeyTraits, V: ValTraits> LeafNode<K, V> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    // TODO delete
    pub fn insert_local(&mut self, mut entry: NodeEntry<K, V>) -> Option<V> {
        // algo invariant: the entries should be sorted
        debug_assert!(is_sorted(self.entries.iter().map(|l| l.key)));

        let res = self.entries.binary_search_by_key(&entry.key, |e| e.key);
        match res {
            Ok(i) => {
                mem::swap(&mut self.entries[i].value, &mut entry.value);
                Some(entry.value)
            }
            Err(i) => {
                self.entries.insert(i, entry);
                None
            }
        }
    }
}

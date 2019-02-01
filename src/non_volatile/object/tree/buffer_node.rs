use super::super::object_type::ObjectType;
use super::*;

use std::fmt;
use std::mem;

use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Insert<V> {
    pub value: V,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Message<V> {
    Insert(Insert<V>),
}

#[derive(Debug, Clone)]
pub struct BufferNode<K, V> {
    pub entries: Vec<NodeEntry<K, Message<V>>>,
}

impl<K, V> Default for BufferNode<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> ConstObjType for BufferNode<K, V> {
    const OBJ_TYPE: ObjectType = ObjectType::BufferNode;
}

impl<K: serde::ser::Serialize, V: serde::ser::Serialize> Serialize for BufferNode<K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("BufferNode", 1)?;
        s.serialize_field("entries", &self.entries)?;
        s.end()
    }
}

impl<'de, K: serde::de::DeserializeOwned, V: serde::de::DeserializeOwned> Deserialize<'de>
    for BufferNode<K, V>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NodeVisitor<K, V> {
            _p: std::marker::PhantomData<(K, V)>,
        }

        impl<'de, K: serde::de::DeserializeOwned, V: serde::de::DeserializeOwned> Visitor<'de>
            for NodeVisitor<K, V>
        {
            type Value = BufferNode<K, V>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an Node")
            }

            fn visit_seq<S>(self, mut seq: S) -> Result<BufferNode<K, V>, S::Error>
            where
                S: SeqAccess<'de>,
            {
                let entries = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                Ok(BufferNode {
                    entries,
                })
            }
        }

        const FIELDS: &[&str] = &["entries"];
        deserializer.deserialize_struct(
            "BufferNode",
            FIELDS,
            NodeVisitor {
                _p: std::marker::PhantomData,
            },
        )
    }
}

impl<K, V> BufferNode<K, V> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn from(entries: Vec<NodeEntry<K, Message<V>>>) -> Self {
        Self {
            entries,
        }
    }

}

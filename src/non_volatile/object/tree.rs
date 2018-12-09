#![allow(clippy::block_in_if_condition_stmt)]

use std::fmt;
use std::marker::PhantomData;
use std::mem;

use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};

use super::super::serializable::Serializable;
use super::object_pointer::ObjectPointer;
use super::object_type::ObjectType;
use crate::common::ConstObjType;

pub mod any_node;
pub mod internal_node;
pub mod leaf_node;

pub use any_node::AnyNode;
pub use internal_node::InternalNode;
pub use leaf_node::LeafNode;

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct NodeEntry<K, V> {
    pub key: K,
    pub value: V,
}

impl<K, V> NodeEntry<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

#[inline]
pub fn is_sorted<I: Iterator<Item = T>, T: PartialOrd>(mut it: I) -> bool {
    let last: T = match it.next() {
        Some(i) => i,
        None => return true,
    };

    for i in it {
        if i <= last {
            return false;
        }
    }
    true
}

trait KeyTraits = Serializable + Copy;
trait ValTraits = Serializable;

#[derive(Debug, Clone)]
pub struct LeafType;
impl ConstObjType for LeafType {
    const OBJ_TYPE: ObjectType = ObjectType::LeafNode;
}

#[derive(Debug, Clone)]
pub struct InternalType;
impl ConstObjType for InternalType {
    const OBJ_TYPE: ObjectType = ObjectType::InternalNode;
}

/// Blanket implementation for all kind of Nodes
impl<K, V, OT: ConstObjType> ConstObjType for Node<K, V, OT> {
    const OBJ_TYPE: ObjectType = OT::OBJ_TYPE;
}

#[derive(Debug, Clone)]
pub struct Node<K, V, OT> {
    pub entries: Vec<NodeEntry<K, V>>,
    _ot: PhantomData<OT>,
}

impl<K: serde::ser::Serialize, V: serde::ser::Serialize, OT> Serialize for Node<K, V, OT> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Node", 1)?;
        s.serialize_field("entries", &self.entries)?;
        s.end()
    }
}

impl<'de, K: serde::de::DeserializeOwned, V: serde::de::DeserializeOwned, OT> Deserialize<'de>
    for Node<K, V, OT>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NodeVisitor<K, V, OT> {
            _p: std::marker::PhantomData<(K, V, OT)>,
        }

        impl<'de, K: serde::de::DeserializeOwned, V: serde::de::DeserializeOwned, OT> Visitor<'de>
            for NodeVisitor<K, V, OT>
        {
            type Value = Node<K, V, OT>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an Node")
            }

            fn visit_seq<S>(self, mut seq: S) -> Result<Node<K, V, OT>, S::Error>
            where
                S: SeqAccess<'de>,
            {
                let entries = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                Ok(Node {
                    entries,
                    _ot: PhantomData,
                })
            }
        }

        const FIELDS: &[&str] = &["entries"];
        deserializer.deserialize_struct(
            "Node",
            FIELDS,
            NodeVisitor {
                _p: std::marker::PhantomData,
            },
        )
    }
}

impl<K, V, OT> Node<K, V, OT> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            _ot: PhantomData,
        }
    }

    pub fn from(entries: Vec<NodeEntry<K, V>>) -> Self {
        Self {
            entries,
            _ot: PhantomData,
        }
    }

    // TODO delete
    pub fn insert_local(&mut self, mut entry: NodeEntry<K, V>) -> Option<V>
    where
        K: Ord + Copy,
    {
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

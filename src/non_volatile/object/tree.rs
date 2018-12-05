#![allow(clippy::block_in_if_condition_stmt)]

use crate::common::RawTyped;
use super::object_pointer::ObjectPointer;
use super::object_type::ObjectType;
use super::super::serializable::Serializable;

pub mod any_node;
pub mod internal_node;
pub mod leaf_node;

pub use any_node::AnyNode;
pub use internal_node::InternalNode;
pub use leaf_node::LeafNode;

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct NodeEntry<K, V> {
    key: K,
    value: V,
}

impl<K: KeyTraits, V: ValTraits> NodeEntry<K, V> {
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

trait KeyTraits = Serializable + Ord + Copy;
trait ValTraits = Serializable;

/*
#[derive(Debug, serde_derive::Serialize)]
pub struct Node<K, V> {
    entries: Vec<NodeEntry<K,V>>
}



impl<K: KeyTraits, V: ValTraits> Node<K, V> {
    pub fn insert(&mut self, mut entry: NodeEntry<K, V>) -> Option<V> {
        // algo invariant: the entries should be sorted
        debug_assert!(is_sorted(self.entries.iter().map(|l|{l.key})));

        let res = self.entries.binary_search_by_key(&entry.key, |e| e.key);
        match res {
            Ok(i)  => {
                mem::swap(&mut self.entries[i].value, &mut entry.value);
                Some(entry.value)
            },
            Err(i) => {
                self.entries.insert(i, entry);
                None
            },
        }
    }
}

*/

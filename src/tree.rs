#![allow(clippy::block_in_if_condition_stmt)]

use super::object_type::ObjectType;
use super::object_pointer::ObjectPointer;
use std::mem;

pub trait KeyTraits = serde::Serialize + Ord + Copy;
pub trait ValTraits = serde::Serialize;

#[derive(Debug, serde_derive::Serialize)]
pub struct NodeEntry<K: KeyTraits, V: ValTraits> {
    key: K,
    value: V,
}

impl<K: KeyTraits, V: ValTraits> NodeEntry<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
        }
    }
}

#[derive(Debug, serde_derive::Serialize)]
pub struct Node<K: KeyTraits, V: ValTraits> {
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


#[inline]
pub fn is_sorted<I: Iterator<Item=T>, T: PartialOrd>(mut it: I) -> bool {
    let last: T = match it.next() {
        Some(i) => i,
        None => return true
    };

    for i in it {
        if i <= last {
            return false;
        }
    }
    true
}

pub struct InternalNode<K: KeyTraits> {
    entries: Vec<NodeEntry<K, ObjectPointer>>
}

impl<K: KeyTraits> super::common::RawTyped for InternalNode<K> {
    const RAW_TYPE: ObjectType = ObjectType::InternalNode;
}

#[derive(serde_derive::Serialize)]
pub struct LeafNode<K: KeyTraits, V: ValTraits> {
    entries: Vec<NodeEntry<K, V>>
}

impl<K: KeyTraits, V: ValTraits> super::common::RawTyped for LeafNode<K, V> {
    const RAW_TYPE: ObjectType = ObjectType::LeafNode;
}

impl<K: KeyTraits> InternalNode<K> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new()
        }
    }

    pub fn insert_local(&mut self, entry: NodeEntry<K, ObjectPointer>) -> Option<ObjectPointer> {
        // algo invariant: the entries should be sorted
        debug_assert!(is_sorted(self.entries.iter().map(|l|{l.key})));

        let res = self.entries.binary_search_by_key(&entry.key, |e| e.key);
        match res {
            Ok(i)  => unreachable!("cow_btree: trying to insert in an InternalNode but key already exists"),
            Err(i) => {
                self.entries.insert(i, entry);
                None
            },
        }
    }
}

impl<K: KeyTraits, V: ValTraits> LeafNode<K, V> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new()
        }
    }

    pub fn insert_local(&mut self, mut entry: NodeEntry<K, V>) -> Option<V> {
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
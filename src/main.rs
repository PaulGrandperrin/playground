#![feature(dbg_macro, uniform_paths, trait_alias, never_type)]
#![allow(dead_code, unused_variables, clippy::needless_pass_by_value)]

use std::collections::BTreeMap;

mod object_pointer;
mod file_backend;
mod uberblock;
mod object_type;
mod tree;
mod context;
mod common; // RawSized
//mod algo; // Tree algorithm


use crate::object_pointer::ObjectPointer;
use crate::object_type::ObjectType;
use crate::context::Context;
use crate::tree::{LeafNode, NodeEntry};

fn main() {
    let mut ctx = Context::new();
    let mut ln = LeafNode::<u64, u64>::new();
    ln.insert_local(NodeEntry::new(1,1001));
    ln.insert_local(NodeEntry::new(2,1002));
    ln.insert_local(NodeEntry::new(3,1003));
    let op = ctx.store(&ln);
    ctx.commit(op.clone());
    ctx.commit(op.clone());
    ctx.commit(op.clone());
    ctx.commit(op.clone());

}



















trait SortedMap<K: std::cmp::Ord, V> {
    fn get(&self, key: &K) -> Option<&V>;
    fn put(&mut self, key: K, value: V) -> Option<V>;
    fn remove(&mut self, key: &K) -> Option<V>;
    // range
}

impl<K: std::cmp::Ord, V> SortedMap<K, V> for BTreeMap<K, V> {
    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }
    fn put(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }
}


fn process_sortedmap(sm: &mut SortedMap<&str, &str>) {
    dbg!(sm.put("a", "b"));
    dbg!(sm.put("c", "d"));
    dbg!(sm.get(&"a"));
    dbg!(sm.remove(&"c"));
    dbg!(sm.get(&"c"));

}
#![feature(non_exhaustive, dbg_macro, uniform_paths, trait_alias, never_type)]
#![allow(dead_code, unused_variables, clippy::needless_pass_by_value)]

use std::collections::BTreeMap;

mod object_pointer;
mod file_backend;
mod uberblock;
mod object_type;
mod tree;
mod context;
//mod serialize;
mod common; // RawSized
mod space_manager;
//mod cache;
//mod algo; // Tree algorithm

use crate::object_pointer::ObjectPointer;
use crate::context::Context;
use crate::tree::{AnyNode};

fn main() {
    //algo::test();

    let mut ctx = Context::format_and_load();
    ctx.insert(1, 1001);
    ctx.insert(2, 1002);
    ctx.insert(3, 1003);
    ctx.commit();
    ctx.commit();
    ctx.commit();
    ctx.commit();

    let mut ctx = Context::load().unwrap();
    dbg!(&ctx);
    let op = ctx.tree_root_pointer.clone();
    let root = ctx.get::<AnyNode<u64,u64>>(&op);
    dbg!(&root);
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
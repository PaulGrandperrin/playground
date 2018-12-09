#![feature(
    trivial_bounds,
    try_from,
    non_exhaustive,
    dbg_macro,
    uniform_paths,
    trait_alias,
    never_type
)]
#![allow(dead_code, unused_variables, clippy::needless_pass_by_value)]

use std::collections::BTreeMap;

mod algorithm;
mod common; // RawSized
mod context;
mod non_volatile; // Tree algorithm

use crate::context::Context;

fn main() {
    println!("format and load");
    let mut ctx = Context::new();

    println!("insert 1");
    ctx.insert2(1, 1001);
    println!("insert 2");
    ctx.insert2(2, 1002);
    println!("insert 3");
    ctx.insert2(3, 1003);
    println!("insert 4");
    ctx.insert2(4, 1004);

    dbg!(&ctx);
    ctx.read_all();
    drop(ctx);

    println!("load");
    let mut ctx = Context::load();
    dbg!(&ctx);
    ctx.read_all();

    
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

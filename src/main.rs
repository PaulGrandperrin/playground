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

    for i in 1..1000 {
        println!("insert {}", i);
        ctx.insert(i, 1000+i);
    }
    return;
    ctx.commit();

    let i=20;
    println!("insert {}", i);
    ctx.insert(i, 1000+i);

    let i=21;
    println!("insert {}", i);
    ctx.insert(i, 1000+i);
    
    let i=22;
    println!("insert {}", i);
    ctx.insert(i, 1000+i);

    //ctx.commit();

    let i=19;
    println!("insert {}", i);
    ctx.insert(i, 1000+i);


    //dbg!(&ctx);
    //drop(ctx);
//
    //println!("load");
    //let mut ctx = Context::load();
    //dbg!(&ctx);
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

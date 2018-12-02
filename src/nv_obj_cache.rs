use std::collections::hash_map::Entry;
use std::collections::HashMap; // maybe use https://github.com/Amanieu/hashbrown
use crate::any_object::{AnyObject, Object};

#[derive(Debug)]
pub struct NVObjectCache {
    map: HashMap<u64, AnyObject>,
}

impl NVObjectCache {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    // get

    // insert
}
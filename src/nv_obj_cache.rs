use std::collections::hash_map::Entry;
use crate::object_pointer::ObjectPointer;
use std::collections::HashMap; // maybe use https://github.com/Amanieu/hashbrown
use std::rc::Rc;
use std::convert::TryInto;
use crate::any_object::{AnyObject, Object};

// TODO rename anyobject to cachedObject/cacheEntry

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

    pub fn get<O: Object>(&self, op: &ObjectPointer) -> Option<Rc<O>> {
        self.map.get(&op.offset).map(|o|{
            o.clone().try_into().unwrap()
        })
    } 

    pub fn insert<O: Object>(&mut self, op: &ObjectPointer, obj: O) {
        // TODO implement some kind of replacement
        // RR / LRU / ARC / CAR 
        self.map.insert(op.offset, obj.into());
    }
}
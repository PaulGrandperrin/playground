use crate::object_pointer::ObjectPointer;

use std::collections::HashMap; // maybe use https://github.com/Amanieu/hashbrown

trait Cache<O> {
    fn store(&mut self, object: &O) -> ObjectPointer
    where O: serde::Serialize + super::common::RawTyped;

    fn retrieve(&mut self, op: &ObjectPointer) -> O
    where O: serde::de::DeserializeOwned;
}

struct BasicCache<O> {
    map: HashMap<u64, O>
}

impl<O> Cache<O> for BasicCache<O> {
    fn store(&mut self, object: &O) -> ObjectPointer
    where O: serde::Serialize + super::common::RawTyped {
        map.insert()
        unimplemented!()
    }

    fn retrieve(&mut self, op: &ObjectPointer) -> O
    where O: serde::de::DeserializeOwned {
        unimplemented!()
    }
}
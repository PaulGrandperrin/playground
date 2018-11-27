use crate::object_pointer::ObjectPointer;
use crate::space_manager::SpaceManager;
use crate::serializable::Serializable;
use crate::any_object::{AnyObject, Object};
use std::collections::hash_map::Entry;
use crate::file_backend::FileBackend;
use std::rc::Rc;
use std::collections::HashMap; // maybe use https://github.com/Amanieu/hashbrown
use std::convert::TryInto;
use std::ops::Deref;

#[derive(Debug)]
pub struct CachedSpaceManager {
    sm: SpaceManager,
    map: HashMap<u64, AnyObject>,
}

impl CachedSpaceManager {
    pub fn new(offset: u64) -> Self {
        Self {
            sm: SpaceManager::new(offset),
            map: HashMap::new(),
        }
    }

    pub fn store<O: Object>(&mut self, object: O) -> ObjectPointer {
        let op = self.sm.store(&object);
        self.map.insert(op.offset, object.into());
        op
    }

    pub fn retrieve<O: Object>(&mut self, op: &ObjectPointer) -> Rc<O> {
        match self.map.entry(op.offset) {
            Entry::Occupied(e) => {
                println!("cache hit :-)");
                let en: AnyObject = e.get().try_into().unwrap();//.try_into().unwrap();
                (*e.get()).try_into().unwrap() //FIXME: compiler problem? e.get().deref().try_into()
                // here, implicitly, we drop the old value
            }
            Entry::Vacant(e) => {
                println!("cache miss :-(");
                let o = self.sm.retrieve::<O>(op);
                let rc = Rc::new(o);
                let v = e.insert(rc.into());
                rc
            }
        }
    }

    pub fn get_mut_free_space_offset(&mut self) -> &mut u64 {
        &mut self.sm.free_space_offset
    }

    pub fn get_mut_block_dev(&mut self) -> &mut FileBackend {
        &mut self.sm.block_dev
    }

    pub fn get_mut_sm(&mut self) -> &mut SpaceManager {
        &mut self.sm
    }
}
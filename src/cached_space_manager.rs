use crate::object_pointer::ObjectPointer;
use crate::space_manager::SpaceManager;
use crate::serializable::Serializable;
use std::collections::hash_map::Entry;
use crate::file_backend::FileBackend;
use std::rc::Rc;
use std::collections::HashMap; // maybe use https://github.com/Amanieu/hashbrown

#[derive(Debug)]
pub struct CachedSpaceManager<O> {
    sm: SpaceManager,
    map: HashMap<u64, Rc<O>>, 
}

impl<O> CachedSpaceManager<O> {
    pub fn new(offset: u64) -> Self {
        Self {
            sm: SpaceManager::new(offset),
            map: HashMap::new(),
        }
    }

    pub fn store(&mut self, object: impl Into<Rc<O>>) -> ObjectPointer
    where O: Serializable {
        let rco = object.into();
        let op = self.sm.store::<O>(&rco);
        self.map.insert(op.offset, rco);
        op
    }

    pub fn retrieve(&mut self, op: &ObjectPointer) -> Rc<O>
    where O: Serializable {
        match self.map.entry(op.offset) {
            Entry::Occupied(e) => {
                println!("cache hit :-)");
                e.get().clone()
            }
            Entry::Vacant(e) => {
                println!("cache miss :-(");
                let o = self.sm.retrieve::<O>(op);
                e.insert(Rc::new(o)).clone()
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
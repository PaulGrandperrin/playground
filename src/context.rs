use crate::non_volatile::manager::NVObjectManager;
use crate::non_volatile::object::object_pointer::ObjectPointer;
use crate::non_volatile::object::tree::LeafNode;
use crate::non_volatile::object::tree::NodeEntry;

use std::collections::BTreeMap;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
pub struct Context {
    nv_obj_mngr: NVObjectManager,
    pub op: ObjectPointer,
    buffer: BTreeMap<u64, u64>,
}

impl Context {
    pub fn new() -> Context {
        let leaf = LeafNode::<u64, u64>::new();
        let (nv_obj_mngr, op) = NVObjectManager::new(leaf);

        Self {
            nv_obj_mngr,
            op,
            buffer: BTreeMap::new(),
        }
    }

    pub fn load() -> Context {
        let (nv_obj_mngr, op) = NVObjectManager::load();

        Self {
            nv_obj_mngr,
            op,
            buffer: BTreeMap::new(),
        }
    }

    pub fn commit(&mut self) {
        self.nv_obj_mngr.commit(&self.op);
    }

    pub fn insert(&mut self, k: u64, v: u64) {
        let mut leaf = self
            .nv_obj_mngr
            .get::<LeafNode<u64, u64>>(&self.op)
            .deref()
            .clone();
        leaf.insert_local(NodeEntry::new(k, v));
        self.op = self.nv_obj_mngr.store(leaf);
        self.nv_obj_mngr.commit(&self.op);
    }

    pub fn read_all(&mut self) {
        let mut leaf = self
            .nv_obj_mngr
            .get::<LeafNode<u64, u64>>(&self.op)
            .deref()
            .clone();
        println!("{:?}", leaf);
    }
}

use std::collections::BTreeMap;
use crate::tree::any_node::AnyNode;
use super::object_pointer::ObjectPointer;
use super::uberblock::Uberblock;
use super::tree::{LeafNode, NodeEntry};
use super::algo;
use std::rc::Rc;
use crate::serializable::Serializable;
use crate::object_type::ObjectType;
use crate::any_object::{AnyObject, Object};
use crate::nv_obj_mngr::NVObjectManager;
use std::ops::Deref;

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
        let mut leaf = self.nv_obj_mngr.get::<LeafNode<u64, u64>>(&self.op).deref().clone();
        leaf.insert_local(NodeEntry::new(1, 1001));
        self.op = self.nv_obj_mngr.store(leaf);
        self.nv_obj_mngr.commit(&self.op);
    }
 
}

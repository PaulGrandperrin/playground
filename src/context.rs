use crate::algorithm;
use crate::common::ConstObjType;
use crate::non_volatile::manager::NVObjectManager;
use crate::non_volatile::object::object_pointer::ObjectPointer;
use crate::non_volatile::object::object_type::ObjectType;
use crate::non_volatile::object::tree::{LeafNode, LeafType, NodeEntry};

use std::collections::BTreeMap;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
pub struct Context {
    nv_obj_mngr: NVObjectManager,
    pub head: ObjectPointer,
    buffer: BTreeMap<u64, u64>,
}

impl Context {
    pub fn new() -> Context {
        let (nv_obj_mngr, head) = NVObjectManager::new(algorithm::b_epsilon_tree::new());

        Self {
            nv_obj_mngr,
            head,
            buffer: BTreeMap::new(),
        }
    }

    pub fn load() -> Context {
        let (nv_obj_mngr, head) = NVObjectManager::load();

        Self {
            nv_obj_mngr,
            head,
            buffer: BTreeMap::new(),
        }
    }

    pub fn commit(&mut self) {
        println!("Commiting: {:?}", self.buffer);
        let mut swap_buffer = BTreeMap::new();
        std::mem::swap(&mut swap_buffer, &mut self.buffer);
        self.head =
            algorithm::b_epsilon_tree::merge_tree(swap_buffer, &mut self.nv_obj_mngr, &self.head);
        self.nv_obj_mngr.commit(&self.head);
        self.debug();
    }

    pub fn insert(&mut self, k: u64, v: u64) {
        self.buffer.insert(k, v);
        if self.buffer.len() >= 3 { // TODO make configurable
            self.commit();
        }
    }

    pub fn debug(&mut self) {
        algorithm::b_epsilon_tree::debug(0, &mut self.nv_obj_mngr, &self.head);
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        println!("dropping Context");
        self.commit();
    }
}

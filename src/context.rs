use crate::algorithm;
use crate::common::ConstObjType;
use crate::non_volatile::manager::NVObjectManager;
use crate::non_volatile::object::object_pointer::ObjectPointer;
use crate::non_volatile::object::object_type::ObjectType;
use crate::non_volatile::object::tree::{LeafNode, LeafType, Node, NodeEntry};

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
        let leaf = LeafNode::<u64, u64>::new();
        let (nv_obj_mngr, head) = NVObjectManager::new(leaf);

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
        println!("Commiting");
        let mut swap_buffer = BTreeMap::new();
        std::mem::swap(&mut swap_buffer, &mut self.buffer);
        self.head =
            algorithm::b_epsilon_tree::merge_tree(swap_buffer, &mut self.nv_obj_mngr, &self.head);
        self.nv_obj_mngr.commit(&self.head);
    }

    pub fn insert(&mut self, k: u64, v: u64) {
        let mut leaf = self
            .nv_obj_mngr
            .get::<LeafNode<u64, u64>>(&self.head)
            .deref()
            .clone();
        leaf.insert_local(NodeEntry::new(k, v));
        self.head = self.nv_obj_mngr.store(leaf);
        self.nv_obj_mngr.commit(&self.head);
    }

    pub fn insert2(&mut self, k: u64, v: u64) {
        self.buffer.insert(k, v);
        if self.buffer.len() >= 3 {
            // TODO make configurable
            println!("Context's buffer full (3)");
            self.commit();
        }
    }

    pub fn read_all(&mut self) {
        let mut leaf = self
            .nv_obj_mngr
            .get::<LeafNode<u64, u64>>(&self.head)
            .deref()
            .clone();
        println!("{:?}", leaf);
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        println!("dropping Context");
        self.commit();
    }
}

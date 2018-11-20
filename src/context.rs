use crate::tree::any_node::AnyNode;
use super::object_pointer::ObjectPointer;
use super::uberblock::Uberblock;
use super::tree::{LeafNode, NodeEntry};
use super::space_manager::SpaceManager;

use itertools::Itertools;

#[derive(Debug)]
pub struct Context {
    pub sm: SpaceManager,
    tgx: u64,
    pub tree_root_pointer: ObjectPointer,
}


impl Context {

    pub fn format_and_load() -> Context { // TODO split
        let mut sm = SpaceManager::new();
        let root_node = LeafNode::<u64, u64>::new();
        let trp = sm.store(&root_node);
        let ub = Uberblock::new(0, trp.clone(), sm.free_space_offset);
        let ub_mem = bincode::serialize(&ub).unwrap();
        sm.block_dev.write(0, &ub_mem);
        Context {
            sm,
            tgx: 1,
            tree_root_pointer: trp,
        }
    }

    pub fn load() -> Result<Context, failure::Error> { // TODO move uberblock finding to SpaceManager
        let mut sm = SpaceManager::new();
        let ub = (0..SpaceManager::NUM_UBERBLOCKS).map(|i| {
            Ok::<_, failure::Error>(sm.retrieve::<Uberblock>(&ObjectPointer::new(i as u64 * Uberblock::RAW_SIZE as u64, Uberblock::RAW_SIZE as u64)))
        }).fold_results(None::<Uberblock>, |acc, u| { // compute max if no error
            if let Some(acc) = acc {
                if u.tgx <= acc.tgx {
                    return Some(acc)
                }
            }
            Some(u)
        }).map(|o| {
            o.unwrap() // guaranted to succeed
        })?;

        sm.free_space_offset = ub.free_space_offset;
        
        Ok(Context{
            tgx: ub.tgx,
            sm,
            tree_root_pointer: ub.tree_root_pointer,
        })
    }
    

    pub fn commit(&mut self) { // TODO move to SpaceManager
        let ub = Uberblock::new(self.tgx, self.tree_root_pointer.clone(), self.sm.free_space_offset);
        let ub_mem = bincode::serialize(&ub).unwrap();
        let ub_offset = (self.tgx % SpaceManager::NUM_UBERBLOCKS) * Uberblock::RAW_SIZE as u64;
        self.sm.block_dev.write(ub_offset, &ub_mem);
        self.tgx += 1;
    }

    pub fn save<O>(&mut self, object: &O) -> ObjectPointer
    where O: serde::Serialize + super::common::RawTyped {
        // TODO: implement cache
        self.sm.store(object)
    }

    pub fn get<O>(&mut self, op: &ObjectPointer) -> O
    where O: serde::de::DeserializeOwned {
        // TODO: implement cache
        self.sm.retrieve(op)
    }

    pub fn insert(&mut self, k: u64, v: u64) {
        let any_node = self.get::<AnyNode<u64,u64>>(&self.tree_root_pointer.clone());
        self.tree_root_pointer = match any_node {
            AnyNode::LeafNode(mut node) => {
                node.insert_local(NodeEntry::new(k, v));
                self.save(&node)
            },
            AnyNode::InternalNode(node) => {
                unimplemented!()
            }
        };
    }
 
}

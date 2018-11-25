use std::collections::BTreeMap;
use crate::tree::any_node::AnyNode;
use super::object_pointer::ObjectPointer;
use super::uberblock::Uberblock;
use super::tree::{LeafNode, NodeEntry};
use super::cached_space_manager::CachedSpaceManager;
use super::algo;
use std::rc::Rc;
use std::ops::Deref;

use itertools::Itertools;

#[derive(Debug)]
pub struct Context {
    pub csm: CachedSpaceManager<AnyNode<u64, u64>>,
    tgx: u64,
    pub tree_root_pointer: ObjectPointer,
    buffer: BTreeMap<u64, u64>,
}


impl Context {
    const NUM_UBERBLOCKS: u64 = 3;

    pub fn format_and_load() -> Context { // TODO split
        let mut csm = CachedSpaceManager::new(Self::NUM_UBERBLOCKS * Uberblock::RAW_SIZE as u64);
        let root_node = AnyNode::LeafNode(LeafNode::<u64, u64>::new());
        let trp = csm.store(root_node);
        let ub = Uberblock::new(0, trp.clone(), *csm.get_mut_free_space_offset());
        let ub_mem = bincode::serialize(&ub).unwrap();
        csm.get_mut_block_dev().write(0 * Uberblock::RAW_SIZE as u64, &ub_mem);
        csm.get_mut_block_dev().write(1 * Uberblock::RAW_SIZE as u64, &ub_mem);
        csm.get_mut_block_dev().write(2 * Uberblock::RAW_SIZE as u64, &ub_mem);
        Context {
            csm,
            tgx: 1,
            tree_root_pointer: trp,
            buffer: BTreeMap::new(),
        }
    }

    pub fn load() -> Result<Context, failure::Error> { // TODO move uberblock finding to SpaceManager
        let mut csm = CachedSpaceManager::new(Self::NUM_UBERBLOCKS * Uberblock::RAW_SIZE as u64);
        let ub = (0..Self::NUM_UBERBLOCKS).map(|i| {
            Ok::<_, failure::Error>(
                csm.get_mut_sm().retrieve::<Uberblock>(
                    &ObjectPointer::new(i as u64 * Uberblock::RAW_SIZE as u64, Uberblock::RAW_SIZE as u64)
                )
            )
        }).fold_results(None::<Uberblock>, |acc, u: Uberblock| { // compute max if no error
            if let Some(acc) = acc {
                if u.tgx <= acc.tgx {
                    return Some(acc)
                }
            }
            Some(u)
        }).map(|o| {
            o.unwrap() // guaranted to succeed
        })?;

        *csm.get_mut_free_space_offset() = ub.free_space_offset;
        
        Ok(Context{
            tgx: ub.tgx,
            csm,
            tree_root_pointer: ub.tree_root_pointer,
            buffer: BTreeMap::new(),
        })
    }
    

    pub fn commit(&mut self) { // TODO move to SpaceManager
        // merge buffer's data into the B^ε-tree using COW
        //algo::merge(&self.buffer, &self.tree_root_pointer, &mut self.sm);

        // write new uber
        let ub = Uberblock::new(self.tgx, self.tree_root_pointer.clone(), *self.csm.get_mut_free_space_offset());
        let ub_mem = bincode::serialize(&ub).unwrap();
        let ub_offset = (self.tgx % Self::NUM_UBERBLOCKS) * Uberblock::RAW_SIZE as u64;
        self.csm.get_mut_block_dev().write(ub_offset, &ub_mem);
        self.tgx += 1;
    }

    pub fn save(&mut self,object: impl Into<Rc<AnyNode<u64, u64>>>) -> ObjectPointer {
        self.csm.store(object)
    }

    pub fn get(&mut self, op: &ObjectPointer) -> Rc<AnyNode<u64, u64>> {
        self.csm.retrieve(op)
    }

    pub fn insert(&mut self, k: u64, v: u64) {
        let trp = self.tree_root_pointer.clone();
        let any_node = self.get(&trp);

        self.tree_root_pointer = match any_node.as_ref() {
            AnyNode::LeafNode(node) => {
                let mut node = node.clone();
                node.insert_local(NodeEntry::new(k, v));
                self.save(AnyNode::LeafNode(node))
            },
            _ => {
                unimplemented!()
            }
        };
    }
 
}

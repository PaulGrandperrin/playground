use std::collections::BTreeMap;

use itertools::Itertools;

use crate::non_volatile::manager::NVObjectManager;
use crate::non_volatile::object::object_pointer::ObjectPointer;
use crate::non_volatile::object::object_type::ObjectType;
use crate::non_volatile::object::tree::LeafNode;
use crate::non_volatile::object::tree::NodeEntry;

pub mod b_epsilon_tree {
    use super::*;

    const B: usize = 5;

    pub fn merge_bulk_operation(ops: BTreeMap<u64, u64>, nv_obj_mngr: &mut NVObjectManager, op: &ObjectPointer) -> ObjectPointer {
        match op.object_type {
            // we point to a leaf
            ObjectType::LeafNode => {
                // get the leaf 
                let leaf = nv_obj_mngr.get::<LeafNode<u64, u64>>(op);
                // is the leaf big enough to merge with buffer
                if leaf.entries.len() + ops.len() <= B {
                    // do a sorted merge
                    let it_leaf = leaf.entries.iter().map(|e| e.clone());
                    let it_ops = ops.into_iter().map(|(k, v)| { NodeEntry::new(k, v)});
                    let entries: Vec<_> = it_leaf.merge_by(it_ops, |a, b| { a.key <= b.key }).collect();
                    let new_leaf = LeafNode{entries};

                    // write new leaf to nv device
                    let op = nv_obj_mngr.store(new_leaf);

                    op
                } else {
                    unimplemented!()
                }
            },
            _ => {
                unimplemented!()
            }
        }
    }
}

/*
pub fn merge(buffer: &BTreeMap<u64, u64>, trp: &ObjectPointer, sm: &mut SpaceManager)  {
    let any_node = sm.retrieve::<AnyNode<u64,u64>>(trp);
    match any_node {
        AnyNode::LeafNode(node) => {
            unimplemented!()

            //let it_buf = buffer.into_iter();
            //let it_node = node.entries.into_iter();


        },
        AnyNode::InternalNode(node) => {
            unimplemented!()
        }
    }

}
*/

//trait AllTraits<'a> = serde::Serialize + serde::de::Deserialize<'a> + std::fmt::Debug;
/*
fn bla<'a, K: 'a+serde::Serialize + serde::de::Deserialize<'a> + std::fmt::Debug>(t: Opaque<K>) {
    println!("{:?}", t);
    let b = bincode::serialize(&t).unwrap();
    println!("{:?}", b);
    //let b = [0,1,2];
    blu::<Opaque<K>>(&b);

}

fn blu<'a, K: serde::de::Deserialize<'a> + std::fmt::Debug>(v: &'a[u8]) {
    let o: K = bincode::deserialize(&v).unwrap();
    println!("{:?}", o);
}
*/

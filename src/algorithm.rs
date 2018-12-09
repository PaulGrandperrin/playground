use std::collections::BTreeMap;
use std::collections::LinkedList;

use itertools::Itertools;

use crate::non_volatile::manager::NVObjectManager;
use crate::non_volatile::object::object_pointer::ObjectPointer;
use crate::non_volatile::object::object_type::ObjectType;
use crate::non_volatile::object::tree::InternalNode;
use crate::non_volatile::object::tree::LeafNode;
use crate::non_volatile::object::tree::NodeEntry;

pub mod b_epsilon_tree {
    use super::*;

    const B: usize = 5;

    pub fn merge_tree(
        buffer: BTreeMap<u64, u64>,
        nv_obj_mngr: &mut NVObjectManager,
        op: &ObjectPointer,
    ) -> ObjectPointer {
        let mut new_leafs_ops = merge_rec(buffer, nv_obj_mngr, op);

        if new_leafs_ops.len() == 1 {
            new_leafs_ops.pop_back().unwrap().value // garanted to succeed
        } else {
            // we need to create a new InternalNode
            let entries = new_leafs_ops.into_iter().collect();
            let inter_node = InternalNode::from(entries); // TODO maybe change type of Node entries to LinkedList
            let op = nv_obj_mngr.store(inter_node);
            op
        }
    }

    pub fn merge_rec(
        buffer: BTreeMap<u64, u64>,
        nv_obj_mngr: &mut NVObjectManager,
        node_op: &ObjectPointer,
    ) -> LinkedList<NodeEntry<u64, ObjectPointer>> {
        match node_op.object_type {
            // we point to a leaf
            ObjectType::LeafNode => {
                // get the leaf
                let leaf = nv_obj_mngr.get::<LeafNode<u64, u64>>(node_op); // TODO: if the leaf was not in the cache before, we could directly get the owned version as we're going to modify it anyway.

                // prepare an iterator representing the view of the sorted merging
                // of the leaf's entries and the buffer of operations
                let it_leaf = leaf.entries.iter().cloned(); // we clone because leaf is RO because it can be cached
                let it_buffer = buffer.into_iter().map(|(k, v)| NodeEntry::new(k, v));
                let it_entries = it_leaf.merge_by(it_buffer, |a, b| a.key <= b.key);

                // split those entries in chunks of B entries,
                // one chunk for each resulting leaf
                let chunks = it_entries.chunks(B);

                // list of resulting object pointers to leafs
                let mut new_leafs_ops = LinkedList::new();

                // one chunk for each leaf
                for chunk in chunks.into_iter() {
                    let entries: Vec<_> = chunk.collect(); // TODO why type inference is not working? wait for chalk
                    let key = entries.first().unwrap().key.clone(); // FIXME we can crash here, but let the fuzzer find it later
                    let new_leaf = LeafNode::from(entries);

                    // write new leaf to nv device
                    let op = nv_obj_mngr.store(new_leaf);
                    new_leafs_ops.push_back(NodeEntry::new(key, op));
                }

                new_leafs_ops
            }
            _ => unimplemented!("merge in InternalNode"),
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

use std::collections::BTreeMap;
use std::collections::LinkedList;

use itertools::Itertools;

use crate::non_volatile::manager::NVObjectManager;
use crate::non_volatile::object::object_pointer::ObjectPointer;
use crate::non_volatile::object::object_type::ObjectType;
use crate::non_volatile::object::tree::InternalNode;
use crate::non_volatile::object::tree::LeafNode;
use crate::non_volatile::object::tree::NodeEntry;
use crate::non_volatile::object::tree::{BufferNode, Message, Insert};

pub mod b_epsilon_tree {
    use crate::non_volatile::object::any_rc_object::Object;
    use crate::common::ConstObjType;
    use crate::non_volatile::serializable::Serializable;
    use super::*;

    const B: usize = 5;

    pub fn new() -> impl Object {
        LeafNode::<u64, u64>::new()
    }

    pub fn debug(indent: usize, nv_obj_mngr: &mut NVObjectManager, node_op: &ObjectPointer) {
        match node_op.object_type {
            ObjectType::LeafNode => {
                // get the leaf
                let node = nv_obj_mngr.get::<LeafNode<u64, u64>>(node_op);
                println!("{:>6} {}Leaf[{}]", format!("@{}", node_op.offset), "  ".repeat(indent),node.entries.iter().map(|e|{format!("{}:{}", e.key, e.value)}).join(", "));
            }
            ObjectType::InternalNode => {
                // get the internal node
                let node = nv_obj_mngr.get::<InternalNode<u64>>(node_op);
                let buffer = nv_obj_mngr.get::<BufferNode<u64, u64>>(&node.buffer_ptr);
                println!("{:>6} {}Internal[{}] <- [{}]",
                    format!("@{}", node_op.offset),
                    "  ".repeat(indent),
                    node.entries.iter().map(|e|{format!("{}:@{}", e.key, e.value.offset)}).join(", "),
                    buffer.entries.iter().map(|e|{format!("{:?}", e)}).join(", "), // TODO make better
                    );

                // recusively print childs
                for c in &node.entries {
                    self::debug(indent + 1, nv_obj_mngr, &c.value);
                }
            }
            _ => unreachable!("expected a node object but got a {:?}", node_op.object_type)
        }
    }

    pub fn merge_tree(
        in_memstore: impl IntoIterator<Item=(u64, u64)>,
        nv_obj_mngr: &mut NVObjectManager,
        op: &ObjectPointer,
    ) -> ObjectPointer {
        // transform the insertion optimized memstore into a custom structure optimized for merging into the Bε-tree
        let mut memstore = Vec::new(); // TODO replace with custom ro-btree with cardinality metadata in nodes
        for (k, v) in in_memstore.into_iter() {
            memstore.push(NodeEntry::new(k, v));
        }

        let mut new_childs_ops = merge_rec(&memstore, nv_obj_mngr, op);

        while new_childs_ops.len() != 1 {
            new_childs_ops = reduce(new_childs_ops).into_iter().map(|chunk|{
                    NodeEntry{key: chunk[0].key, value: nv_obj_mngr.store(InternalNode::from(chunk))}
                }).collect()
        }

        new_childs_ops.pop_back().unwrap().value // garanted to succeed
    }

    fn merge_rec(
        memstore: &[NodeEntry<u64, u64>],
        nv_obj_mngr: &mut NVObjectManager,
        node_op: &ObjectPointer,
    ) -> LinkedList<NodeEntry<u64, ObjectPointer>> {
        println!("merging this memstore: {:?}", memstore);
        match node_op.object_type {
            ObjectType::LeafNode => {
                // get the leaf
                let leaf = nv_obj_mngr.get::<LeafNode<u64, u64>>(node_op); // TODO: if the node was not in the cache before, we could directly get the owned version as we're going to modify it anyway.

                // prepare an iterator representing the view of the sorted merging
                // of the leaf's entries and the memstore of operations
                let it_leaf = leaf.entries.iter().cloned(); // we clone because leaf is RO because it can be cached
                let it_memstore = memstore.iter().cloned(); // TODO when it'll be possible, move instead of clone
                let entries: LinkedList<_> = it_leaf.merge_by(it_memstore, |a, b| a.key <= b.key).collect();

                // 
                reduce(entries).into_iter().map(|chunk|{
                    NodeEntry{key: chunk[0].key, value: nv_obj_mngr.store(LeafNode::from(chunk))}
                }).collect()
            }
            ObjectType::InternalNode => {
                let mut new_entries = LinkedList::new();
                let mut old_entries_it = 0;
                let mut memstore_it = 0;

                // get the node
                let internal = nv_obj_mngr.get::<InternalNode<u64>>(node_op); // TODO: if the node was not in the cache before, we could directly get the owned version as we're going to modify it anyway.
                println!("read at {}: {:?}", node_op.offset, internal);

                assert!(internal.entries.len() > 0); // FIXME replace with real value when I know it
                // find branch where to insert first element of memstore
                while memstore_it < memstore.len() {
                    println!("searching for {} in {:?}", &memstore[memstore_it].key, &internal.entries[old_entries_it..]);
                    // find branch index where to insert the begining of the memstore
                    let branch_index = match internal.entries[old_entries_it..].binary_search_by_key(&memstore[memstore_it].key, |entry| entry.key) {
                        Ok(i) => i, // exact match
                        Err(0) => 0, // key is smaller than first entry FIXME: maybe that's not supposed to happen
                        Err(i) => i - 1, // match first bigger entry or end of slice
                    };

                    println!("branch_index: {}", branch_index);

                    // now we need to find how many elements of the memstore we can insert in this branch
                    // first find the bigger allow element
                    memstore_it = if let Some(NodeEntry{key: max, value: _}) = internal.entries.get(branch_index + 1) {
                        // there is another entry so our memstore needs to be right bounded
                        dbg!(memstore);
                        println!("max: {}", max);
                        match memstore.binary_search_by_key(max, |entry| entry.key) {
                            Ok(i) => i, // exact match
                            Err(0) => unreachable!(), // key is smaller than first entry
                            Err(i) => i, // match first bigger entry or end of slice
                        }
                    } else {
                        println!("selecting all memstore");
                        memstore.len()
                    };

                    // move directly skipped and untouched entries in new list
                    for i in old_entries_it..branch_index {
                        println!("moving directly to newlist: {:?}", internal.entries[i]);
                        new_entries.push_back(internal.entries[i].clone()); // TODO when it'll be possible don't clone, but move
                    }

                    // append all new entries
                    new_entries.append(&mut merge_rec(&memstore[..memstore_it], nv_obj_mngr, &internal.entries[branch_index].value));
                    old_entries_it = branch_index + 1;
                }

                 // move all remaining entries
                for i in old_entries_it..internal.entries.len() {
                    println!("moving directly to newlist: {:?}", internal.entries[i]);
                    new_entries.push_back(internal.entries[i].clone()); // TODO when it'll be possible don't clone, but move
                }


                reduce(new_entries).into_iter().map(|chunk|{
                    NodeEntry{key: chunk[0].key, value: nv_obj_mngr.store(InternalNode::from(chunk))}
                }).collect()
            },
            _ => unreachable!("expected a node object but got a {:?}", node_op.object_type)
        }
    }

    struct Repartition {
        smaller_chunk_len: usize,
        smaller_chunk_num: usize,
        bigger_chunk_len: usize,
        bigger_chunk_num: usize,
    }

    #[inline]
    fn compute_repartition(num_entries: usize, max_chunk_len: usize) -> Repartition {
        // Compute the length of newly created chunks
        // 
        // We want to minimize the number of chunks and maximize their occupancy while also
        // spreading as evenly as possible the branches between the internal nodes.
        // The consequence is that there will be at maximum 2 kind of nodes created:
        // one with a bigger size and one with a smaller size. And their sizes will differ by exactly one.
        // 
        // This might not be the best strategy, but I think it is ;-)
        // It's quite complex to explain, but in the end, the reason that makes me think
        // that it's the best stragegy is because we heavily batch inserts and we do copy-on-write.
        let total_chunk_num = num_entries / max_chunk_len + if num_entries % max_chunk_len == 0 { 0 } else { 1 };
        let bigger_chunk_len = num_entries / total_chunk_num + if num_entries % total_chunk_num == 0 { 0 } else { 1 };
        let smaller_chunk_len = bigger_chunk_len - 1;
        let smaller_chunk_num = bigger_chunk_len * total_chunk_num - num_entries;
        let bigger_chunk_num = (num_entries - smaller_chunk_len * smaller_chunk_num) / bigger_chunk_len;

        Repartition {
            smaller_chunk_len,
            smaller_chunk_num,
            bigger_chunk_len,
            bigger_chunk_num,
        }
    }

    fn reduce<K: Serializable + Copy, V: Serializable>(new_entries: LinkedList<NodeEntry<K,V>>)
    -> LinkedList<Vec<NodeEntry<K, V>>> {
        let rep = compute_repartition(new_entries.len(), B);

        let mut result = LinkedList::new();
        let mut new_entries_it = new_entries.into_iter();

        for (chunk_len, chunk_num) in &[(rep.bigger_chunk_len, rep.bigger_chunk_num),(rep.smaller_chunk_len, rep.smaller_chunk_num)] {
            for _ in 0..*chunk_num {
                let mut chunked_entries = Vec::new();
                for i in 0..*chunk_len {
                    if let Some(e) = new_entries_it.next() {
                        chunked_entries.push(e);
                    } else {
                        #[cfg(debug_assertions)]
                        unreachable!();
                        #[cfg(not(debug_assertions))]
                        unsafe {std::hint::unreachable_unchecked()}; // YOLO
                    }
                }

                let key = chunked_entries[0].key; // TODO do not copy
                result.push_back(chunked_entries);
            }
        }
        
        result
    }

    pub fn merge_tree_epsilon(
        in_memstore: impl IntoIterator<Item=(u64, Message<u64>)>,
        nv_obj_mngr: &mut NVObjectManager,
        op: &ObjectPointer,
    ) -> ObjectPointer {
        // transform the insertion optimized memstore into a custom structure optimized for merging into the Bε-tree
        let mut memstore = Vec::new(); // TODO replace with custom ro-btree with cardinality metadata in nodes
        for msg in in_memstore.into_iter() {
            memstore.push(NodeEntry::new(msg.0, msg.1));
        }

        let mut new_childs_ops = merge_rec_epsilon(&memstore, nv_obj_mngr, op);

        while new_childs_ops.len() != 1 {
            new_childs_ops = reduce(new_childs_ops).into_iter().map(|chunk|{
                    NodeEntry{key: chunk[0].key, value: nv_obj_mngr.store(InternalNode::from(chunk))}
                }).collect()
        }

        new_childs_ops.pop_back().unwrap().value // garanted to succeed
    }


    fn merge_rec_epsilon(
        memstore: &[NodeEntry<u64, Message<u64>>],
        nv_obj_mngr: &mut NVObjectManager,
        node_op: &ObjectPointer,
    ) -> LinkedList<NodeEntry<u64, ObjectPointer>> {
        println!("merging this memstore: {:?}", memstore);
        match node_op.object_type {
            ObjectType::LeafNode => {
                // get the leaf
                let leaf = nv_obj_mngr.get::<LeafNode<u64, u64>>(node_op); // TODO: if the node was not in the cache before, we could directly get the owned version as we're going to modify it anyway.

                // prepare an iterator representing the view of the sorted merging
                // of the leaf's entries and the memstore of operations
                let it_leaf = leaf.entries.iter().cloned(); // we clone because leaf is RO because it can be cached
                let it_memstore = memstore.iter().cloned(); // TODO when it'll be possible, move instead of clone
                let it_memstore = it_memstore.map(|msg| {
                    match msg.value {
                        Message::Insert(i) => {
                            NodeEntry::new(msg.key, i.value)
                        }
                    }
                });
                let entries: LinkedList<_> = it_leaf.merge_by(it_memstore, |a, b| a.key <= b.key).collect();

                // 
                reduce(entries).into_iter().map(|chunk|{
                    NodeEntry{key: chunk[0].key, value: nv_obj_mngr.store(LeafNode::from(chunk))}
                }).collect()
            }
            ObjectType::InternalNode => {

                let mut new_entries = LinkedList::new();
                let mut old_entries_it = 0;
                let mut memstore_it = 0;

                // get the node
                let internal = nv_obj_mngr.get::<InternalNode<u64>>(node_op); // TODO: if the node was not in the cache before, we could directly get the owned version as we're going to modify it anyway.
                println!("read at {}: {:?}", node_op.offset, internal);
                assert!(internal.entries.len() > 0); // FIXME replace with real value when I know it

                // get the buffer
                let buffer_node = nv_obj_mngr.get::<BufferNode<u64, u64>>(&internal.buffer_ptr);

                unimplemented!();

                // find branch where to insert first element of memstore
                while memstore_it < memstore.len() {
                    /*println!("searching for {} in {:?}", &memstore[memstore_it].key, &internal.entries[old_entries_it..]);
                    // find branch index where to insert the begining of the memstore
                    let branch_index = match internal.entries[old_entries_it..].binary_search_by_key(&memstore[memstore_it].key, |entry| entry.key) {
                        Ok(i) => i, // exact match
                        Err(0) => 0, // key is smaller than first entry FIXME: maybe that's not supposed to happen
                        Err(i) => i - 1, // match first bigger entry or end of slice
                    };

                    println!("branch_index: {}", branch_index);

                    // now we need to find how many elements of the memstore we can insert in this branch
                    // first find the bigger allow element
                    memstore_it = if let Some(NodeEntry{key: max, value: _}) = internal.entries.get(branch_index + 1) {
                        // there is another entry so our memstore needs to be right bounded
                        dbg!(memstore);
                        println!("max: {}", max);
                        match memstore.binary_search_by_key(max, |entry| entry.key) {
                            Ok(i) => i, // exact match
                            Err(0) => unreachable!(), // key is smaller than first entry
                            Err(i) => i, // match first bigger entry or end of slice
                        }
                    } else {
                        println!("selecting all memstore");
                        memstore.len()
                    };

                    // move directly skipped and untouched entries in new list
                    for i in old_entries_it..branch_index {
                        println!("moving directly to newlist: {:?}", internal.entries[i]);
                        new_entries.push_back(internal.entries[i].clone()); // TODO when it'll be possible don't clone, but move
                    }

                    // append all new entries
                    new_entries.append(&mut merge_rec(&memstore[..memstore_it], nv_obj_mngr, &internal.entries[branch_index].value));
                    old_entries_it = branch_index + 1;
                    */
                }

                 // move all remaining entries
                for i in old_entries_it..internal.entries.len() {
                    println!("moving directly to newlist: {:?}", internal.entries[i]);
                    new_entries.push_back(internal.entries[i].clone()); // TODO when it'll be possible don't clone, but move
                }


                reduce(new_entries).into_iter().map(|chunk|{
                    NodeEntry{key: chunk[0].key, value: nv_obj_mngr.store(InternalNode::from(chunk))}
                }).collect()
            },
            _ => unreachable!("expected a node object but got a {:?}", node_op.object_type)
        }
    }

}

/*
pub fn merge(memstore: &BTreeMap<u64, u64>, trp: &ObjectPointer, sm: &mut SpaceManager)  {
    let any_node = sm.retrieve::<AnyNode<u64,u64>>(trp);
    match any_node {
        AnyNode::LeafNode(node) => {
            unimplemented!()

            //let it_buf = memstore.into_iter();
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

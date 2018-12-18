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

    pub fn debug(indent: usize, nv_obj_mngr: &mut NVObjectManager, node_op: &ObjectPointer) {
        match node_op.object_type {
            ObjectType::LeafNode => {
                // get the leaf
                let node = nv_obj_mngr.get::<LeafNode<u64, u64>>(node_op);
                println!("@{:<6}{}Leaf[{}]", node_op.offset, "  ".repeat(indent),node.entries.iter().map(|e|{format!("{}=>{}", e.key, e.value)}).join(", "));
            }
            ObjectType::InternalNode => {
                // get the internal node
                let node = nv_obj_mngr.get::<InternalNode<u64>>(node_op);
                println!("@{:<6}{}Internal[{}]", node_op.offset, "  ".repeat(indent), node.entries.iter().map(|e|{format!("{}=>@{}", e.key, e.value.offset)}).join(", "));

                // recusively print childs
                for c in &node.entries {
                    self::debug(indent + 1, nv_obj_mngr, &c.value);
                }
            }
            _ => unreachable!("expected a node object but got a {:?}", node_op.object_type)
        }
    }

    pub fn merge_tree(
        in_buffer: impl IntoIterator<Item=(u64, u64)>,
        nv_obj_mngr: &mut NVObjectManager,
        op: &ObjectPointer,
    ) -> ObjectPointer {
        // transform the insertion optimized buffer into a custom structure optimized for merging into the Bε-tree
        let mut buffer = Vec::new(); // TODO replace with custom ro-btree with cardinality metadata in nodes
        for (k, v) in in_buffer.into_iter() {
            buffer.push(NodeEntry::new(k, v));
        }

        let mut new_leafs_ops = merge_rec(&buffer, nv_obj_mngr, op);

        if new_leafs_ops.len() == 1 {
            new_leafs_ops.pop_back().unwrap().value // garanted to succeed
        } else {
            // we need to create a new InternalNode
            if new_leafs_ops.len() > B {
                unimplemented!("recursive tree root construction") // TODO implement
            }
            let entries = new_leafs_ops.into_iter().collect();
            let inter_node = InternalNode::from(entries); // TODO maybe change type of Node entries to LinkedList
            let op = nv_obj_mngr.store(inter_node);
            op
        }
    }

    fn merge_rec(
        buffer: &[NodeEntry<u64, u64>],
        nv_obj_mngr: &mut NVObjectManager,
        node_op: &ObjectPointer,
    ) -> LinkedList<NodeEntry<u64, ObjectPointer>> {
        println!("merging this buffer: {:?}", buffer);
        match node_op.object_type {
            ObjectType::LeafNode => {
                // get the leaf
                let leaf = nv_obj_mngr.get::<LeafNode<u64, u64>>(node_op); // TODO: if the node was not in the cache before, we could directly get the owned version as we're going to modify it anyway.

                // prepare an iterator representing the view of the sorted merging
                // of the leaf's entries and the buffer of operations
                let it_leaf = leaf.entries.iter().cloned(); // we clone because leaf is RO because it can be cached
                let it_buffer = buffer.iter().cloned(); // TODO when it'll be possible, move instead of clone
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
                    println!("writing leaf: {:?}", new_leaf);
                    let op = nv_obj_mngr.store(new_leaf);
                    new_leafs_ops.push_back(NodeEntry::new(key, op));
                }

                new_leafs_ops
            }
            ObjectType::InternalNode => {
                let mut new_entries = LinkedList::new();
                let mut old_entries_it = 0;
                let mut buffer_it = 0;

                // get the node
                let internal = nv_obj_mngr.get::<InternalNode<u64>>(node_op); // TODO: if the node was not in the cache before, we could directly get the owned version as we're going to modify it anyway.
                println!("read at {}: {:?}", node_op.offset, internal);

                assert!(internal.entries.len() > 0); // FIXME replace with real value when I know it
                // find branch where to insert first element of buffer
                while buffer_it < buffer.len() {
                    println!("searching for {} in {:?}", &buffer[buffer_it].key, &internal.entries[old_entries_it..]);
                    // find branch index where to insert the begining of the buffer
                    let branch_index = match internal.entries[old_entries_it..].binary_search_by_key(&buffer[buffer_it].key, |entry| entry.key) {
                        Ok(i) => i, // exact match
                        Err(0) => 0, // key is smaller than first entry FIXME: maybe that's not supposed to happen
                        Err(i) => i - 1, // match first bigger entry or end of slice
                    };

                    println!("branch_index: {}", branch_index);

                    // now we need to find how many elements of the buffer we can insert in this branch
                    // first find the bigger allow element
                    buffer_it = if let Some(NodeEntry{key: max, value: _}) = internal.entries.get(branch_index + 1) {
                        // there is another entry so our buffer needs to be right bounded
                        dbg!(buffer);
                        println!("max: {}", max);
                        match buffer.binary_search_by_key(max, |entry| entry.key) {
                            Ok(i) => i, // exact match
                            Err(0) => unreachable!(), // key is smaller than first entry
                            Err(i) => i, // match first bigger entry or end of slice
                        }
                    } else {
                        println!("selecting all buffer");
                        buffer.len()
                    };

                    // move directly skipped and untouched entries in new list
                    for i in old_entries_it..branch_index {
                        println!("moving directly to newlist: {:?}", internal.entries[i]);
                        new_entries.push_back(internal.entries[i].clone()); // TODO when it'll be possible don't clone, but move
                    }

                    // append all new entries
                    new_entries.append(&mut merge_rec(&buffer[..buffer_it], nv_obj_mngr, &internal.entries[branch_index].value));
                    old_entries_it = branch_index + 1;
                }

                 // move all remaining entries
                for i in old_entries_it..internal.entries.len() {
                    println!("moving directly to newlist: {:?}", internal.entries[i]);
                    new_entries.push_back(internal.entries[i].clone()); // TODO when it'll be possible don't clone, but move
                }

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
                let num_entries = new_entries.len() as u64;
                let total_chunk_num = (num_entries as f64 / B as f64).ceil() as u64;
                let smaller_chunk_len = num_entries / total_chunk_num;
                let bigger_chunk_num = num_entries % smaller_chunk_len;
                let bigger_chunk_len = smaller_chunk_len + 1;
                let smaller_chunk_num =  (num_entries - bigger_chunk_len * bigger_chunk_num) / smaller_chunk_len;

                let mut new_entries_it = new_entries.into_iter();
                let mut new_nodes_ops = LinkedList::new();

                for (chunk_len, chunk_num) in &[(bigger_chunk_len, bigger_chunk_num),(smaller_chunk_len, smaller_chunk_num)] {
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

                        let key = chunked_entries[0].key;
                        let new_internal_node = InternalNode::from(chunked_entries);
                        
                        // write node
                        let op = nv_obj_mngr.store(new_internal_node);
                        new_nodes_ops.push_back(NodeEntry::new(key, op));
                    }
                }
                
                new_nodes_ops
            },
            _ => unreachable!("expected a node object but got a {:?}", node_op.object_type)
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

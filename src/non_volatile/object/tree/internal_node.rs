use super::*;

#[derive(Debug)]
pub struct InternalNode<K> {
    entries: Vec<NodeEntry<K, ObjectPointer>>,
}

impl<K> RawTyped for InternalNode<K> {
    const RAW_TYPE: ObjectType = ObjectType::InternalNode;
}

impl<K: KeyTraits> InternalNode<K> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn insert_local(&mut self, entry: NodeEntry<K, ObjectPointer>) -> Option<ObjectPointer> {
        // algo invariant: the entries should be sorted
        debug_assert!(is_sorted(self.entries.iter().map(|l| l.key)));

        let res = self.entries.binary_search_by_key(&entry.key, |e| e.key);
        match res {
            Ok(i) => unreachable!(
                "cow_btree: trying to insert in an InternalNode but key already exists"
            ),
            Err(i) => {
                self.entries.insert(i, entry);
                None
            }
        }
    }
}

use super::*;

use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};

//pub type InternalNode<K> = Node<K, ObjectPointer, InternalType>;

#[derive(Debug, Clone)]
pub struct InternalNode<K> {
    pub entries: Vec<NodeEntry<K, ObjectPointer>>,
}

impl<K> ConstObjType for InternalNode<K> {
    const OBJ_TYPE: ObjectType = ObjectType::InternalNode;
}

impl<K: serde::ser::Serialize> Serialize for InternalNode<K> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("InternalNode", 1)?;
        s.serialize_field("entries", &self.entries)?;
        s.end()
    }
}

impl<'de, K: serde::de::DeserializeOwned> Deserialize<'de>
    for InternalNode<K>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NodeVisitor<K> {
            _p: std::marker::PhantomData<(K)>,
        }

        impl<'de, K: serde::de::DeserializeOwned> Visitor<'de>
            for NodeVisitor<K>
        {
            type Value = InternalNode<K>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an Node")
            }

            fn visit_seq<S>(self, mut seq: S) -> Result<InternalNode<K>, S::Error>
            where
                S: SeqAccess<'de>,
            {
                let entries = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                Ok(InternalNode {
                    entries,
                })
            }
        }

        const FIELDS: &[&str] = &["entries"];
        deserializer.deserialize_struct(
            "InternalNode",
            FIELDS,
            NodeVisitor {
                _p: std::marker::PhantomData,
            },
        )
    }
}

impl<K> InternalNode<K> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn from(entries: Vec<NodeEntry<K, ObjectPointer>>) -> Self {
        Self {
            entries,
        }
    }

    // TODO delete
    pub fn insert_local(&mut self, mut entry: NodeEntry<K, ObjectPointer>) -> Option<ObjectPointer>
    where
        K: Ord + Copy,
    {
        // algo invariant: the entries should be sorted
        debug_assert!(is_sorted(self.entries.iter().map(|l| l.key)));

        let res = self.entries.binary_search_by_key(&entry.key, |e| e.key);
        match res {
            Ok(i) => {
                mem::swap(&mut self.entries[i].value, &mut entry.value);
                Some(entry.value)
            }
            Err(i) => {
                self.entries.insert(i, entry);
                None
            }
        }
    }
}

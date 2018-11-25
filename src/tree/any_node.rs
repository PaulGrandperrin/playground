use crate::tree::leaf_node::LeafNode;
use crate::tree::internal_node::InternalNode;
use crate::object_type::ObjectType;

use std::fmt;

use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess};
use serde::ser::{Serialize, Serializer, SerializeStruct};


#[derive(Debug)]
pub enum AnyNode<K, V> {
    LeafNode(LeafNode<K, V>),
    InternalNode(InternalNode<K>),
}

/*
impl<K,V> AnyNode<K, V> {
    fn get_object_type(&self) -> ObjectType {
        match self {
            AnyNode::LeafNode(_) => ObjectType::LeafNode,
            AnyNode::InternalNode(_) => ObjectType::InternalNode,
        }
    }
}
*/

impl<K: Copy + Ord + Serialize, V: Serialize> Serialize for AnyNode<K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        let mut s = serializer.serialize_struct("AnyNode", 2)?;
        match self {
            AnyNode::LeafNode(n) => {
                s.serialize_field("object_type", &ObjectType::LeafNode)?;
                s.serialize_field("leaf_node", n)?;
            }
            AnyNode::InternalNode(n) => {
                unimplemented!()
            }
        }
        s.end()
    }
}

impl<'de, K: serde::de::DeserializeOwned, V: serde::de::DeserializeOwned> Deserialize<'de> for AnyNode<K, V> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>,
    {
        
        struct AnyNodeVisitor<K, V> {
            _p: std::marker::PhantomData<(K, V)>
        }

        impl<'de, K: serde::de::DeserializeOwned, V: serde::de::DeserializeOwned> Visitor<'de> for AnyNodeVisitor<K, V> {
            type Value = AnyNode<K, V>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an AnyNode")
            }

            fn visit_seq<S>(self, mut seq: S) -> Result<AnyNode<K,V>, S::Error>
            where S: SeqAccess<'de>,
            {
                
                let object_type: ObjectType = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                match object_type {
                    ObjectType::LeafNode => {
                        Ok(AnyNode::LeafNode(seq.next_element()?
                            .ok_or_else(|| de::Error::invalid_length(1, &self))?))
                    },
                    ObjectType::InternalNode => {
                        unimplemented!()
                    },
                    _ => {
                        Err(de::Error::custom(format!("expected LeafNode or InternalNode but got {:?}", object_type)))
                    }
                }


            }
        }

        const FIELDS: &[&str] = &["object_type", "leaf node", "internal_node"];
        deserializer.deserialize_struct("AnyNode", FIELDS, AnyNodeVisitor{_p:std::marker::PhantomData})
    }
}
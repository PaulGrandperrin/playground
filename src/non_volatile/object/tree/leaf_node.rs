use super::super::object_type::ObjectType;
use super::*;

use std::fmt;
use std::mem;

use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub type LeafNode<K, V> = Node<K, V, LeafType>;




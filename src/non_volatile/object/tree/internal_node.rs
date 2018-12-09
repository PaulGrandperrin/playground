use super::*;

use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub type InternalNode<K> = Node<K, ObjectPointer, InternalType>;

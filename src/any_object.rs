use crate::uberblock::Uberblock;
use crate::tree::{LeafNode, InternalNode};
use crate::serializable::Serializable;
use crate::common::RawTyped;
use std::rc::Rc;

#[derive(Debug)]
pub enum AnyObject {
    Uberblock(Rc<Uberblock>),
    LeafNode(Rc<LeafNode<u64, u64>>),
    InternalNode(Rc<InternalNode<u64>>),
}

impl AnyObject {
    //fn into_object() -> 
}

// TODO SIMPLIFY THIS CRAZY SHIT

pub trait Object: Serializable + RawTyped
where AnyObject: From<Rc<Self>> { // TODO also add RawSized but as an Option<u64>
    fn into_any(this: impl Into<Rc<Self>>) -> AnyObject;
}

impl<T: Serializable + RawTyped> Object for T where AnyObject: From<Rc<Self>> {
    fn into_any(this: impl Into<Rc<Self>>) -> AnyObject {
        this.into().into()
    }
}

impl From<Rc<Uberblock>> for AnyObject {
    fn from(this: Rc<Uberblock>) -> AnyObject {
        AnyObject::Uberblock(this)
    }
}

impl From<Rc<LeafNode<u64, u64>>> for AnyObject {
    fn from(this: Rc<LeafNode<u64, u64>>) -> AnyObject {
        AnyObject::LeafNode(this)
    }
}

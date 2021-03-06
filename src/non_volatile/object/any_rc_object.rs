use super::super::serializable::Serializable;
use super::tree::{InternalNode, LeafNode, BufferNode};
use super::uberblock::Uberblock;
use crate::common::ConstObjType;
use failure::format_err;
use std::borrow::Borrow;
use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum AnyRcObject {
    Uberblock(Rc<Uberblock>),
    LeafNode(Rc<LeafNode<u64, u64>>),
    InternalNode(Rc<InternalNode<u64>>),
    BufferNode(Rc<BufferNode<u64, u64>>),
}

impl AnyRcObject {
    //fn into_object() ->
}

// TODO SIMPLIFY THIS CRAZY SHIT

/*
trait FooBar: Foo + Bar {}
impl<T: Foo + Bar> FooBar for T {}

*/

pub trait Object = Serializable + ConstObjType + Default where AnyRcObject: From<Self>,
        AnyRcObject: From<Rc<Self>>,
        Rc<Self>: TryFrom<AnyRcObject>,
        <Rc<Self> as TryFrom<AnyRcObject>>::Error: Debug;

/*
pub trait Object: Serializable + RawTyped
where AnyRcObject: From<Rc<Self>> { // TODO also add RawSized but as an Option<u64>
    fn into_any(this: impl Into<Rc<Self>>) -> AnyRcObject;
}

impl<T: Serializable + RawTyped> Object for T where AnyRcObject: From<Rc<Self>> {
    fn into_any(this: impl Into<Rc<Self>>) -> AnyRcObject {
        this.into().into()
    }
}
*/

impl From<Uberblock> for AnyRcObject {
    fn from(this: Uberblock) -> Self {
        AnyRcObject::Uberblock(Rc::new(this))
    }
}

impl From<Rc<Uberblock>> for AnyRcObject {
    fn from(this: Rc<Uberblock>) -> Self {
        AnyRcObject::Uberblock(this)
    }
}

impl TryFrom<AnyRcObject> for Rc<Uberblock> {
    type Error = failure::Error;

    fn try_from(this: AnyRcObject) -> Result<Self, Self::Error> {
        match this {
            AnyRcObject::Uberblock(u) => Ok(u),
            _ => {
                Err(format_err!("Cannot convert this AnyNode to an Uberblock")) // TODO better error message with trait
            }
        }
    }
}

impl From<LeafNode<u64, u64>> for AnyRcObject {
    fn from(this: LeafNode<u64, u64>) -> Self {
        AnyRcObject::LeafNode(Rc::new(this))
    }
}

impl From<Rc<LeafNode<u64, u64>>> for AnyRcObject {
    fn from(this: Rc<LeafNode<u64, u64>>) -> Self {
        AnyRcObject::LeafNode(this)
    }
}

impl TryFrom<AnyRcObject> for Rc<LeafNode<u64, u64>> {
    type Error = failure::Error;

    fn try_from(this: AnyRcObject) -> Result<Self, Self::Error> {
        match this {
            AnyRcObject::LeafNode(n) => Ok(n),
            _ => {
                Err(format_err!("Cannot convert this AnyNode to a LeafNode")) // TODO better error message with trait
            }
        }
    }
}

impl From<InternalNode<u64>> for AnyRcObject {
    fn from(this: InternalNode<u64>) -> Self {
        AnyRcObject::InternalNode(Rc::new(this))
    }
}

impl From<Rc<InternalNode<u64>>> for AnyRcObject {
    fn from(this: Rc<InternalNode<u64>>) -> Self {
        AnyRcObject::InternalNode(this)
    }
}

impl TryFrom<AnyRcObject> for Rc<InternalNode<u64>> {
    type Error = failure::Error;

    fn try_from(this: AnyRcObject) -> Result<Self, Self::Error> {
        match this {
            AnyRcObject::InternalNode(n) => Ok(n),
            _ => {
                Err(format_err!("Cannot convert this AnyNode to a LeafNode")) // TODO better error message with trait
            }
        }
    }
}

impl From<BufferNode<u64, u64>> for AnyRcObject {
    fn from(this: BufferNode<u64, u64>) -> Self {
        AnyRcObject::BufferNode(Rc::new(this))
    }
}

impl From<Rc<BufferNode<u64, u64>>> for AnyRcObject {
    fn from(this: Rc<BufferNode<u64, u64>>) -> Self {
        AnyRcObject::BufferNode(this)
    }
}

impl TryFrom<AnyRcObject> for Rc<BufferNode<u64, u64>> {
    type Error = failure::Error;

    fn try_from(this: AnyRcObject) -> Result<Self, Self::Error> {
        match this {
            AnyRcObject::BufferNode(n) => Ok(n),
            _ => {
                Err(format_err!("Cannot convert this AnyNode to a BufferNode")) // TODO better error message with trait
            }
        }
    }
}
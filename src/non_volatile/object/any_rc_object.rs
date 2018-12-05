use super::super::serializable::Serializable;
use super::tree::{InternalNode, LeafNode};
use super::uberblock::Uberblock;
use crate::common::RawTyped;
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
}

impl AnyRcObject {
    //fn into_object() ->
}

// TODO SIMPLIFY THIS CRAZY SHIT

/*
trait FooBar: Foo + Bar {}
impl<T: Foo + Bar> FooBar for T {}

*/

trait Object = Serializable + RawTyped;

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

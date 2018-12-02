use crate::uberblock::Uberblock;
use crate::tree::{LeafNode, InternalNode};
use crate::serializable::Serializable;
use crate::common::RawTyped;
use std::rc::Rc;
use std::convert::{TryFrom,TryInto};
use failure::format_err;
use std::fmt::Debug;
use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub enum AnyObject {
    Uberblock(Rc<Uberblock>),
    LeafNode(Rc<LeafNode<u64, u64>>),
    InternalNode(Rc<InternalNode<u64>>),
}

impl AnyObject {
    //fn into_object() -> 
}

// TODO SIMPLIFY THIS CRAZY SHIT

/*
trait FooBar: Foo + Bar {}
impl<T: Foo + Bar> FooBar for T {}

*/

pub trait Object = Serializable + RawTyped
where   AnyObject: From<Self>,
        AnyObject: From<Rc<Self>>,
        Rc<Self>: TryFrom<AnyObject>,
        <Rc<Self> as TryFrom<AnyObject>>::Error: Debug;

/*
pub trait Object: Serializable + RawTyped
where AnyObject: From<Rc<Self>> { // TODO also add RawSized but as an Option<u64>
    fn into_any(this: impl Into<Rc<Self>>) -> AnyObject;
}

impl<T: Serializable + RawTyped> Object for T where AnyObject: From<Rc<Self>> {
    fn into_any(this: impl Into<Rc<Self>>) -> AnyObject {
        this.into().into()
    }
}
*/

impl From<Uberblock> for AnyObject {
    fn from(this: Uberblock) -> Self {
        AnyObject::Uberblock(Rc::new(this))
    }
}

impl From<Rc<Uberblock>> for AnyObject {
    fn from(this: Rc<Uberblock>) -> Self {
        AnyObject::Uberblock(this)
    }
}

impl TryFrom<AnyObject> for Rc<Uberblock> {
    type Error = failure::Error;

    fn try_from(this: AnyObject) -> Result<Self, Self::Error> {
        match this {
            AnyObject::Uberblock(u) => Ok(u),
            _ => {
                Err(format_err!("Cannot convert this AnyNode to an Uberblock")) // TODO better error message with trait
            }
        }
    }
}

impl From<LeafNode<u64, u64>> for AnyObject {
    fn from(this: LeafNode<u64, u64>) -> Self {
        AnyObject::LeafNode(Rc::new(this))
    }
}

impl From<Rc<LeafNode<u64, u64>>> for AnyObject {
    fn from(this: Rc<LeafNode<u64, u64>>) -> Self {
        AnyObject::LeafNode(this)
    }
}

impl TryFrom<AnyObject> for Rc<LeafNode<u64, u64>> {
    type Error = failure::Error;

    fn try_from(this: AnyObject) -> Result<Self, Self::Error> {
        match this {
            AnyObject::LeafNode(n) => Ok(n),
            _ => {
                Err(format_err!("Cannot convert this AnyNode to a LeafNode")) // TODO better error message with trait
            }
        }
    }
}
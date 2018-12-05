//#![allow(clippy::int_plus_one)]

use super::object_type::ObjectType;
use std::mem::size_of;

use bytes::{Buf, BufMut};
use std::io::Cursor;

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ObjectPointer {
    // TODO => rename ExtendPointer
    pub offset: u64,
    pub len: u64,
    object_type: ObjectType,
    // checksum
}

/*
enum ObjectPointer {
    Committed(ExtendPointer),
    Pending(WeakPointer), // or some ID, with TXG
}

*/

impl ObjectPointer {
    pub const RAW_SIZE: usize = size_of::<u64>() * 2 + ObjectType::RAW_SIZE;

    pub fn new(offset: u64, len: u64, object_type: ObjectType) -> ObjectPointer {
        ObjectPointer {
            offset,
            len,
            object_type,
        }
    }
}

impl crate::common::RawSized for ObjectPointer {
    const RAW_SIZE: usize = Self::RAW_SIZE;
}

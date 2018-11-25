//#![allow(clippy::int_plus_one)]

//use super::object_type::ObjectType;

use std::io::Cursor;
use bytes::{Buf, BufMut};

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ObjectPointer { // => rename ExtendPointer
    pub offset: u64,
    pub len: u64,

    //object_type: ObjectType
    // checksum
}

/*
enum ObjectPointer {
    Committed(ExtendPointer),
    Pending(WeakPointer), // or some ID, with TXG
}


*/

impl ObjectPointer {
    pub const RAW_SIZE: usize = 8 + 8;// + super::ObjectType::RAW_SIZE;

    pub fn new(offset: u64, len: u64/*, object_type: ObjectType*/) -> ObjectPointer {
        ObjectPointer {
            offset,
            len,
            //object_type,
        }
    }

    pub fn from_bytes(bytes: &mut Cursor<&[u8]>) -> Result<ObjectPointer, failure::Error> {
        assert!(bytes.remaining() >= Self::RAW_SIZE);
        
        let offset = bytes.get_u64_le();
        let len = bytes.get_u64_le();
        //let object_type = ObjectType::from_u8(bytes.get_u8());

        Ok(
            ObjectPointer {
                offset,
                len,
                //object_type,
            }
        )
    }

    pub fn to_bytes(&self, bytes: &mut Cursor<&mut [u8]>) {
        assert!(bytes.remaining_mut() >= Self::RAW_SIZE);
        
        bytes.put_u64_le(self.offset);
        bytes.put_u64_le(self.len);
        //bytes.put_u8(self.object_type.to_u8()); // there is less than 2^8 types
    }

    pub fn to_mem(&self) -> Box<[u8]> {
        let size = Self::RAW_SIZE;
        let mut mem = Vec::with_capacity(size);
        unsafe{mem.set_len(size)};
        self.to_bytes(&mut Cursor::new(&mut mem));
        
        mem.into_boxed_slice()
    }
}

impl crate::common::RawSized for ObjectPointer {
    const RAW_SIZE: usize = Self::RAW_SIZE;
}


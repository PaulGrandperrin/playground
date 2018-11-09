use std::io::Cursor;
use bytes::{Buf, BufMut};

pub trait Serializable: Sized {
    const SIZE: usize;

    fn to_bytes(&self, bytes: &mut Cursor<&mut [u8]>);
    fn from_bytes(bytes: &mut Cursor<&[u8]>) -> Result<Self, failure::Error>;
}

impl Serializable for u64 {
    const SIZE: usize = 8;

    fn to_bytes(&self, bytes: &mut Cursor<&mut [u8]>) {
        bytes.put_u64_le(*self);
    }
    fn from_bytes(bytes: &mut Cursor<&[u8]>) -> Result<Self, failure::Error> {
        Ok(bytes.get_u64_le())
    }
}

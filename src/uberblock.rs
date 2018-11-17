use super::object_pointer::ObjectPointer;
use std::io::Cursor;
use failure::format_err;
use std::mem;
use bytes::{Buf, BufMut};
use std::fmt;

const MAGIC_NUMBER: &[u8;8] = b"ReactDB0";

#[derive(Debug)]
pub struct Uberblock {
    pub tgx: u64,
    pub free_space_offset: u64,
    pub tree_root_pointer: ObjectPointer,
}

impl Uberblock {
    pub const RAW_SIZE: usize = 8 + 8 + 8 + super::ObjectPointer::RAW_SIZE;

    pub fn new(tgx: u64, tree_root_pointer: ObjectPointer, free_space_offset: u64) -> Uberblock {
        Uberblock {
            tgx,
            free_space_offset,
            tree_root_pointer,
        }
    }

    pub fn from_bytes(bytes: &mut Cursor<&[u8]>) -> Result<Uberblock, failure::Error> {
        assert!(bytes.remaining() >= Self::RAW_SIZE);

        let mut magic= [0; 8];
        bytes.copy_to_slice(&mut magic);
        if magic != *MAGIC_NUMBER {
            return Err(format_err!("Incorrect magic number. found: {:?}, expected: {:?}", magic, MAGIC_NUMBER));
        }
        let tgx = bytes.get_u64_le();
        let free_space_offset = bytes.get_u64_le();
        let tree_root_pointer = ObjectPointer::from_bytes(bytes)?;

        assert!(bytes.remaining() == 0);

        Ok(
            Uberblock {
                tgx,
                tree_root_pointer,
                free_space_offset,
            }
        )
    }

    pub fn to_bytes(&self, bytes: &mut Cursor<&mut [u8]>) {
        assert!(bytes.remaining_mut() >= 8 + 8 + 8);
        
        bytes.put_slice(MAGIC_NUMBER);
        bytes.put_u64_le(self.tgx);
        bytes.put_u64_le(self.free_space_offset);
        self.tree_root_pointer.to_bytes(bytes);
    }

    pub fn to_mem(&self) -> Box<[u8]> {
        let mut mem: Box<[u8;41]> = Box::new(unsafe{mem::uninitialized()});
        self.to_bytes(&mut Cursor::new(&mut *mem));
        mem
    }
}

impl crate::common::RawSized for Uberblock {
    const RAW_SIZE: usize = Self::RAW_SIZE;
}
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess};
use serde::ser::{Serialize, Serializer, SerializeStruct};

impl Serialize for Uberblock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        let mut s = serializer.serialize_struct("Uberblock", 4)?;
        s.serialize_field("magic_number", MAGIC_NUMBER)?;
        s.serialize_field("tgx", &self.tgx)?;
        s.serialize_field("free_space_offset", &self.free_space_offset)?;
        s.serialize_field("tree_root_pointer", &self.tree_root_pointer)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for Uberblock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>,
    {
        struct UberblockVisitor;

        impl<'de> Visitor<'de> for UberblockVisitor {
            type Value = Uberblock;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an Uberblock")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Uberblock, V::Error>
            where V: SeqAccess<'de>,
            {
                let magic: [u8; 8] = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                if &magic != MAGIC_NUMBER {
                    return Err(de::Error::custom(format!("invalid magic number: {:?}", magic)))
                }
                let tgx = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let free_space_offset = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let tree_root_pointer = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                Ok(Uberblock::new(tgx, tree_root_pointer, free_space_offset))
            }
        }

        const FIELDS: &[&str] = &["magic_number", "tgx", "tree_root_pointer", "free_space_offset"];
        deserializer.deserialize_struct("Uberblock", FIELDS, UberblockVisitor)
    }
}
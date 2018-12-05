use super::object_pointer::ObjectPointer;
use crate::common::RawTyped;
use super::object_type::ObjectType;
use bytes::{Buf, BufMut};
use failure::format_err;
use std::fmt;
use std::io::Cursor;
use std::mem;
use std::mem::size_of;

const MAGIC_NUMBER: &[u8; 8] = b"ReactDB0";

#[derive(Debug)]
pub struct Uberblock {
    pub txg: u64,
    pub fso: u64,
    pub op: ObjectPointer,
}

impl Uberblock {
    pub const RAW_SIZE: usize = size_of::<u64>() * 2 + 8 + ObjectPointer::RAW_SIZE;

    pub fn new(txg: u64, op: ObjectPointer, fso: u64) -> Uberblock {
        Uberblock { txg, fso, op }
    }
}

impl RawTyped for Uberblock {
    const RAW_TYPE: ObjectType = ObjectType::Uberblock;
}

impl crate::common::RawSized for Uberblock {
    const RAW_SIZE: usize = Self::RAW_SIZE;
}
use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};

impl Serialize for Uberblock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Uberblock", 4)?;
        s.serialize_field("magic_number", MAGIC_NUMBER)?;
        s.serialize_field("txg", &self.txg)?;
        s.serialize_field("fso", &self.fso)?;
        s.serialize_field("op", &self.op)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for Uberblock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UberblockVisitor;

        impl<'de> Visitor<'de> for UberblockVisitor {
            type Value = Uberblock;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an Uberblock")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Uberblock, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let magic: [u8; 8] = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                if &magic != MAGIC_NUMBER {
                    return Err(de::Error::custom(format!(
                        "invalid magic number: {:?}",
                        magic
                    )));
                }
                let txg = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let fso = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let op = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                Ok(Uberblock::new(txg, op, fso))
            }
        }

        const FIELDS: &[&str] = &["magic_number", "txg", "op", "fso"];
        deserializer.deserialize_struct("Uberblock", FIELDS, UberblockVisitor)
    }
}

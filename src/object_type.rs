use std::fmt;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum ObjectType {
    InternalNode = 0,
    LeafNode = 1
}

impl ObjectType {
    pub const RAW_SIZE: usize = 1;

    pub fn from_u8(n: u8) -> ObjectType {
        match n {
            0 => ObjectType::InternalNode,
            1 => ObjectType::LeafNode,
            _ => panic!("impossible ObjectType cast: {:?}", n)
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            ObjectType::InternalNode => 0,
            ObjectType::LeafNode => 1,
        }
    }
}


impl serde::Serialize for ObjectType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(self.to_u8())
    }
}

impl<'de> serde::Deserialize<'de> for ObjectType {
    fn deserialize<D>(deserializer: D) -> Result<ObjectType, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ObjectTypeVisitor;
        impl<'de> serde::de::Visitor<'de> for ObjectTypeVisitor {
            type Value = ObjectType;
        
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a byte")
            }
        
            fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ObjectType::from_u8(value))
            }
        }
        deserializer.deserialize_u8(ObjectTypeVisitor)
    }
}

impl crate::common::RawSized for ObjectType {
    const RAW_SIZE: usize = Self::RAW_SIZE;
}
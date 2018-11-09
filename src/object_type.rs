#[derive(Debug, Clone)]
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
            _ => panic!("impossible ObjectType cast")
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

impl crate::common::RawSized for ObjectType {
    const RAW_SIZE: usize = Self::RAW_SIZE;
}
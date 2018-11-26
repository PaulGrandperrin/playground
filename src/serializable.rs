pub trait Serializable: Sized {
    fn serialize(&self) -> Result<Vec<u8>, failure::Error>;
    fn deserialize(bytes: &[u8]) -> Result<Self, failure::Error>;
}

impl<T: serde::ser::Serialize + serde::de::DeserializeOwned> Serializable for T {
    fn serialize(&self) -> Result<Vec<u8>, failure::Error> {
        bincode::serialize(self).map_err(|e|{e.into()})
    }
    fn deserialize(bytes: &[u8]) -> Result<Self, failure::Error> {
        bincode::deserialize::<Self>(bytes).map_err(|e|{e.into()})
    }
}
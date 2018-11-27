use crate::object_pointer::ObjectPointer;
use crate::file_backend::FileBackend;
use crate::serializable::Serializable;
use crate::common::RawTyped;

// Kind of like ZFS's DMU (Data Management Unit)

#[derive(Debug)]
pub struct SpaceManager {
    pub block_dev: FileBackend,
    pub free_space_offset: u64,
}

impl SpaceManager {
    #[must_use]
    pub fn new(offset: u64) -> Self {
        Self {
            block_dev: FileBackend::new(),
            free_space_offset: offset,
        }
    }

    #[must_use]
    fn alloc<T: num::NumCast>(&mut self, size: T) -> u64 {
        let o = self.free_space_offset;
        self.free_space_offset += num::cast::<T, u64>(size).unwrap();
        o
    }

    #[must_use]
    pub fn store<O: Serializable + RawTyped>(&mut self, object: &O) -> ObjectPointer {
        let object_mem = object.serialize().unwrap();
        let len = object_mem.len() as u64;
        let offset = self.alloc(len);
        self.block_dev.write(offset, &object_mem);
        ObjectPointer::new(offset, len, O::RAW_TYPE)
    }

    #[must_use]
    pub fn retrieve<O: Serializable>(&mut self, op: &ObjectPointer) -> O {
        let raw = self.block_dev.read(op.offset, op.len);
        Serializable::deserialize(&raw).unwrap() // FIXME: not zero-copy
    }
}

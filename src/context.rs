use super::object_pointer::ObjectPointer;
use super::file_backend::FileBackend;
use super::uberblock::Uberblock;

pub struct Context {
    block_dev: FileBackend,
    free_space_offset: u64,
    tgx: u64,
}

impl Context {
    const NUM_UBERBLOCKS: u64 = 3;

    pub fn new() -> Context {
        Context {
            block_dev: FileBackend::new(),
            free_space_offset: Self::NUM_UBERBLOCKS * Uberblock::RAW_SIZE as u64,
            tgx: 0,
        }
    }

    pub fn alloc<T: num::NumCast>(&mut self, size: T) -> u64 {
        let o = self.free_space_offset;
        self.free_space_offset += num::cast::<T, u64>(size).unwrap();
        o
    }

    pub fn store<O>(&mut self, object: &O) -> ObjectPointer
        where O: serde::Serialize + super::common::RawTyped {
        let object_mem = bincode::serialize(&object).unwrap();
        let offset = self.alloc(object_mem.len());
        self.block_dev.write(offset, &object_mem);
        ObjectPointer::new(offset, object_mem.len() as u64, O::RAW_TYPE)
    }

    pub fn commit(&mut self, op: impl Into<ObjectPointer>) {
        let ub = Uberblock::new(self.tgx, op.into(), self.free_space_offset);
        let ub_mem = bincode::serialize(&ub).unwrap();
        let ub_offset = (self.tgx % Self::NUM_UBERBLOCKS) * Uberblock::RAW_SIZE as u64;
        self.block_dev.write(ub_offset, &ub_mem);
        self.tgx += 1;
    }
}
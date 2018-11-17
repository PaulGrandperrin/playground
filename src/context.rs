use itertools::Itertools;

use super::object_pointer::ObjectPointer;
use super::file_backend::FileBackend;
use super::uberblock::Uberblock;
use super::tree::LeafNode;

#[derive(Debug)]
pub struct Context {
    pub sm: SpaceManager,
    tgx: u64,
    pub tree_root_pointer: ObjectPointer,
}

#[derive(Debug)]
pub struct SpaceManager {
    block_dev: FileBackend,
    free_space_offset: u64,
}

impl SpaceManager {
    pub fn new() -> Self {
        Self {
            block_dev: FileBackend::new(),
            free_space_offset: Context::NUM_UBERBLOCKS * Uberblock::RAW_SIZE as u64,
        }
    }

    fn alloc<T: num::NumCast>(&mut self, size: T) -> u64 {
        let o = self.free_space_offset;
        self.free_space_offset += num::cast::<T, u64>(size).unwrap();
        o
    }

    fn store<O>(&mut self, object: &O) -> ObjectPointer
    where O: serde::Serialize + super::common::RawTyped {
        let object_mem = bincode::serialize(&object).unwrap();
        let len = object_mem.len() as u64;
        let offset = self.alloc(len);
        self.block_dev.write(offset, &object_mem);
        ObjectPointer::new(offset, len)
    }

    fn retrieve<O>(&mut self, op: &ObjectPointer) -> O
    where O: serde::de::DeserializeOwned {
        let raw = self.block_dev.read(op.offset, op.len);
        bincode::deserialize::<O>(&raw).unwrap() // FIXME: not zero-copy
    }
}

impl Context {
    const NUM_UBERBLOCKS: u64 = 3;

    pub fn format_and_load() -> Context {
        let mut sm = SpaceManager::new();
        let root_node = LeafNode::<u64, u64>::new();
        let trp = sm.store(&root_node);
        let ub = Uberblock::new(0, trp.clone(), sm.free_space_offset);
        let ub_mem = bincode::serialize(&ub).unwrap();
        sm.block_dev.write(0, &ub_mem);
        Context {
            sm,
            tgx: 1,
            tree_root_pointer: trp,
        }
    }

    pub fn load() -> Result<Context, failure::Error> {
        let mut sm = SpaceManager::new();
        let ub = (0..Self::NUM_UBERBLOCKS).map(|i| {
            Ok::<_, failure::Error>(sm.retrieve::<Uberblock>(&ObjectPointer::new(i as u64 * Uberblock::RAW_SIZE as u64, Uberblock::RAW_SIZE as u64)))
        }).fold_results(None::<Uberblock>, |acc, u| { // compute max if no error
            if let Some(acc) = acc {
                if u.tgx <= acc.tgx {
                    return Some(acc)
                }
            }
            Some(u)
        }).map(|o| {
            o.unwrap() // guaranted to succeed
        })?;

        sm.free_space_offset = ub.free_space_offset;
        
        Ok(Context{
            tgx: ub.tgx,
            sm,
            tree_root_pointer: ub.tree_root_pointer,
        })
    }
    

    pub fn commit(&mut self, op: impl Into<ObjectPointer>) {
        let ub = Uberblock::new(self.tgx, op.into(), self.sm.free_space_offset);
        let ub_mem = bincode::serialize(&ub).unwrap();
        let ub_offset = (self.tgx % Self::NUM_UBERBLOCKS) * Uberblock::RAW_SIZE as u64;
        self.sm.block_dev.write(ub_offset, &ub_mem);
        self.tgx += 1;
    }

    pub fn save<O>(&mut self, object: &O) -> ObjectPointer
    where O: serde::Serialize + super::common::RawTyped {
        // TODO: implement cache
        self.sm.store(object)
    }

    pub fn get<O>(&mut self, op: &ObjectPointer) -> O
    where O: serde::de::DeserializeOwned {
        // TODO: implement cache
        self.sm.retrieve(op)
    }
}
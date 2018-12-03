use crate::object_pointer::ObjectPointer;
use crate::file_backend::FileBackend;
use crate::serializable::Serializable;
use crate::common::RawTyped;
use crate::any_object::{AnyObject, Object};
use crate::uberblock::Uberblock;
use crate::nv_obj_cache::NVObjectCache;
use std::rc::Rc;

// Kind of like ZFS' zpool's  DMU (Data Management Unit)

/// responsible for serialization, compression, checksuming and type check and caching
#[derive(Debug)]
pub struct NVObjectManager {
    nv_blk_dev: FileBackend,
    /// transaction group number
    txg: u64,
    /// free space offset
    fso: u64,
    /// clean cache
    ccache: NVObjectCache,
}

impl NVObjectManager {
    const NUM_UBERBLOCKS: u64 = 3;

    #[must_use]
    pub fn new<O: Object>(obj: O) -> (Self, ObjectPointer) {
        // initialization
        let mut nv_blk_dev = FileBackend::new();
        let mut fso = Self::NUM_UBERBLOCKS * Uberblock::RAW_SIZE as u64;
        let txg = 1;
        let mut ccache = NVObjectCache::new();

        // convert obj to raw
        let obj_raw = Self::obj_to_raw(&obj).unwrap();
        let len = obj_raw.len() as u64;

        // write obj
        let offset = Self::alloc_impl(&mut fso, len);
        let op = Self::write_impl(&mut ccache, &mut nv_blk_dev, obj, obj_raw, offset);
        
        // create uberblock
        let ub = Uberblock::new(txg, op.clone(), fso);
        let ub_raw = ub.serialize().unwrap();

        // write uberblock copies
        for i in 0..Self::NUM_UBERBLOCKS {
            nv_blk_dev.write(i * Uberblock::RAW_SIZE as u64, &ub_raw);
        }

        // return
        (
            Self {
                nv_blk_dev: FileBackend::new(),
                txg,
                fso,
                ccache,
            },
            op
        )
    }

    #[must_use]
    pub fn load() -> (Self, ObjectPointer) {
        // initialization
        let mut nv_blk_dev = FileBackend::new();
        let ccache = NVObjectCache::new();

        // find latest uberblock
        let mut latest_txg = 0;
        let mut latest_ub = None;
        for i in 0..Self::NUM_UBERBLOCKS {
            let raw = nv_blk_dev.read(i * Uberblock::RAW_SIZE as u64, Uberblock::RAW_SIZE as u64);
            let ub: Uberblock = Serializable::deserialize(&raw).unwrap(); // FIXME: not zero-copy
            if ub.txg > latest_txg {
                latest_txg = ub.txg;
                latest_ub = Some(ub);
            }
        }
        let txg = latest_txg;
        let ub = latest_ub.unwrap();

        // return
        (
            Self {
                nv_blk_dev,
                txg,
                fso: ub.fso,
                ccache,
            },
            ub.op
        )
    }

     #[must_use]
    pub fn commit(&mut self, op: &ObjectPointer) {
        // TODO merge buffer's data into the B^ε-tree using COW

        // write new uber
        self.txg += 1;
        let ub = Uberblock::new(self.txg, op.clone(), self.fso);
        let ub_raw = ub.serialize().unwrap();
        let ub_offset = (self.txg % Self::NUM_UBERBLOCKS) * Uberblock::RAW_SIZE as u64;
        self.nv_blk_dev.write(ub_offset, &ub_raw);
    }

    // TODO harmonize trait bounds

    #[must_use]
    pub fn get<O: Object>(&mut self, op: &ObjectPointer) -> Rc<O> {
        match self.ccache.get::<O>(op) {
            Some(o) => {
                println!("cache hit :-)");
                o
            },
            None => {
                println!("cache miss :-(");
                let raw = self.nv_blk_dev.read(op.offset, op.len);
                let obj = Serializable::deserialize(&raw).unwrap(); // FIXME: not zero-copy
                Rc::new(obj)
            }
        }
    }

    #[must_use]
    pub fn store<O: Object>(&mut self, obj: O) -> ObjectPointer {
        let obj_raw = Self::obj_to_raw(&obj).unwrap();
        let len = obj_raw.len() as u64;

        let offset = self.alloc(len);
        Self::write_impl(&mut self.ccache, &mut self.nv_blk_dev, obj, obj_raw, offset)
    }

     #[must_use]
    fn alloc<T: num::NumCast>(&mut self, size: T) -> u64 {
        Self::alloc_impl(&mut self.fso, size)
    }

    ////////

    fn write_impl<O: Object>(ccache: &mut NVObjectCache,nv_blk_dev: &mut FileBackend, obj: O, obj_raw: Vec<u8>, offset: u64) -> ObjectPointer {
        // TODO batch writes

        let op = ObjectPointer::new(offset, obj_raw.len() as u64, O::RAW_TYPE);

        // insert in clean cache
        ccache.insert(&op, obj);

        nv_blk_dev.write(offset, &obj_raw);
        op
    }

    fn obj_to_raw<O: Serializable + RawTyped>(object: &O) -> Result<Vec<u8>, failure::Error> {
        object.serialize()
        // TODO compress, checksum...
    }

    #[must_use]
    fn alloc_impl<T: num::NumCast>(fso: &mut u64, size: T) -> u64 {
        let o = *fso;
        *fso += num::cast::<T, u64>(size).unwrap();
        o
    }
}


    
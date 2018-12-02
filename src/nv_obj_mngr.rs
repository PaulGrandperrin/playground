use crate::object_pointer::ObjectPointer;
use crate::file_backend::FileBackend;
use crate::serializable::Serializable;
use crate::common::RawTyped;
use crate::any_object::AnyObject;
use crate::uberblock::Uberblock;
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
    // TODO add cache
}

impl NVObjectManager {
    const NUM_UBERBLOCKS: u64 = 3;

    #[must_use]
    pub fn new<O: Serializable + RawTyped>(obj: O) -> (Self, ObjectPointer) {
        // initialization
        let mut nv_blk_dev = FileBackend::new();
        let mut fso = Self::NUM_UBERBLOCKS * Uberblock::RAW_SIZE as u64;
        let txg = 1;

        // convert obj to raw
        let obj_raw = Self::obj_to_raw(&obj).unwrap();
        let len = obj_raw.len() as u64;

        // write obj
        let offset = Self::alloc_impl(&mut fso, len);
        let op = Self::write_impl(&mut nv_blk_dev, obj, obj_raw, offset);
        
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
                // TODO cache
            },
            op
        )
    }

    #[must_use]
    pub fn load() -> (Self, ObjectPointer) {
        // initialization
        let mut nv_blk_dev = FileBackend::new();

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
                // TODO cache
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
    pub fn get<O: Serializable>(&mut self, op: &ObjectPointer) -> Rc<O> {
        /*
        match self.map.entry(op.offset) {
            Entry::Occupied(e) => {
                println!("cache hit :-)");
                let en: AnyObject = e.get().try_into().unwrap();//.try_into().unwrap();
                (*e.get()).try_into().unwrap() //FIXME: compiler problem? e.get().deref().try_into()
                // here, implicitly, we drop the old value
            }
            Entry::Vacant(e) => {
                println!("cache miss :-(");
                let o = self.sm.retrieve::<O>(op);
                let rc = Rc::new(o);
                let v = e.insert(rc.into());
                rc
            }
        }
    */
        // TODO check cache
        let raw = self.nv_blk_dev.read(op.offset, op.len);
        let obj = Serializable::deserialize(&raw).unwrap(); // FIXME: not zero-copy
        Rc::new(obj)
    }

    #[must_use]
    fn write<O: Serializable + RawTyped>(&mut self, offset: u64, object: O) -> ObjectPointer {
        let object_raw = Self::obj_to_raw(&object).unwrap();
        let len = object_raw.len() as u64;

        self.nv_blk_dev.write(offset, &object_raw);
        ObjectPointer::new(offset, len, O::RAW_TYPE)
    }

    #[must_use]
    pub fn store<O: Serializable + RawTyped>(&mut self, object: O) -> ObjectPointer {
        let object_raw = Self::obj_to_raw(&object).unwrap();
        let len = object_raw.len() as u64;

        let offset = self.alloc(len);

        self.nv_blk_dev.write(offset, &object_raw);
        ObjectPointer::new(offset, len, O::RAW_TYPE)
    }

     #[must_use]
    fn alloc<T: num::NumCast>(&mut self, size: T) -> u64 {
        Self::alloc_impl(&mut self.fso, size)
    }

    ////////

    fn write_impl<O: Serializable + RawTyped>(nv_blk_dev: &mut FileBackend, obj: O, obj_raw: Vec<u8>, offset: u64) -> ObjectPointer {
        // TODO batch writes
        // TODO cache obj
        nv_blk_dev.write(offset, &obj_raw);
        ObjectPointer::new(offset, obj_raw.len() as u64, O::RAW_TYPE)
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


    
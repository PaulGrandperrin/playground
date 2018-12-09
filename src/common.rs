use super::non_volatile::object::object_type::ObjectType;

pub trait RawSized {
    /// size of seialized form in bytes
    const RAW_SIZE: usize;
}

pub trait ConstObjType {
    const OBJ_TYPE: ObjectType;
}
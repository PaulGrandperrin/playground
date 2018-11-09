use super::object_type::ObjectType;

pub trait RawSized {
    /// size of seialized form in bytes
    const RAW_SIZE: usize;
}

pub trait RawTyped {
    const RAW_TYPE: ObjectType;
}
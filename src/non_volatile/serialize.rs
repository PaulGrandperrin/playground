use bytes::{Buf, BufMut};
use std::io::Cursor;

trait ZeroCopyWrite: std::io::Write {
    fn write_all_to_slice(&mut self, len: u64, f: impl Fn(&mut [u8])) -> std::io::Result<()>;
    //fn take_writable_slice(&mut self, len: u64) -> std::io::Result<&mut [u8]>;
}

impl<'a> ZeroCopyWrite for Cursor<&'a mut [u8]> {
    /*fn take_writable_slice(&mut self, len: u64) -> std::io::Result<&mut [u8]> {
        let pos = self.position();
        self.set_position(pos + len as u64);
        self.get_mut()
            .get_mut(pos as usize..(pos+len) as usize)
            .ok_or(std::io::Error::new(std::io::ErrorKind::WriteZero, "ZeroCopyWrite: not enough space left".into()))
    }*/

    fn write_all_to_slice(&mut self, len: u64, f: impl Fn(&mut [u8])) -> std::io::Result<()> {
        let pos = self.position();
        self.set_position(pos + len as u64);
        let slice = self
            .get_mut()
            .get_mut(pos as usize..(pos + len) as usize)
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::WriteZero,
                "ZeroCopyWrite: not enough space left".into(),
            ))?;

        f(slice);
        Ok(())
    }
}

impl ZeroCopyWrite for std::io::Write {
    fn write_all_to_slice(&mut self, len: u64, f: impl Fn(&mut [u8])) -> std::io::Result<()> {
        let mut slice = vec![0; len as usize].as_mut_slice();
        f(slice);
        self.write_all(slice)
    }
}

pub trait Serialize {
    const FIXED_SIZE: Option<usize>;

    fn serialize(&self, bytes: &mut impl ZeroCopyWrite);
    //fn from_bytes(bytes: &mut Cursor<&[u8]>) -> Result<Self, failure::Error>;
}

use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};

#[derive(Debug)]
pub struct FileBackend {
    bd: File,
    log: File,
}

impl FileBackend {
    #[must_use]
    pub fn new() -> FileBackend {
        let bd = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("bd.raw")
            .expect("failed to open bd.raw");

        let log = OpenOptions::new()
            .read(false)
            .append(true)
            .create(true)
            .open("bd.log")
            .expect("failed to open bd.log");

        FileBackend { bd, log }
    }

    #[must_use]
    pub fn read(&mut self, offset: u64, len: u64) -> Box<[u8]> {
        self.bd.seek(SeekFrom::Start(offset)).unwrap();
        let mut data = vec![0; len as usize].into_boxed_slice();
        self.bd
            .read_exact(&mut data)
            .expect("File::read_exact failed");
        data
    }

    #[must_use]
    pub fn write(&mut self, offset: u64, data: &[u8]) {
        self.bd.seek(SeekFrom::Start(offset)).unwrap();
        self.bd.write_all(&data).expect("File.write_all failed");
    }
}

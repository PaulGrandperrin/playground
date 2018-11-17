use std::fs::OpenOptions;
use std::fs::File;
use std::io::{Read, Seek, Write, SeekFrom};

#[derive(Debug)]
pub struct FileBackend {
    bd: File,
    log: File,
}

impl FileBackend {
    pub fn new() -> FileBackend {
        let bd = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("bd.raw").expect("failed to open bd.raw");

    let log = OpenOptions::new()
        .read(false)
        .append(true)
        .create(true)
        .open("bd.log").expect("failed to open bd.log");

        FileBackend {
            bd,
            log,
        }
    }

    pub fn read(&mut self, offset: u64, length: u64) -> Box<[u8]> {
        self.bd.seek(SeekFrom::Start(offset)).unwrap();
        let mut data = vec![0;length as usize].into_boxed_slice();
        self.bd.read_exact(&mut data).expect("File::read_exact failed");
        data
    }

    pub fn write(&mut self, offset: u64, data: &[u8]) {
        self.bd.seek(SeekFrom::Start(offset)).unwrap();
        self.bd.write_all(&data).expect("File.write_all failed");
    }
}
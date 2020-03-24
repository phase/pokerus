use std::fs::File;
use std::io::{Seek, SeekFrom, Read, Write};

struct Rom {
    path: String,
    buffer: Vec<u8>,
}

impl Rom {
    pub fn from_file(file_name: &str) -> std::io::Result<Rom> {
        let mut file = File::open("foo.txt")?;
        let size = file.seek(SeekFrom::End(0))? as usize;
        let mut buffer = Vec::with_capacity(size);
        file.read_to_end(&mut buffer)?;
        Ok(Rom { path: file_name.to_string(), buffer })
    }

    pub fn write_to_file(&self) -> std::io::Result<()> {
        let mut file = File::open(&self.path)?;
        file.write_all(self.buffer.as_slice())?;
        Ok(())
    }

    pub fn read_byte(&self, offset: usize) -> Option<u8> {
        self.buffer.get(offset).map(|x| x.clone())
    }
}
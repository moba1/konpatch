use std::{convert, io};

pub mod header;

#[derive(Debug)]
pub struct Elf64 {
    header: header::Header64,
}

impl convert::TryInto<Vec<u8>> for Elf64 {
    type Error = io::Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let mut buf = self.header.try_into()?;

        Ok(buf)
    }
}

impl Elf64 {
    pub fn new(hdr_param: header::Header64Parameter) -> Self {
        Self {
            header: header::Header64::new(hdr_param),
        }
    }
}
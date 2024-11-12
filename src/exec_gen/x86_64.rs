use std::io;
use crate::parser;
use crate::elf::header;

#[derive(Debug)]
pub enum Os {
    Linux,
}

#[derive(Debug)]
pub struct ExecGenerator {
    os: Os,
}

impl ExecGenerator {
    pub fn new(os: Os) -> Self {
        Self {
            os,
        }
    }
}

impl super::ExecGenerator for ExecGenerator {
    fn write<W: io::Write>(&self, code: Vec<parser::Symbol>, f: &mut W) -> io::Result<()> {
        let header_param = header::Header64Parameter  {
            byte_order: header::ByteOrder::LittleEndian,
            data_encoding: header::DataEncoding::Data2Lsb,
            version: header::Version::Current,
            os_abi: header::OsAbi::SystemV,
            object_file_type: header::ObjectFileType::Dyn,
            machine: header::Machine::X86_64,
            file_version: header::FileVersion::Current,
            flag: header::Flag::None,
        };
        let hdr = header::Header64::new(header_param);
        let header_bytes: Vec<u8> = hdr.try_into()?;
        f.write_all(&header_bytes)?;

        Ok(())
    }
}

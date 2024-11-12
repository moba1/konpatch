use std::{convert, io};

use byteorder::WriteBytesExt;

#[derive(Debug, Clone)]
pub enum DataEncoding {
    Data2Lsb,
    Data2Msb,
}

impl convert::Into<u8> for DataEncoding {
    fn into(self) -> u8 {
        match self {
            Self::Data2Lsb => 1,
            Self::Data2Msb => 2,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Version {
    Current
}

impl convert::Into<u8> for Version {
    fn into(self) -> u8 {
        match self {
            Self::Current => 1,
        }
    }
}

#[derive(Debug, Clone)]
pub enum OsAbi {
    None,
    SystemV,
}

impl convert::Into<u8> for OsAbi {
    fn into(self) -> u8 {
        match self {
            Self::None | OsAbi::SystemV => 0,
        }
    }
}

#[derive(Debug)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}

#[derive(Debug, Clone)]
pub enum ObjectFileType {
    Dyn,
}

impl convert::Into<u16> for ObjectFileType {
    fn into(self) -> u16 {
        match self {
            Self::Dyn => 3,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Machine {
    X86_64,
}

impl convert::Into<u16> for Machine {
    fn into(self) -> u16 {
        match self {
            Self::X86_64 => 62,
        }
    }
}

#[derive(Debug, Clone)]
pub enum FileVersion {
    Current,
}

impl convert::Into<u32> for FileVersion {
    fn into(self) -> u32 {
        match self {
            Self::Current => 1,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Flag {
    None,
}

impl convert::Into<u32> for Flag {
    fn into(self) -> u32 {
        match self {
            Self::None => 0,
        }
    }
}

#[derive(Debug)]
pub struct Header64Parameter {
    pub byte_order: ByteOrder,
    pub data_encoding: DataEncoding,
    pub version: Version,
    pub os_abi: OsAbi,
    pub object_file_type: ObjectFileType,
    pub machine: Machine,
    pub file_version: FileVersion,
    pub flag: Flag,
}

#[derive(Debug)]
pub(super) struct Header64 {
    param: Header64Parameter,
    entry_address: u64,
    section_header_offset_address: u64,
    program_header_num: u16,
    section_header_num: u16,
    section_string_index: u16,
}

impl convert::TryInto<Vec<u8>> for Header64 {
    type Error = io::Error;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let mut buf = vec![
            // magic number
            0x7F, 0x45, 0x4C, 0x46,
            2,
            self.param.data_encoding.clone().into(),
            self.param.version.clone().into(),
            self.param.os_abi.clone().into(),
            // ABI Version
            0,
            // padding
            0, 0, 0, 0, 0, 0, 0,
        ];

        let subsequent_buf = match self.param.byte_order {
            ByteOrder::LittleEndian => self.to_ordered_bytes::<byteorder::LittleEndian>(),
            ByteOrder::BigEndian => self.to_ordered_bytes::<byteorder::BigEndian>(),
        }?;
        buf.extend_from_slice(&subsequent_buf);

        Ok(buf)
    }
}

impl Header64 {
    pub fn new(param: Header64Parameter) -> Self {
        Self {
            param,
            entry_address: 0,
            section_header_offset_address: 0,
            program_header_num: 0,
            section_header_num: 0,
            section_string_index: 0,
        }
    }

    fn to_ordered_bytes<E: byteorder::ByteOrder>(&self) -> io::Result<Vec<u8>> {
        let mut buf = vec![];

        // ELF Header
        buf.write_u16::<E>(self.param.object_file_type.clone().into())?;
        buf.write_u16::<E>(self.param.machine.clone().into())?;
        buf.write_u32::<E>(self.param.file_version.clone().into())?;
        buf.write_u64::<E>(self.entry_address)?;
        buf.write_u64::<E>(64)?;
        buf.write_u64::<E>(self.section_header_offset_address)?;
        buf.write_u32::<E>(self.param.flag.clone().into())?;
        buf.write_u16::<E>(64)?;
        buf.write_u16::<E>(56)?;
        buf.write_u16::<E>(self.program_header_num)?;
        buf.write_u16::<E>(64)?;
        buf.write_u16::<E>(self.section_header_num)?;
        buf.write_u16::<E>(self.section_string_index)?;

        Ok(buf)
    }
}

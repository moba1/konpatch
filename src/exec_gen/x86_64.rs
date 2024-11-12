use std::io;
use crate::parser;

#[derive(Debug)]
pub struct ExecGenerator;

impl ExecGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl super::ExecGenerator for ExecGenerator {
    fn write<W: io::Write>(&self, code: Vec<parser::Symbol>, f: &mut W) -> io::Result<()> {
        write!(f, "\x7FELF\x02")?;

        Ok(())
    }
}

use std::io;
use crate::parser;

pub mod x86_64;

pub trait ExecGenerator {
    fn write<W: io::Write>(&self, code: Vec<parser::Symbol>, f: &mut W) -> io::Result<()>;
}

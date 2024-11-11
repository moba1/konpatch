
use std::io;
use std::ptr;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Symbol {
    ValueIncrement,
    ValueDecrement,
}

#[derive(Debug)]
pub struct UnknownSymbolError(u8);

impl fmt::Display for UnknownSymbolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown symbol (0x{:02x})", self.0)
    }
}

impl error::Error for UnknownSymbolError {}

pub fn parse<R: io::Read>(reader: R) -> io::Result<Vec<Symbol>> {
    let mut code = vec![];

    for byte in reader.bytes() {
        let symbol = match byte? {
            // `+` in US ASCII
            0x2B => Symbol::ValueIncrement,
            // `-` in US ASCII
            0x2D => Symbol::ValueDecrement,
            // `\n` in US ASCII
            0x0A => continue,
            unknown_symbol => return Err(io::Error::new(
                io::ErrorKind::Other, UnknownSymbolError(unknown_symbol)
            )),
        };
        code.push(symbol)
    }

    Ok(code)
}

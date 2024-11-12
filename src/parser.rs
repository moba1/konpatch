
use std::io;
use std::error;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Symbol {
    ValueIncrement,
    ValueDecrement,
    PointerIncrement,
    PointerDecrement,
    PutCharacter,
    ForwardJump,
    BackwardJump,
    GetCharacter,
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
            // `>` in US ASCII
            0x3E => Symbol::PointerIncrement,
            // `<` in US ASCII
            0x3C => Symbol::PointerDecrement,
            // `.` in US ASCII
            0x2E => Symbol::PutCharacter,
            // `,` in US ASCII
            0x2C => Symbol::GetCharacter,
            // `[` in US ASCII
            0x5B => Symbol::ForwardJump,
            // `]` in US ASCII
            0x5D => Symbol::BackwardJump,
            // `\n` in US ASCII
            _ => continue,
        };
        code.push(symbol)
    }

    Ok(code)
}

#[cfg(test)]
mod tests {
    use std::io;

    #[test]
    fn it_parses() {
        let code = "+-><[],.";
        let parsed_code = vec![
            super::Symbol::ValueIncrement,
            super::Symbol::ValueDecrement,
            super::Symbol::PointerIncrement,
            super::Symbol::PointerDecrement,
            super::Symbol::ForwardJump,
            super::Symbol::BackwardJump,
            super::Symbol::GetCharacter,
            super::Symbol::PutCharacter,
        ];
        let value = super::parse(io::Cursor::new(code));
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), parsed_code);

        let code = "";
        let parsed_code = vec![];
        let value = super::parse(io::Cursor::new(code));
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), parsed_code);
    }
}

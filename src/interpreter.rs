use std::error;
use std::fmt;
use crate::parser;

#[derive(Debug)]
pub struct OutOfRangeError(u8);

impl fmt::Display for OutOfRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "index out of range: {:?}", self.0)
    }
}

impl error::Error for OutOfRangeError {}

#[derive(Debug)]
pub struct Interpreter {
    mem: Vec<u8>,
    index: usize,
}

impl Interpreter {
    pub fn run(&mut self, code: Vec<parser::Symbol>) -> Result<(), OutOfRangeError> {
        for mnemonic in code {
            match mnemonic {
                parser::Symbol::ValueIncrement => self.increment_value(),
                parser::Symbol::ValueDecrement => self.decrement_value(),
                parser::Symbol::PointerIncrement => self.increment_pointer(),
                parser::Symbol::PointerDecrement => self.decrement_pointer()?,
            }
        }

        Ok(())
    }

    fn increment_value(&mut self) {
        self.mem[self.index] = self.mem[self.index].wrapping_add(1);
    }

    fn decrement_value(&mut self) {
        self.mem[self.index] = self.mem[self.index].wrapping_sub(1);
    }

    fn increment_pointer(&mut self) {
        self.index += 1;
        if self.mem.len() < self.index + 1 {
            self.mem.push(0);
        }
    }

    fn decrement_pointer(&mut self) -> Result<(), OutOfRangeError> {
        if self.index == 0 {
            return Err(OutOfRangeError(0));
        }
        self.index -= 1;

        Ok(())
    }

    pub fn new() -> Self {
        Self {
            mem: vec![0],
            index: 0,
        }
    }
}
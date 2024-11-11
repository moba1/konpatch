use crate::parser;

#[derive(Debug)]
pub struct Interpreter {
    mem: Vec<u8>,
    index: usize,
}

impl Interpreter {
    pub fn run(&mut self, code: Vec<parser::Symbol>) {
        for mnemonic in code {
            match mnemonic {
                parser::Symbol::ValueIncrement => self.increment_value(),
                parser::Symbol::ValueDecrement => self.decrement_value(),
                parser::Symbol::PointerIncrement => self.increment_pointer(),
            }
        }
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

    pub fn new() -> Self {
        Self {
            mem: vec![0],
            index: 0,
        }
    }
}
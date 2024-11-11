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
            }
        }
    }

    fn increment_value(&mut self) {
        let extended_size = self.index + 1 - self.mem.len();
        for _ in 0..extended_size {
            self.mem.push(0);
        }

        self.mem[self.index]+=1;
    }

    pub fn new() -> Self {
        Self {
            mem: vec![],
            index: 0,
        }
    }
}
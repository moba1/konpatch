use std::error;
use std::fmt;
use crate::parser;

#[derive(Debug)]
pub struct OutOfRangeError(String);

impl fmt::Display for OutOfRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "index out of range: {:?}", self.0)
    }
}

impl error::Error for OutOfRangeError {}

#[derive(Debug, PartialEq, Eq)]
enum State {
    ForwardJumped,
    None,
    BackwardJumped,
}

#[derive(Debug)]
pub struct Interpreter {
    mem: Vec<u8>,
    index: usize,
    state: State,
}

impl Interpreter {
    pub fn run(&mut self, code: Vec<parser::Symbol>) -> Result<(), Box<dyn error::Error>> {
        let mut index = 0_usize;
        loop {
            let mnemonic = code[index].clone();
            match mnemonic {
                parser::Symbol::ValueIncrement => self.increment_value(),
                parser::Symbol::ValueDecrement => self.decrement_value(),
                parser::Symbol::PointerIncrement => self.increment_pointer(),
                parser::Symbol::PointerDecrement => self.decrement_pointer()?,
                parser::Symbol::PutCharacter => self.put_character(),
                parser::Symbol::ForwardJump => self.forward_jump(),
                parser::Symbol::BackwardJump => self.backward_jump(),
            }

            match self.state {
                State::None | State::ForwardJumped => {
                    match index.checked_add(1) {
                        None => Err(OutOfRangeError("index overflow".to_string()))?,
                        Some(next_index) => index = next_index,
                    };
                },
                State::BackwardJumped => {
                    match index.checked_sub(1) {
                        None => Err(OutOfRangeError("index underflow".to_string()))?,
                        Some(next_index) => index = next_index,
                    }
                },
            }
            
            if index >= code.len() {
                break
            }
        }

        Ok(())
    }

    fn increment_value(&mut self) {
        if self.state != State::None { return; }
        self.mem[self.index] = self.mem[self.index].wrapping_add(1);
    }

    fn decrement_value(&mut self) {
        if self.state != State::None { return; }
        self.mem[self.index] = self.mem[self.index].wrapping_sub(1);
    }

    fn increment_pointer(&mut self) {
        if self.state != State::None { return; }
        self.index += 1;
        if self.mem.len() < self.index + 1 {
            self.mem.push(0);
        }
    }

    fn decrement_pointer(&mut self) -> Result<(), OutOfRangeError> {
        if self.state != State::None { return Ok(()); }
        if self.index == 0 {
            return Err(OutOfRangeError("index underflow".to_string()));
        }
        self.index -= 1;

        Ok(())
    }

    fn put_character(&mut self) {
        if self.state != State::None { return; }
        let character = char::try_from(self.mem[self.index]).unwrap();
        print!("{}", character);
    }

    fn forward_jump(&mut self) {
        match self.state {
            State::ForwardJumped => return,
            State::BackwardJumped => self.state = State::None,
            State::None => if self.mem[self.index] == 0 {
                self.state = State::ForwardJumped
            }
        }
    }

    fn backward_jump(&mut self) {
        match self.state {
            State::BackwardJumped => return,
            State::ForwardJumped => self.state = State::None,
            State::None => if self.mem[self.index] != 0 {
                self.state = State::BackwardJumped
            }
        }
    }

    pub fn new() -> Self {
        Self {
            mem: vec![0],
            index: 0,
            state: State::None,
        }
    }
}
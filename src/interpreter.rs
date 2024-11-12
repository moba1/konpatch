use std::error;
use std::fmt;
use std::io;
use std::io::Read;
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
pub struct Interrupted;

impl fmt::Display for Interrupted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "interrupted input")
    }
}

impl error::Error for Interrupted {}

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
                parser::Symbol::GetCharacter => self.get_character()?,
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

    fn get_character(&mut self) -> Result<(), Box<dyn error::Error>> {
        if self.state != State::None { return Ok(()); }
        eprint!("input code => ");
        let input_byte  = io::stdin()
            .bytes()
            .next();
        let input_byte = match input_byte {
            None => return Err(Interrupted)?,
            Some(byte) => byte?,
        };

        self.mem[self.index] = input_byte;

        Ok(())
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

#[cfg(test)]
mod tests {
    use std::io;
    use crate::parser;

    #[test]
    fn it_runs() {
        let code = parser::parse(io::Cursor::new(
            "+++[-]>++<++"
        ));
        assert!(code.is_ok());

        let mut vm = super::Interpreter::new();
        let result = vm.run(code.unwrap());
        assert!(result.is_ok());
        assert_eq!(vm.mem, vec![2, 2]);
    }
}
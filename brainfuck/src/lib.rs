pub mod brainfuck;
mod parser;

use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use brainfuck::Brainfuck;

#[derive(Debug)]
pub struct InvalidTokenError(pub char);

impl Display for InvalidTokenError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Invalid token: {}", self.0)
    }
}

impl Error for InvalidTokenError {}

pub fn execute_bf(bf_string: &str) -> Result<Brainfuck, InvalidTokenError> {
    let mut tape = Brainfuck::new();

    tape.run(bf_string)?;
    Ok(tape)
}

#[cfg(test)]
mod tests {
    use crate::{execute_bf, parser::BfInstructions};

    #[test]
    fn it_works() {
        let bf = "++>+++>----";

        let result: Vec<u8> = execute_bf(bf).unwrap().into();

        assert_eq!(result[0], 2);
        assert_eq!(result[1], 3);
        assert_eq!(result[2], 252);
    }

    #[test]
    fn finta() {
        let instructions = BfInstructions::new("++++[>+++[>++<-]<-]").unwrap();

        dbg!(instructions.instructions());
    }
}

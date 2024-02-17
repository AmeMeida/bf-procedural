use std::{io, num::Wrapping};

use crate::{
    parser::{BfInstructions, Token, Unit},
    InvalidTokenError,
};

const ZERO: Wrapping<u8> = Wrapping(0u8);
const ONE: Wrapping<u8> = Wrapping(1u8);

pub struct Brainfuck {
    tape: Vec<Wrapping<u8>>,
    pointer: Wrapping<u8>,
}

impl Brainfuck {
    pub fn new() -> Self {
        Brainfuck {
            tape: vec![ZERO],
            pointer: ZERO,
        }
    }

    /// <
    pub fn left(&mut self) {
        self.pointer = self.pointer - ONE;
    }

    /// \>
    pub fn right(&mut self) {
        self.pointer = self.pointer + ONE;

        if self.pointer.0 > (self.tape.len() as u8) - 1 {
            self.tape.push(ZERO);
        }
    }

    /// +
    pub fn add(&mut self) {
        self.tape[self.pointer.0 as usize] = self.tape[self.pointer.0 as usize] + ONE;
    }

    /// -
    pub fn subtract(&mut self) {
        self.tape[self.pointer.0 as usize] = self.tape[self.pointer.0 as usize] - ONE;
    }

    /// .
    pub fn print(&self) -> char {
        self.tape[self.pointer.0 as usize].0 as char
    }

    pub fn read(&mut self) {
        let mut input = String::new();

        let Ok(_) = io::stdin().read_line(&mut input) else {
            return;
        };

        let Ok(num) = input.parse::<u8>() else {
            return;
        };

        self.tape[self.pointer.0 as usize] = Wrapping(num);
    }

    pub fn tape(&self) -> Vec<u8> {
        self.tape.iter().map(|num| num.0).collect()
    }

    pub fn run(&mut self, commands: &str) -> Result<String, InvalidTokenError> {
        let instructions = BfInstructions::new(commands)?;

        Ok(self.execute(&instructions))
    }

    pub fn execute(&mut self, commands: &BfInstructions) -> String {
        let mut output = String::new();

        for token in commands.instructions() {
            match token {
                Unit::Token(Token::Left) => self.left(),
                Unit::Token(Token::Right) => self.right(),
                Unit::Token(Token::Add) => self.add(),
                Unit::Token(Token::Subtract) => self.subtract(),
                Unit::Token(Token::Print) => output.push(self.print()),
                Unit::Token(Token::Read) => self.read(),
                Unit::Loop(tokens) => {
                    while self.tape[self.pointer.0 as usize].0 != 0 {
                        self.execute(&tokens);
                    }
                }
            }
        }

        output
    }
}

impl Into<Vec<u8>> for Brainfuck {
    fn into(self) -> Vec<u8> {
        self.tape.iter().map(|num| num.0).collect()
    }
}

use crate::InvalidTokenError;

#[derive(Debug)]
pub enum Token {
    Left,
    Right,
    Add,
    Subtract,
    Print,
    Read,
}

#[derive(Debug)]
pub enum Unit {
    Token(Token),
    Loop(Vec<Unit>),
}

pub fn tokenize(bf_string: String) -> Result<Vec<Unit>, InvalidTokenError> {
    let mut tree: Vec<Unit> = Vec::new();

    let tokens: Vec<char> = bf_string.chars().collect();

    let mut position: usize = 0;

    while let Some(token) = tokens.get(position) {
        match token {
            '>' => tree.push(Unit::Token(Token::Right)),
            '<' => tree.push(Unit::Token(Token::Left)),
            '+' => tree.push(Unit::Token(Token::Add)),
            '-' => tree.push(Unit::Token(Token::Subtract)),
            '.' => tree.push(Unit::Token(Token::Print)),
            ',' => tree.push(Unit::Token(Token::Read)),
            '[' => {
                position += 1;
                let mut closing: usize = 1;

                for (index, token) in tokens[position..].iter().enumerate() {
                    match token {
                        '[' => closing += 1,
                        ']' => closing -= 1,
                        _ => (),
                    }

                    if closing == 0 {
                        closing = position + index;
                        break;
                    }
                }

                let loop_slice = String::from(&bf_string[position..closing]);
                tree.push(Unit::Loop(tokenize(loop_slice)?));

                position = closing;
            },
            ' ' | '\n' | '\r' | '\t' => (),
            _ => return Err(InvalidTokenError(token.clone())),
        }

        position += 1;
    }

    Ok(tree)
}

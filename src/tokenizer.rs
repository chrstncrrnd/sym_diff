use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub enum Token {
    Var, // TODO: add functions
    Num(f64),
    Plus,
    Minus,
    Mult,
    Div,
    Power,
    LParen, // groupings
    RParen,
    Eol, //end of line
}

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    current_token: String,
}

impl<'a> Lexer<'a> {
    pub fn new(input: Peekable<Chars<'a>>) -> Self {
        Self {
            input,
            current_token: String::new(),
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(c) = self.input.peek() {
                if *c == 'x' {
                    return Some(Token::Var);
                } else if c.is_ascii_digit() {
                    if let Some(next) = self.input.peek()
                        && next.is_ascii_digit()
                    {
                        self.current_token = format!("{}{}", self.current_token, c);
                        continue;
                    } 
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
    }
}

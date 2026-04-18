use std::{iter::Peekable, str::Chars};

use crate::functions::Func;

#[derive(Debug, PartialEq, Clone)]
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
    Err(String),
    Func(Func),
}

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: Peekable<Chars<'a>>) -> Self {
        Self { input }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // consume whitespace
        while let Some(&ch) = self.input.peek() {
            if ch.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }

        let ch = self.input.next()?;

        // numbers (including decimals)
        if ch.is_ascii_digit() || ch == '.' {
            let mut buf = String::new();
            buf.push(ch);

            while let Some(&next_ch) = self.input.peek() {
                if next_ch.is_ascii_digit() || next_ch == '.' {
                    buf.push(self.input.next().unwrap());
                } else {
                    break;
                }
            }

            return match buf.parse() {
                Ok(n) => Some(Token::Num(n)),
                Err(_) => Some(Token::Err(format!("Invalid number format: {}", buf))),
            };
        }

        // var and funcs
        if ch.is_alphabetic() {
            let mut buf = String::new();
            buf.push(ch);

            // consume all contiguous alphabetic characters
            while let Some(&next_ch) = self.input.peek() {
                if next_ch.is_alphabetic() {
                    buf.push(self.input.next().unwrap());
                } else {
                    break;
                }
            }

            // check if its a var
            if buf == "x" {
                return Some(Token::Var);
            }

            if let Ok(func) = buf.parse::<Func>() {
                return Some(Token::Func(func));
            }

            return Some(Token::Err(format!("Unknown identifier: {}", buf)));
        }

        // operators
        match ch {
            '*' => {
                if let Some(&'*') = self.input.peek() {
                    self.input.next();
                    Some(Token::Power)
                } else {
                    Some(Token::Mult)
                }
            }
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            '/' => Some(Token::Div),
            _ => Some(Token::Err(format!("Unexpected character: {}", ch))),
        }
    }
}

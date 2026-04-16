use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub enum Token{
    Var, // TODO: add functions
    Num(f64),
    Plus,
    Minus,
    Mult,
    Div,
    Power,
    LParen, // groupings
    RParen,
    Eol //end of line
}


#[derive(Debug)]
pub struct Lexer<'a>{
    input: Peekable<Chars<'a>>,
}


impl<'a> Lexer<'a>{
    pub fn new(input: Peekable<Chars<'a>>) -> Self{
        Self{
            input
        }
    }
}

impl Iterator for Lexer<'_>{
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item>{
        match self.input.peek(){
            Some(_) => {
                self.input.next();
                Some(Token::Var)
            },
            None => Some(Token::Eol)
        }
    }
}

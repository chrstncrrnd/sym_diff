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
  Err(String),
}

enum LexerState {
  Initial,
  Asterisk,
  Digit,
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
    let mut state: LexerState = LexerState::Initial;
    let mut c: Option<char>;
    let mut buf = String::new();

    loop {
      c = self.input.next();
      match state {
        LexerState::Initial => {
          // returns none if this is none
        let ch = c?;
        if ch.is_whitespace(){
            continue;
        }
          if ch.is_ascii_digit() {
            if ch == '0' {
              return Some(Token::Err("Got a number starting with 0!".to_string()));
            }
            buf = format!("{}{}", buf, ch);
            if self.input.peek().is_none(){
                return Some(Token::Num(buf.parse().unwrap()));
            }
            if let Some(nch) = self.input.peek() && (*nch).is_ascii_digit(){
                state = LexerState::Digit;
            }else{
                state = LexerState::Initial;
            }

            continue;
          }

          if ch == '*' {
            let next = self.input.peek();
            if let Some(nch) = next
              && *nch == '*'
            {
              state = LexerState::Asterisk;
              continue;
            }
            return Some(Token::Mult);
          }

          return match ch {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            '/' => Some(Token::Div),
            'x' => Some(Token::Var),
            _ => Some(Token::Err("Unexpected character!".to_string())),
          };
        }
        LexerState::Digit => {
          let ch = c.unwrap();
          if ch.is_ascii_digit() {
            buf = format!("{}{}", buf, ch);
          }
          let next = self.input.peek();
          if let Some(nch) = next {
            if !(*nch).is_ascii_digit() {
              return Some(Token::Num(buf.parse().unwrap()));
            }
          } else {
            return Some(Token::Num(buf.parse().unwrap()));
          }
        }
        LexerState::Asterisk => {
        if let Some(ch) = c && ch == '*'{
            return Some(Token::Power);
        }else{
            return Some(Token::Err("Unexpected character!".to_string()));
        }
        }
      }
    }
  }
}

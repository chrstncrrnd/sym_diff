use std::io;

use crate::{parser::parse, tokenizer::{Lexer, Token}};

mod parser;
mod tokenizer;

fn main() {
    let mut buf: String = String::new();
    println!("Expression to differentiate: ");
    io::stdin().read_line(&mut buf).unwrap();
    let a = buf.trim().chars().peekable();

    let lexer = Lexer::new(a);
    println!("{:?}", parse(lexer));
}

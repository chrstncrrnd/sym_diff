use std::io;

use crate::{parser::parse, tokenizer::Lexer};

mod parser;
mod tokenizer;

fn main() {
    let mut buf: String = String::new();
    println!("Expression to differentiate: ");
    io::stdin().read_line(&mut buf).unwrap();
    let a = buf.trim().chars().peekable();

    let lexer = Lexer::new(a);
    let res = parse(lexer);
    if let Ok(oki) = res{
        println!("Parsed correctly as:");
        println!("{:?}", oki);
    }else{
        println!("Got an error!");
        println!("{}", res.err().unwrap());
    }

}

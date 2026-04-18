use std::io;

use crate::{
    parser::parse,
    tokenizer::{Lexer, Token},
};

mod functions;
mod parser;
mod tokenizer;

fn main() {
    let mut buf: String = String::new();
    println!("Expression to differentiate: ");
    io::stdin().read_line(&mut buf).unwrap();
    let a = buf.trim().chars().peekable();

    let lexer = Lexer::new(a);
    println!("Lexer: {:?}", lexer.collect::<Vec<Token>>());
    return;
    let res = parse(lexer);
    if let Ok(oki) = res {
        print!("Parsed expression correctly as: ");
        println!("View: {}", oki);
        println!("DebugView: {:?}", oki);
    } else {
        eprint!("Recieved an error: ");
        eprintln!("{}", res.err().unwrap());
    }
}

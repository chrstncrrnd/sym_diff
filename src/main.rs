use std::io;

use crate::{diff::diff, parser::parse, simpl::simplify, tokenizer::Lexer};

mod diff;
mod functions;
mod matches;
mod parser;
mod simpl;
mod tokenizer;

fn main() {
    let mut buf: String = String::new();
    println!("Expression to differentiate: ");
    io::stdin().read_line(&mut buf).unwrap();
    let a = buf.trim().chars().peekable();

    let lexer = Lexer::new(a);

    let res = parse(lexer);
    if let Ok(oki) = res {
        print!("Parsed expression correctly as: ");
        println!("View: {}", oki);
        println!("DebugView: {:?}", oki);
        let diffed = diff(oki.clone()).unwrap();
        println!("Diff: {}", diffed);
        let simplified = simplify(diffed);
        println!("Simplified: {}", simplified);
    } else {
        eprint!("Recieved an error: ");
        eprintln!("{}", res.err().unwrap());
    }
}

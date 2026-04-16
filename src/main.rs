use std::io;

use crate::tokenizer::Lexer;

mod tokenizer;


fn main() {
    let mut buf: String = String::new();
    println!("Expression to differentiate: ");
    io::stdin().read_line(&mut buf).unwrap();
    let a = buf.trim().chars().peekable();

    let mut lexer = Lexer::new(a);
    println!("Lexer one: {:?}, two: {:?}", lexer.next(), lexer.next());
}

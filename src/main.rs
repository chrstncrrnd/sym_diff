use std::io;

use clap::Parser;
use crate::{diff::diff, parser::parse, simpl::simplify, tokenizer::Lexer};

mod diff;
mod functions;
mod parser;
mod rule_gen;
mod simpl;
mod tokenizer;


/// Symbolically differentiate a given expression
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// If we just simplify the expression
    #[arg(short, long)]
    simplify: bool,

    /// Expression to differentiate or simplify
    #[arg(short, long)]
    expression: Option<String>
}

fn main() {
    let args = Args::parse();


    let mut expression_string = String::new();

    // if we are supplied the expression as an argument
    if let Some(expr) = args.expression{
        expression_string = expr;
    }
    // otherwise we ask for the expression in standard input
    else{
        println!("Expression to differentiate: ");
        io::stdin().read_line(&mut expression_string).unwrap();
    }


    let chars = expression_string.trim().chars().peekable();
    let lexer = Lexer::new(chars);

    let res = parse(lexer);
    if let Ok(oki) = res {
        print!("Parsed expression correctly as: ");
        println!("View: {}", oki);
        println!("DebugView: {:?}", oki);

        // This is for debug purposes:
        if args.simplify{
            println!("Only Simplifying expression!");
            let simplified = simplify(oki.clone());
            println!("Simplified: {}", simplified);
            println!("Simplified DebugView: {:?}", simplified);
            return;
        }


        let diffed = diff(oki.clone()).unwrap();
        println!("Diff: {}", diffed);
        println!("Diff DebugView: {:?}", diffed);
        let simplified = simplify(diffed);
        println!("Simplified: {}", simplified);
        println!("Simplified DebugView: {:?}", simplified);
    } else {
        eprint!("Recieved an error: ");
        eprintln!("{}", res.err().unwrap());
    }
}

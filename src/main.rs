use std::io::{self, Write, stdout};

use crate::{diff::diff, parser::parse, parser::Expr, simpl::simplify, tokenizer::Lexer};
use clap::Parser;

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
    expression: Option<String>,

    /// Enable debug logs
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let mut expression_string = String::new();

    // if we are supplied the expression as an argument
    if let Some(expr) = args.expression {
        expression_string = expr;
    }
    // otherwise we ask for the expression in standard input
    else {
        println!("Expression to differentiate: ");
        print!("> ");
        let _ = stdout().flush();
        io::stdin().read_line(&mut expression_string).unwrap();
    }

    let chars = expression_string.trim().chars().peekable();
    let lexer = Lexer::new(chars);

    let res = parse(lexer);
    if let Ok(oki) = res {
        if args.debug {
            print!("Parsed expression correctly as: ");
            println!("View: {}", oki);
            println!("DebugView: {:?}", oki);
        }
        // This is for debug purposes:
        if args.simplify {
            println!("[INFO] Only simplifying expression");
        }
        let diffed = if args.simplify {Expr::Num(0_f64)} else{diff(oki.clone()).unwrap()};

        if args.debug && !args.simplify {
            println!("Diff: {}", diffed);
            println!("Diff DebugView: {:?}", diffed);
        }
        let simplified = if args.simplify {
            simplify(oki)
        } else {
            simplify(diffed)
        };

        if args.debug {
            println!("Simplified: {}", simplified);
            println!("Simplified DebugView: {:?}", simplified);
        } else {
            println!("{}", simplified);
        }
    } else {
        eprint!("Recieved an error: ");
        eprintln!("{}", res.err().unwrap());
    }
}

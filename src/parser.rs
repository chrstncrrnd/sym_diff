use crate::tokenizer::{Lexer, Token};

#[derive(Clone, Debug)]
pub enum Expr {
    Num(f64),
    Var,
    Prod(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Sum(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
}
// Brackets, powers, Division, Multiplication, addition subtraction
const TOK_PRECENDENCE_COUNT: usize = 5;
const TOK_PRECENDENCE: [Token; TOK_PRECENDENCE_COUNT] = [Token::Power, Token::Div, Token::Mult, Token::Plus, Token::Minus];


pub fn parse(lexer: Lexer) -> Result<Expr, String>{
    let toks: Vec<Token> = lexer.collect();
    parse_at_lvl(toks, TOK_PRECENDENCE_COUNT)
}

fn parse_at_lvl(toks: Vec<Token>, level: usize) -> Result<Expr, String> {
    // println!("Called parse_at_lvl with toks: {:?} and level: {}", toks, level);
    // resolve lowest level
    if level == 0 {
        if toks.len() != 1 {
            return Err("Expected a singular token between binary expression!".to_string());
        }
        println!("{:?}", toks[0]);
        return match toks[0] {
            Token::Var => Ok(Expr::Var),
            Token::Num(n) => Ok(Expr::Num(n)),
            _ => Err("Unexpected token!".to_string()),
        };
    }

    let mut lhs: Vec<Token> = Vec::new();
    let mut rhs: Vec<Token> = Vec::new();
    let mut on_lhs = true;

    for tok in &toks {
        // level 0 is special
        if *tok == TOK_PRECENDENCE[level - 1] {
            on_lhs = false;
        } else if on_lhs {
            lhs.push(tok.clone());
        } else {
            rhs.push(tok.clone());
        }
    }
    // there is none of the specified token in this expression
    if on_lhs {
        parse_at_lvl(lhs, level - 1)
    } else {
        let res_lhs = parse_at_lvl(lhs.clone(), level - 1);
        // rhs may not be free of plus
        let res_rhs = parse_at_lvl(rhs.clone(), level);
        if let Ok(ref lhs_ok) = res_lhs
            && let Ok(rhs_ok) = res_rhs
        {
            match TOK_PRECENDENCE[level - 1] {
                Token::Mult => Ok(Expr::Prod(Box::new(lhs_ok.clone()), Box::new(rhs_ok))),
                Token::Div => Ok(Expr::Div(Box::new(lhs_ok.clone()), Box::new(rhs_ok))),
                Token::Plus => Ok(Expr::Sum(Box::new(lhs_ok.clone()), Box::new(rhs_ok))),
                Token::Minus => Ok(Expr::Sub(Box::new(lhs_ok.clone()), Box::new(rhs_ok))),
                Token::Power => Ok(Expr::Pow(Box::new(lhs_ok.clone()), Box::new(rhs_ok))),
                _ => Err("Fatal error!".to_string()),
            }
        } else if let Err(lhs_err) = res_lhs {
                Err(lhs_err)
            } else {
                res_rhs
            }
            
    }
}

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
    for token in &toks{
        if let Token::Err(msg) = token{
            return Err(format!("Syntax error: {}", msg));
        }
    }
    parse_at_lvl(toks, TOK_PRECENDENCE_COUNT)
}

fn parse_at_lvl(toks: Vec<Token>, level: usize) -> Result<Expr, String> {
    // println!("Called parse_at_lvl with toks: {:?} and level: {}", toks, level);
    // resolve lowest level
    if level == 0 {
        if toks.len() != 1 {
            return Err(format!("Expected a singular token or bracketed expression between binary operator, got tokens: {:?}", toks));
        }
        return match toks[0] {
            Token::Var => Ok(Expr::Var),
            Token::Num(n) => Ok(Expr::Num(n)),
            _ => Err("Unexpected token!".to_string()),
        };
    }

    if TOK_PRECENDENCE[level - 1] == *toks.first().unwrap() || TOK_PRECENDENCE[level - 1] == *toks.last().unwrap(){
        return Err("Error, binary operator requires two arguments!".to_string());
    }

    let mut lhs: Vec<Token> = Vec::new();
    let mut rhs: Vec<Token> = Vec::new();
    let mut on_lhs = true;
    let mut mult_implicit = false;
    let mut bracket_level = 0;
    let mut bracketed_statement = true;

    for tok in &toks {
        if let Token::LParen = tok{
            bracket_level += 1;
        }
        // this condition must go here because if we put it before the above, we get zero at the
        // start of the loop and after the next means we get zero at the end of the loop
        if bracket_level == 0{
            bracketed_statement = false;
        }
        if let Token::RParen = tok{
            bracket_level -= 1;
        }


        // if we are just after a number and we get a variable
        // just like 10x
        if mult_implicit && let Token::Var = tok{
            on_lhs = false;
        }

        // 2 is the multiplication level, but we need to add one to account for the 0th level
        if level == 3 && let Token::Num(_) = tok{
            mult_implicit = true;
        }else{
            mult_implicit = false;
        }

        // level 0 is special
        if *tok == TOK_PRECENDENCE[level - 1] && bracket_level == 0 && on_lhs {
            on_lhs = false;
        } else if on_lhs {
            lhs.push(tok.clone());
        } else {
            rhs.push(tok.clone());
        }
    }
    if bracketed_statement{
        if lhs.is_empty(){
            return Err("Missing operand!".to_string());
        }

        // remove the brackets
        lhs.pop();
        lhs.remove(0);
        return parse_at_lvl(lhs, TOK_PRECENDENCE_COUNT);
    }
    // there is none of the specified operator in this expression
    if on_lhs {
        parse_at_lvl(lhs, level - 1)
    }
    // we have encountered at least one of the given operator
    else {
        // println!("LHS: {:?}, RHS: {:?}", lhs, rhs);
        let res_lhs = parse_at_lvl(lhs.clone(), level - 1);
        // rhs may not be free of current token
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

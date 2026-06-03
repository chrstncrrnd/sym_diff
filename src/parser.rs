use std::rc::Rc;
use std::{fmt::Display, vec};

use crate::{
    functions::Func,
    tokenizer::{Lexer, Token},
};

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Num(f64),
    Var,
    Prod(Rc<Expr>, Rc<Expr>),
    Div(Rc<Expr>, Rc<Expr>),
    Sum(Rc<Expr>, Rc<Expr>),
    Sub(Rc<Expr>, Rc<Expr>),
    Pow(Rc<Expr>, Rc<Expr>),
    F(Func, Rc<Expr>),
}

// Brackets, powers, Division, Multiplication, addition subtraction
const TOK_PRECENDENCE_COUNT: usize = 3;
const TOK_PRECENDENCE: [&[Token]; TOK_PRECENDENCE_COUNT] = [
    &[Token::Power],
    &[Token::Mult, Token::Div],
    &[Token::Plus, Token::Minus],
];

pub fn parse(lexer: Lexer) -> Result<Expr, String> {
    let toks: Vec<Token> = lexer.collect();
    for token in &toks {
        if let Token::Err(msg) = token {
            return Err(format!("Syntax error: {}", msg));
        }
    }
    let toks = preprocessor_explicit_mult(toks);
    parse_at_lvl(toks, TOK_PRECENDENCE_COUNT)
}

fn preprocessor_explicit_mult(tokens: Vec<Token>) -> Vec<Token> {
    let mut out = vec![];
    // we insert a mult between:
    // Num, Var           e.g. => 10x
    // Var, LParen        e.g. => x(10)
    // RParen, LParen     e.g. => (10)(x)
    // Num, LParen        e.g. => 10(x)
    // RParen, Var        e.g. => (10)x

    let mut prev_token: Option<Token> = None;

    for tok in tokens {
        // do we insert a mult token
        let insert_mult = matches!(
            (&prev_token, &tok),
            (
                Some(Token::Num(_)),
                Token::Var | Token::LParen | Token::Func(_)
            ) | (
                Some(Token::Var),
                Token::LParen | Token::Func(_) | Token::Var
            ) | (
                Some(Token::RParen),
                Token::LParen | Token::Var | Token::Func(_)
            )
        );

        if insert_mult {
            out.push(Token::Mult);
        }

        out.push(tok.clone());
        prev_token = Some(tok);
    }

    out
}

fn parse_at_lvl(toks: Vec<Token>, level: usize) -> Result<Expr, String> {
    // println!("Called parse_at_lvl with toks: {:?} and level: {}", toks, level);
    // resolve lowest level
    if level == 0 {
        if matches!(
            (toks.first(), toks.last()),
            (Some(Token::LParen), Some(Token::RParen))
        ) {
            return parse_at_lvl(toks.clone(), TOK_PRECENDENCE_COUNT);
        }
        // we check for functions at the lowest level
        if let Some(Token::Func(f)) = toks.first() {
            if toks.len() < 2 {
                return Err(format!("Expected argument for function {}", f));
            }
            // parse arg of func
            let arg = parse_at_lvl(toks[1..].to_vec(), TOK_PRECENDENCE_COUNT)?;
            return Ok(Expr::F(*f, Rc::new(arg)));
        }

        if toks.len() != 1 {
            return Err(format!(
                "Expected a singular token or bracketed expression between binary operator, got tokens: {:?}",
                toks
            ));
        }
        return match toks[0] {
            Token::Var => Ok(Expr::Var),
            Token::Num(n) => Ok(Expr::Num(n)),
            _ => Err("Unexpected token!".to_string()),
        };
    }

    if *toks.first().unwrap() == Token::Minus {
        let rhs = parse_at_lvl(toks.clone().split_off(1), level)?;
        return Ok(Expr::Prod(Rc::new(Expr::Num(-1.0)), Rc::new(rhs)));
    }

    if TOK_PRECENDENCE[level - 1].contains(toks.first().unwrap())
        || TOK_PRECENDENCE[level - 1].contains(toks.last().unwrap())
    {
        return Err("Error, binary operator requires two arguments!".to_string());
    }

    let mut split_idx: Option<usize> = None;
    let mut bracket_level = 0;
    let mut bracketed_statement = true;

    for (i, tok) in toks.iter().enumerate() {
        if let Token::LParen = tok {
            bracket_level += 1;
        }
        if bracket_level == 0 {
            bracketed_statement = false;
        }
        if let Token::RParen = tok {
            bracket_level -= 1;
        }

        if TOK_PRECENDENCE[level - 1].contains(tok) && bracket_level == 0 {
            if level - 1 == 0 {
                // power is right-associative, so split at the first occurrence
                if split_idx.is_none() {
                    split_idx = Some(i);
                }
            } else {
                // rest is left-associative, so split at the last occurrence
                split_idx = Some(i);
            }
        }
    }

    if bracketed_statement {
        if toks.len() <= 2 {
            return Err("Missing operand!".to_string());
        }

        // remove the brackets
        let mut inner = toks.clone();
        inner.pop();
        inner.remove(0);
        return parse_at_lvl(inner, TOK_PRECENDENCE_COUNT);
    }

    if let Some(idx) = split_idx {
        // we have encountered at least one of the given operator
        let operator_tok = toks[idx].clone();
        let mut lhs = toks.clone();
        let mut rhs = lhs.split_off(idx);
        // remove the operator token
        rhs.remove(0);

        let (lhs_level, rhs_level) = if level - 1 == 0 {
            (level - 1, level)
        } else {
            (level, level - 1)
        };

        let res_lhs = parse_at_lvl(lhs.clone(), lhs_level);
        let res_rhs = parse_at_lvl(rhs.clone(), rhs_level);
        if let Ok(ref lhs_ok) = res_lhs
            && let Ok(rhs_ok) = res_rhs
        {
            match operator_tok {
                Token::Mult => Ok(Expr::Prod(Rc::new(lhs_ok.clone()), Rc::new(rhs_ok))),
                Token::Div => Ok(Expr::Div(Rc::new(lhs_ok.clone()), Rc::new(rhs_ok))),
                Token::Plus => Ok(Expr::Sum(Rc::new(lhs_ok.clone()), Rc::new(rhs_ok))),
                Token::Minus => Ok(Expr::Sub(Rc::new(lhs_ok.clone()), Rc::new(rhs_ok))),
                Token::Power => Ok(Expr::Pow(Rc::new(lhs_ok.clone()), Rc::new(rhs_ok))),
                _ => Err("Fatal error!".to_string()),
            }
        } else if let Err(lhs_err) = res_lhs {
            Err(lhs_err)
        } else {
            res_rhs
        }
    }
    // there is none of the specified operator in this expression
    else{
        parse_at_lvl(toks, level - 1)
    }
}

impl Expr {
    pub fn precedence(&self) -> u8 {
        match self {
            Expr::Sum(_, _) | Expr::Sub(_, _) => 1,
            Expr::Prod(_, _) | Expr::Div(_, _) => 2,
            Expr::Pow(_, _) => 3,
            Expr::Num(_) | Expr::Var | Expr::F(_, _) => 4,
        }
    }
}

fn fmt_child(child: &Expr, parent_prec: u8, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if child.precedence() < parent_prec {
        write!(f, "({})", child)
    } else {
        write!(f, "{}", child)
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prec = self.precedence();
        match self {
            Expr::Var => write!(f, "x"),
            Expr::Num(n) => write!(f, "{n}"),
            Expr::Div(a, b) => {
                fmt_child(a, prec, f)?;
                write!(f, " / ")?;
                fmt_child(b, prec + 1, f)
            }
            Expr::Sum(a, b) => {
                fmt_child(a, prec, f)?;
                write!(f, " + ")?;
                fmt_child(b, prec, f)
            }
            Expr::Sub(a, b) => {
                fmt_child(a, prec, f)?;
                write!(f, " - ")?;
                fmt_child(b, prec + 1, f)
            }
            Expr::Pow(a, b) => {
                fmt_child(a, prec + 1, f)?;
                write!(f, " ** ")?;
                fmt_child(b, prec, f)
            }
            Expr::Prod(a, b) => {
                if let Expr::Num(-1.0) = **a {
                    write!(f, "-{b}")
                } else if let Expr::Num(first) = **a
                    && let Expr::Num(second) = **b
                {
                    write!(f, "{first} * {second}")
                } else {
                    fmt_child(a, prec, f)?;
                    let needs_star = match &**a {
                        Expr::Num(_) => match &**b {
                            Expr::Num(_) => true,
                            Expr::Prod(b_left, _) => matches!(**b_left, Expr::Num(_)),
                            _ => false,
                        },
                        _ => false,
                    };
                    if needs_star {
                        write!(f, " * ")?;
                    }
                    fmt_child(b, prec, f)
                }
            }
            Expr::F(func, arg) => {
                write!(f, "{func}({arg})")
            }
        }
    }
}

use crate::parser::Expr;
use crate::{gen_rule, try_apply_all, try_apply};

pub fn simplify(expr: Expr) -> Expr {
    // iterate until we reach a fixed point
    let mut expr = expr;
    let mut prev: Option<Expr> = None;
    // TODO: memory optimizations here
    loop {
        if let Some(p) = prev
            && p == expr
        {
            break;
        }
        prev = Some(expr.clone());
        expr = simpl(expr);
    }
    expr
}


gen_rule!(sum_ident_left; Expr::Sum(lhs, rhs), Expr::Num(k) = *lhs, k == 0.0 => (*rhs).clone());
gen_rule!(sum_ident_right; Expr::Sum(lhs, rhs), Expr::Num(k) = *rhs, k == 0.0 => (*lhs).clone());

// internal simplification function
fn simpl(expr: Expr) -> Expr {
    try_apply_all!(
        @no_some
        sum_ident_left,
        sum_ident_right
        on expr
    );
    expr
}


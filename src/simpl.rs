use crate::parser::Expr;
use std::rc::Rc;
use crate::{gen_rule, try_apply_all, try_apply, sum, sub, prod, div, pow};

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

// SUM/SUB RULES:
gen_rule!(sum_ident_left; Expr::Sum(lhs, rhs) | Expr::Sub(lhs, rhs), Expr::Num(k) = *lhs, k == 0.0 
    => simpl((*rhs).clone()));
gen_rule!(sum_ident_right; Expr::Sum(lhs, rhs) | Expr::Sub(lhs, rhs), Expr::Num(k) = *rhs, k == 0.0 
    => simpl((*lhs).clone()));
gen_rule!(sum_elements; Expr::Sum(lhs, rhs), Expr::Num(k) = *lhs, Expr::Num(n) = *rhs => Expr::Num(k + n));
gen_rule!(sub_elements; Expr::Sub(lhs, rhs), Expr::Num(k) = *lhs, Expr::Num(n) = *rhs => Expr::Num(k - n));

// MULT/DIV RULES
gen_rule!(mult_ident_left; Expr::Prod(lhs, rhs), Expr::Num(k) = *lhs, k == 1.0 
    => simpl((*rhs).clone()));
gen_rule!(mult_ident_right; Expr::Prod(lhs, rhs), Expr::Num(k) = *rhs, k == 1.0 => 
    simpl((*lhs).clone()));
gen_rule!(mult_elements; Expr::Prod(lhs, rhs), Expr::Num(k) = *lhs, Expr::Num(n) = *rhs => Expr::Num(k * n));

// 1/k != k, k/1 == k
gen_rule!(div_ident; Expr::Div(lhs, rhs), Expr::Num(k) = *rhs, k == 1.0 => simpl((*lhs).clone()));
gen_rule!(div_zero; Expr::Div(_, rhs), Expr::Num(n) = *rhs, n == 0.0 => @no_return panic!("Division by zero!"));
gen_rule!(div_elements; Expr::Div(lhs, rhs), Expr::Num(k) = *lhs, Expr::Num(n) = *rhs => Expr::Num(k / n));

// POW RUlES
gen_rule!(pow_ident; Expr::Pow(lhs, rhs), Expr::Num(k) = *rhs, k == 1.0 => simpl((*lhs).clone()));
gen_rule!(pow_zero_zero; Expr::Pow(lhs, rhs), Expr::Num(k) = *rhs, Expr::Num(n) = *lhs, k == 0.0, n == 0.0 => @no_return panic!("0**0 is undefined!"));
gen_rule!(pow_zero; Expr::Pow(_, rhs), Expr::Num(k) = *rhs, k == 0.0 => Expr::Num(1.0));


// recursion rules
gen_rule!(sum_recursion; Expr::Sum(a, b) => sum!(simplify((*a).clone()), simplify((*b).clone())));
gen_rule!(sub_recursion; Expr::Sub(a, b) => sub!(simplify((*a).clone()), simplify((*b).clone())));
gen_rule!(prod_recursion; Expr::Prod(a, b) => prod!(simplify((*a).clone()), simplify((*b).clone())));
gen_rule!(div_recursion; Expr::Div(a, b) => div!(simplify((*a).clone()), simplify((*b).clone())));
gen_rule!(pow_recursion; Expr::Pow(a, b) => pow!(simplify((*a).clone()), simplify((*b).clone())));



// internal simplification function
fn simpl(expr: Expr) -> Expr {
    try_apply_all!(
        @no_some
        sum_ident_left,
        sum_ident_right,
        sum_elements,
        sub_elements,
        mult_ident_left,
        mult_ident_right,
        mult_elements,
        div_ident,
        div_zero,
        div_elements,
        pow_ident,
        pow_zero_zero,
        pow_zero,
        sum_recursion,
        sub_recursion,
        prod_recursion,
        div_recursion,
        pow_recursion
        on expr
    );
    expr
}


use crate::parser::Expr;
use std::rc::Rc;
use crate::{gen_rule, try_apply_all, try_apply, sum, sub, prod, div, pow};

pub fn simplify(expr: Expr) -> Expr {
    // iterate until we reach a fixed point
    let mut expr = expr;
    let mut prev: Option<Expr> = None;
    // TODO: memory optimizations here
    loop {
        // dbg!("One loop", &expr);
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
gen_rule!(sum_recursion; Expr::Sum(a, b) => sum!(simpl((*a).clone()), simpl((*b).clone())));
gen_rule!(sub_recursion; Expr::Sub(a, b) => sub!(simpl((*a).clone()), simpl((*b).clone())));
gen_rule!(prod_recursion; Expr::Prod(a, b) => prod!(simpl((*a).clone()), simpl((*b).clone())));
gen_rule!(div_recursion; Expr::Div(a, b) => div!(simpl((*a).clone()), simpl((*b).clone())));
gen_rule!(pow_recursion; Expr::Pow(a, b) => pow!(simpl((*a).clone()), simpl((*b).clone())));


gen_rule![collect_coeffs_left -> Option<(f64, Expr)>; 
    Expr::Prod(lhs, rhs),
    Expr::Num(k) = *lhs => @no_return
    {
        let mut ret = collect_coeffs((*rhs).clone());
        ret.0 *= k;
        return Some(ret);
    }
];

gen_rule![collect_coeffs_right -> Option<(f64, Expr)>; 
    Expr::Prod(lhs, rhs),
    Expr::Num(k) = *rhs => @no_return
    {
        let mut ret = collect_coeffs((*lhs).clone());
        ret.0 *= k;
        return Some(ret);
    }
];


gen_rule!(base_num; Expr::Num(k) => Expr::Num(k));
gen_rule!(base_var; Expr::Var => Expr::Var);


// internal simplification function
fn simpl(expr: Expr) -> Expr {
    try_apply_all!(
        @no_some
        base_num,
        base_var,
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
        pow_recursion,
        collect_coeffs_wrapper
        on expr
    );
    expr
}


fn collect_coeffs_wrapper(expr: Expr) -> Option<Expr>{
    let collected = collect_coeffs(expr);
    Some(prod!(Expr::Num(collected.0), collected.1))
}

fn collect_coeffs(expr: Expr) -> (f64, Expr) {
    // dbg!("Collect coeffs: ", expr.clone());
    try_apply_all!(
        @no_some
        collect_coeffs_left,
        collect_coeffs_right
        on expr
    );
    (1.0, expr)
}

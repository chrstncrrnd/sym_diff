use crate::parser::Expr;
use std::rc::Rc;
use crate::{gen_rule, try_apply_all, try_apply, sum, sub, prod, div, pow};

pub fn simplify(expr: Expr) -> Expr {
    // iterate until we reach a fixed point
    let mut expr = expr;
    let mut prev: Option<Expr> = None;
    // TODO: memory optimizations here
    loop {
        // dbg!(&expr);
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
gen_rule!(sum_ident_left; Expr::Sum(lhs, rhs), Expr::Num(k) = *lhs, k == 0.0 
    => simpl((*rhs).clone()));
gen_rule!(sub_ident_left; Expr::Sub(lhs, rhs), Expr::Num(k) = *lhs, k == 0.0 
    => prod!(Expr::Num(-1.0), simpl((*rhs).clone())));
gen_rule!(sum_ident_right; Expr::Sum(lhs, rhs) | Expr::Sub(lhs, rhs), Expr::Num(k) = *rhs, k == 0.0 
    => simpl((*lhs).clone()));
gen_rule!(sum_elements; Expr::Sum(lhs, rhs), Expr::Num(k) = *lhs, Expr::Num(n) = *rhs => Expr::Num(k + n));
gen_rule!(sub_elements; Expr::Sub(lhs, rhs), Expr::Num(k) = *lhs, Expr::Num(n) = *rhs => Expr::Num(k - n));


// PROD/DIV RULES
gen_rule!(prod_ident_left; Expr::Prod(lhs, rhs), Expr::Num(k) = *lhs, k == 1.0 
    => simpl((*rhs).clone()));
gen_rule!(prod_ident_right; Expr::Prod(lhs, rhs), Expr::Num(k) = *rhs, k == 1.0 => 
    simpl((*lhs).clone()));
gen_rule!(prod_elements; Expr::Prod(lhs, rhs), Expr::Num(k) = *lhs, Expr::Num(n) = *rhs => Expr::Num(k * n));
gen_rule!(prod_zero_lhs; Expr::Prod(lhs, _), Expr::Num(k) = *lhs, k == 0.0 => Expr::Num(0.0));
gen_rule!(prod_zero_rhs; Expr::Prod(_, rhs), Expr::Num(k) = *rhs, k == 0.0 => Expr::Num(0.0));

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
        let mut ret = collect_coeffs(simpl((*rhs).clone()));
        ret.0 *= k;
        return Some(ret);
    }
];

gen_rule![collect_coeffs_right -> Option<(f64, Expr)>; 
    Expr::Prod(lhs, rhs),
    Expr::Num(k) = *rhs => @no_return
    {
        let mut ret = collect_coeffs(simpl((*lhs).clone()));
        ret.0 *= k;
        return Some(ret);
    }
];

gen_rule![collect_sums_left -> Option<(f64, Expr)>; 
    Expr::Sum(lhs, rhs),
    Expr::Num(k) = *lhs => @no_return
    {
        let mut ret = collect_sums(simpl((*rhs).clone()));
        ret.0 += k;
        return Some(ret);
    }
];

gen_rule![collect_sums_right -> Option<(f64, Expr)>; 
    Expr::Sum(lhs, rhs),
    Expr::Num(k) = *rhs => @no_return
    {
        let mut ret = collect_sums(simpl((*lhs).clone()));
        ret.0 += k;
        return Some(ret);
    }
];

gen_rule![collect_subs_left -> Option<(f64, Expr)>; 
    Expr::Sub(lhs, rhs),
    Expr::Num(k) = *lhs => @no_return
    {
        let mut ret = collect_sums(simpl((*rhs).clone()));
        ret.0 += k;
        ret.1 = prod!(Expr::Num(-1.0), ret.1);
        return Some(ret);
    }
];

gen_rule![collect_subs_right -> Option<(f64, Expr)>; 
    Expr::Sub(lhs, rhs),
    Expr::Num(k) = *rhs => @no_return
    {
        let mut ret = collect_sums(simpl((*lhs).clone()));
        ret.0 -= k;
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
        sub_ident_left,
        sum_ident_right,
        sum_elements,
        sub_elements,
        prod_ident_left,
        prod_ident_right,
        prod_zero_lhs,
        prod_zero_rhs,
        prod_elements,
        div_ident,
        div_zero,
        div_elements,
        pow_ident,
        pow_zero_zero,
        pow_zero,
        collect_coeffs_wrapper,
        collect_sums_wrapper,
        sum_recursion,
        sub_recursion,
        prod_recursion,
        div_recursion,
        pow_recursion
        on expr
    );
    expr
}


fn collect_sums_wrapper(expr: Expr) -> Option<Expr>{
    let collected = collect_sums(expr.clone());
    if collected.0 == 0.0 && collected.1 == expr {
        None
    } else {
        Some(sum!(Expr::Num(collected.0), collected.1))
    }
}

fn collect_sums(expr: Expr) -> (f64, Expr) {
    // dbg!("Collect sums: ", expr.clone());
    try_apply_all!(
        @no_some
        collect_sums_left,
        collect_sums_right,
        collect_subs_left,
        collect_subs_right
        on expr
    );
    (0.0, expr)
}

fn collect_coeffs_wrapper(expr: Expr) -> Option<Expr>{
    let collected = collect_coeffs(expr.clone());
    if collected.0 == 1.0 && collected.1 == expr {
        None
    } else {
        Some(prod!(Expr::Num(collected.0), collected.1))
    }
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

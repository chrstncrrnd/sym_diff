use crate::parser::Expr;
use crate::functions::Func;
use crate::{div, gen_rule, pow, prod, sub, sum, try_apply};

gen_rule!(var_rule; Expr::Var => Expr::Num(1.0));

gen_rule!(const_rule; Expr::Num(_) => Expr::Num(0_f64));

gen_rule!(prod_linearity; Expr::Prod(ea, eb), Expr::Num(k) = *ea =>
    @no_some diff(*eb).map(|rhs| prod!(rhs, Expr::Num(k)))
);

gen_rule!(sum_linearity; Expr::Sum(ea, eb), Some(lhs) = diff(*ea), Some(rhs) = diff(*eb) => sum!(lhs, rhs));

gen_rule!(sub_linearity; Expr::Sub(ea, eb), Some(lhs) = diff(*ea), Some(rhs) = diff(*eb) => sub!(lhs, rhs));

gen_rule!(pow_rule; Expr::Pow(base, exp), Expr::Num(p) = *exp, Some(u_prime) = diff(*base.clone())
    =>
    prod!(
        Expr::Num(p),
        prod!(
            u_prime,
            pow!(
                *base,
                Expr::Num(p-1.0)
            )
        )
    )
);

gen_rule!(product_rule; Expr::Prod(a, b), Some(a_prime) = diff(*a.clone()), Some(b_prime) = diff(*b.clone()) =>
    sum!(
        prod!(*a, b_prime),
        prod!(*b, a_prime)
    )
);

gen_rule!(quotient_rule; Expr::Div(u, v), Some(u_prime) = diff(*u.clone()), Some(v_prime) = diff(*v.clone()) =>
    div!(
        sub!(
            prod!(
                *v.clone(),
                u_prime
            ),
            prod!(
                *u.clone(),
                v_prime
            )
        ),
        pow!(
            *v,
            Expr::Num(2_f64)
        )
    )
);

gen_rule!(sine_rule; Expr::F(Func::Sin, arg), Some(arg_prime) = diff(*arg.clone()) => 
    prod!(arg_prime, Expr::F(Func::Cos, arg))
);

pub fn diff(expr: Expr) -> Option<Expr> {
    try_apply!(var_rule, expr);
    try_apply!(const_rule, expr);
    try_apply!(prod_linearity, expr);
    try_apply!(sum_linearity, expr);
    try_apply!(pow_rule, expr);
    try_apply!(sub_linearity, expr);
    try_apply!(product_rule, expr);
    try_apply!(quotient_rule, expr);
    try_apply!(sine_rule, expr);
    None
}

//fn var_rule(expr: Expr) -> Option<Expr> {
//    if let Expr::Var = expr {
//        return Some(Expr::Num(1_f64));
//    }
//    None
//}
//
//fn const_rule(expr: Expr) -> Option<Expr> {
//    if let Expr::Num(_) = expr {
//        return Some(Expr::Num(0_f64));
//    }
//    None
//}
//
//fn pow_rule(expr: Expr) -> Option<Expr> {
//    if let Expr::Pow(base, exp) = expr
//        && let Expr::Num(p) = *exp
//        && let Some(u_prime) = diff(*base.clone())
//    {
//        let ret = Some(Expr::Prod(
//            Box::new(Expr::Num(p)),
//            Box::new(Expr::Prod(
//                Box::new(u_prime),
//                Box::new(Expr::Pow(Box::new(*base), Box::new(Expr::Num(p - 1.0)))),
//            )),
//        ));
//        println!("{:?}", ret);
//        return ret;
//    }
//    None
//}

//fn linearity_rule(expr: Expr) -> Option<Expr> {
//    if let Expr::Prod(ea, eb) = expr.clone()
//        && let Expr::Num(k) = *ea
//    {
//        if let Some(rhs) = diff(*eb) {
//            return Some(Expr::Prod(Box::new(Expr::Num(k)), Box::new(rhs)));
//        } else {
//            return None;
//        }
//    }
//
//    if let Expr::Sum(ea, eb) = expr {
//        if let Some(lhs) = diff(*ea)
//            && let Some(rhs) = diff(*eb)
//        {
//            return Some(Expr::Sum(Box::new(lhs), Box::new(rhs)));
//        } else {
//            return None;
//        }
//    }
//
//    None
//}

use crate::gen_rule;
use crate::parser::Expr;
use crate::try_apply;

gen_rule!(var_rule; Expr::Var => Expr::Num(1.0));
gen_rule!(const_rule; Expr::Num(_) => Expr::Num(0_f64));
gen_rule!(prod_linearity; Expr::Prod(ea, eb), Expr::Num(k) = *ea => @no_some{
        if let Some(rhs) = diff(*eb) {
            return Some(Expr::Prod(Box::new(Expr::Num(k)), Box::new(rhs)));
        } else {
            return None;
        }
});

pub fn diff(expr: Expr) -> Option<Expr> {
    try_apply!(var_rule, expr);
    try_apply!(const_rule, expr);
    try_apply!(prod_linearity, expr);
    //try_apply!(pow_rule, expr);
    //try_apply!(const_rule, expr);
    //try_apply!(var_rule, expr);

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
fn pow_rule(expr: Expr) -> Option<Expr> {
    if let Expr::Pow(base, exp) = expr
        && let Expr::Num(p) = *exp
        && let Some(u_prime) = diff(*base.clone())
    {
        let ret = Some(Expr::Prod(
            Box::new(Expr::Num(p)),
            Box::new(Expr::Prod(
                Box::new(u_prime),
                Box::new(Expr::Pow(Box::new(*base), Box::new(Expr::Num(p - 1.0)))),
            )),
        ));
        println!("{:?}", ret);
        return ret;
    }
    None
}

fn linearity_rule(expr: Expr) -> Option<Expr> {
    if let Expr::Prod(ea, eb) = expr.clone()
        && let Expr::Num(k) = *ea
    {
        if let Some(rhs) = diff(*eb) {
            return Some(Expr::Prod(Box::new(Expr::Num(k)), Box::new(rhs)));
        } else {
            return None;
        }
    }

    if let Expr::Sum(ea, eb) = expr {
        if let Some(lhs) = diff(*ea)
            && let Some(rhs) = diff(*eb)
        {
            return Some(Expr::Sum(Box::new(lhs), Box::new(rhs)));
        } else {
            return None;
        }
    }

    None
}

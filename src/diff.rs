use crate::parser::Expr;

pub fn diff(expr: Expr) -> Option<Expr> {
    if let Some(ret) = linearity_rule(expr.clone()) {
        return Some(ret);
    }

    if let Some(ret) = pow_rule(expr.clone()) {
        return Some(ret);
    }

    if let Some(ret) = const_rule(expr.clone()){
        return Some(ret);
    }

    if let Some(ret) = var_rule(expr.clone()){
        return Some(ret);
    }
    None
}

fn var_rule(expr: Expr) -> Option<Expr>{
    if let Expr::Var = expr{
        return Some(Expr::Num(1_f64));
    }
    None
}

fn const_rule(expr: Expr) -> Option<Expr>{
    if let Expr::Num(_) = expr{
        return Some(Expr::Num(0_f64))
    }
    None
}


fn pow_rule(expr: Expr) -> Option<Expr> {
    if let Expr::Pow(base, exp) = expr
        && let Expr::Num(p) = *exp
    {
        if let Some(u_prime) = diff(*base.clone()) {
            return Some(Expr::Prod(
                Box::new(Expr::Num(p)),
                Box::new(Expr::Prod(
                    Box::new(u_prime),
                    Box::new(Expr::Pow(Box::new(*base), Box::new(Expr::Num(p - 1.0)))),
                )),
            ));
        }
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

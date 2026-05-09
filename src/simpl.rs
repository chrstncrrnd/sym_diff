use crate::parser::Expr;

pub fn simplify(expr: Expr) -> Expr {
    println!("Running simplify on expression: {}", expr);

    let (coeff, base_expr) = collect_coeffs(expr.clone());
    if coeff != 1.0 {
        return Expr::Prod(Box::new(Expr::Num(coeff)), Box::new(simplify(base_expr)));
    }

    let (s, base_expr) = collect_sums(expr.clone());
    if s != 0.0 {
        return Expr::Sum(Box::new(Expr::Num(s)), Box::new(simplify(base_expr)));
    }

    if let Expr::Prod(a, b) = expr.clone() {
        if let Expr::Num(1.0) = *a {
            return simplify(*b);
        }
        if let Expr::Num(1.0) = *b {
            return simplify(*a);
        }
        if let Expr::Num(n) = *a
            && let Expr::Num(k) = *b
        {
            return Expr::Num(n * k);
        }
        return Expr::Prod(Box::new(simplify(*a)), Box::new(simplify(*b)));
    }

    if let Expr::Sum(a, b) = expr.clone() {
        if let Expr::Num(0.0) = *a {
            return simplify(*b);
        }
        if let Expr::Num(0.0) = *b {
            return simplify(*a);
        }

        if let Expr::Num(n) = *a
            && let Expr::Num(k) = *b
        {
            return Expr::Num(n + k);
        }
        return Expr::Sum(Box::new(simplify(*a)), Box::new(simplify(*b)));
    }

    if let Expr::Pow(a, b) = expr.clone() {
        if let Expr::Num(1.0) = *b {
            return simplify(*a);
        }
        return Expr::Pow(Box::new(simplify(*a)), Box::new(simplify(*b)))
    }

    expr
}

fn collect_coeffs(expr: Expr) -> (f64, Expr) {
    if let Expr::Prod(lhs, rhs) = expr.clone() {
        if let Expr::Num(k) = *lhs {
            let mut ret = collect_coeffs(*rhs);
            ret.0 *= k;
            return ret;
        }
        if let Expr::Num(k) = *rhs {
            let mut ret = collect_coeffs(*lhs);
            ret.0 *= k;
            return ret;
        }
    }
    (1.0, expr)
}

fn collect_sums(expr: Expr) -> (f64, Expr) {
    if let Expr::Sum(lhs, rhs) = expr.clone() {
        if let Expr::Num(k) = *lhs {
            let mut ret = collect_sums(*rhs);
            ret.0 += k;
            return ret;
        }
        if let Expr::Num(k) = *rhs {
            let mut ret = collect_sums(*lhs);
            ret.0 += k;
            return ret;
        }
    }
    (0.0, expr)
}

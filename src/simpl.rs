use std::rc::Rc;
use crate::parser::Expr;

pub fn simplify(expr: Expr) -> Expr {
    // dbg!("Simplify: ", expr.clone());
    if let Expr::Div(a, b) = expr.clone(){
        let a = Rc::new(simplify((*a).clone()));
        let b = Rc::new(simplify((*b).clone()));
        return Expr::Div(a, b);
    }


    if let Expr::Prod(a, b) = expr.clone() {
        let a = Rc::new(simplify((*a).clone()));
        let b = Rc::new(simplify((*b).clone()));
        if let Expr::Num(1.0) = *a {
            return (*b).clone();
        }
        if let Expr::Num(1.0) = *b {
            return (*a).clone();
        }
        if let Expr::Num(n) = *a
            && let Expr::Num(k) = *b
        {
            return Expr::Num(n * k);
        }
        let expr = Expr::Prod(a.clone(), b.clone());
        let (coeff, base_expr) = collect_coeffs(expr.clone());
        if coeff == 0.0 {
            return Expr::Num(0_f64);
        }
        if coeff != 1.0 {
            return Expr::Prod(Rc::new(Expr::Num(coeff)), Rc::new(simplify(base_expr)));
        }
        return base_expr;
    }

    if let Expr::Sum(a, b) | Expr::Sub(a, b) = expr.clone() {
        let sub = matches!(expr, Expr::Sub(_, _));
        let a = Rc::new(simplify((*a).clone()));
        let b = Rc::new(simplify((*b).clone()));
        if let Expr::Num(0.0) = *a {
            return (*b).clone();
        }
        if let Expr::Num(0.0) = *b {
            return (*a).clone();
        }

        if let Expr::Num(n) = *a
            && let Expr::Num(k) = *b
        {
            if sub {
                return Expr::Num(n - k);
            } else {
                return Expr::Num(n + k);
            }
        }
        let expr = if sub {
            Expr::Sub(a.clone(), b.clone())
        } else {
            Expr::Sum(a.clone(), b.clone())
        };
        let (s, base_expr) = collect_sums(expr.clone());

        // if our coefficient was non-zero we must sum or sub it
        if s != 0.0 {
            // collect_sums already accounts for subs
            return Expr::Sum(Rc::new(Expr::Num(s)), Rc::new(simplify(base_expr)));
        }
        return base_expr;
    }

    if let Expr::Pow(a, b) = expr.clone() {
        let a = Rc::new(simplify((*a).clone()));
        let b = Rc::new(simplify((*b).clone()));
        if let Expr::Num(1.0) = *b {
            return (*a).clone();
        }
        if let Expr::Num(0.0) = *b {
            if let Expr::Num(0.0) = *a {
                panic!("Error: 0**0");
            }
            return Expr::Num(1.0);
        }
        return Expr::Pow(a, b);
    }

    expr
}

fn collect_coeffs(expr: Expr) -> (f64, Expr) {
    // dbg!("Collect coeffs: ", expr.clone());
    if let Expr::Prod(lhs, rhs) = expr.clone() {
        if let Expr::Num(k) = *lhs {
            let mut ret = collect_coeffs((*rhs).clone());
            ret.0 *= k;
            return ret;
        }
        if let Expr::Num(k) = *rhs {
            let mut ret = collect_coeffs((*lhs).clone());
            ret.0 *= k;
            return ret;
        }
    }
    (1.0, expr)
}

fn collect_sums(expr: Expr) -> (f64, Expr) {
    // dbg!("Collect sums: ", expr.clone());
    if let Expr::Sum(lhs, rhs) = expr.clone() {
        if let Expr::Num(k) = *lhs {
            let mut ret = collect_sums((*rhs).clone());
            ret.0 += k;
            return ret;
        }
        if let Expr::Num(k) = *rhs {
            let mut ret = collect_sums((*lhs).clone());
            ret.0 += k;
            return ret;
        }
    }
    // same but for sub
    if let Expr::Sub(lhs, rhs) = expr.clone() {
        if let Expr::Num(k) = *lhs {
            let mut ret = collect_sums((*rhs).clone());
            ret.0 -= k;
            return ret;
        }
        if let Expr::Num(k) = *rhs {
            let mut ret = collect_sums((*lhs).clone());
            ret.0 -= k;
            return ret;
        }
    }

    (0.0, expr)
}

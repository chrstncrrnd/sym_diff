use crate::parser::Expr;

pub fn simplify(expr: Expr) -> Expr{
    let (coeff, base_expr) = collect_coeffs(expr);
    Expr::Prod(Box::new(Expr::Num(coeff)), Box::new(base_expr))
}


fn collect_coeffs(expr: Expr) -> (f64, Expr){
    if let Expr::Prod(lhs, rhs) = expr.clone(){
        if let Expr::Num(k) = *lhs{
            let mut ret = collect_coeffs(*rhs);
            ret.0 *= k;
            return ret;
        }
        if let Expr::Num(k) = *rhs{
            let mut ret = collect_coeffs(*lhs);
            ret.0 *= k;
            return ret;
        }
    }

    (1.0, expr)

}

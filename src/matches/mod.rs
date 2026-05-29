// Interface:
// try_match!(expression, k: num, u: expr; Prod(k, u) => Prod(k, diff(u)))
//     => matches Prod(Num, Expr) => Prod(Num, diff(Expr))

#[macro_export]
macro_rules! gen_rule {
    ($rule_name:ident; $pat1:pat => $pat2:expr) => {
        fn $rule_name(expr: Expr) -> Option<Expr> {
            if let $pat1 = expr
            {
                return Some($pat2)
            }
            None
        }


    };
}

#[macro_export]
macro_rules! try_apply {
    ($rule_name:ident, $expression:ident) => {
        if let Some(ret) = $rule_name($expression.clone()){
            return Some(ret);
        }
    }
}


// if _matches_expr(input, ku)
macro_rules! _matches_expr {
    ($subject:stmt, $pattern:ident) => {
        
    };
}

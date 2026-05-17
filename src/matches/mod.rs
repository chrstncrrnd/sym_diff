// Interface:
// try_match!(expression, k: num, u: expr; Prod(k, u) => Prod(k, diff(u)))
//     => matches Prod(Num, Expr) => Prod(Num, diff(Expr))

#[macro_export]
macro_rules! try_match {
    (let $ident1:ident : $type1:ty, $ident2:ident, $type2:ty; $inp:stmt => $out:stmt) => {

    };
}



// if _matches_expr(input, ku)
macro_rules! _matches_expr {
    ($subject:stmt, $pattern:ident) => {
        
    };
}

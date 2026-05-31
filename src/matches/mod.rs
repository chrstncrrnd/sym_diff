// Interface:
// try_match!(expression, k: num, u: expr; Prod(k, u) => Prod(k, diff(u)))
//     => matches Prod(Num, Expr) => Prod(Num, diff(Expr))

#[macro_export]
macro_rules! gen_rule {
    ($rule_name:ident; $head_pat:pat $(, $p:pat = $e:expr )* => @no_some $out_pat:expr) => {
        fn $rule_name(expr: Expr) -> Option<Expr> {
            gen_rule!(@nest let $head_pat = expr $(, let $p = $e )* => return $out_pat);
            None
        }
    };

    ($rule_name:ident; $head_pat:pat $(, $p:pat = $e:expr )* => $out_pat:expr) => {
        fn $rule_name(expr: Expr) -> Option<Expr> {
            gen_rule!(@nest let $head_pat = expr $(, let $p = $e )* => return Some($out_pat));
            None
        }
    };

    (@nest let $head_pat:pat = $head_expr:expr, $( let $tail_pat:pat = $tail_e:expr),* => $out_pat:expr) => {
        if let $head_pat = $head_expr {
            gen_rule!(@nest $( let $tail_pat = $tail_e),* => $out_pat)
        }
    };

    (@nest let $last_pat:pat = $last_e:expr => $body:expr) => {
        if let $last_pat = $last_e {
            $body;
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



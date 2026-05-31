#[macro_export]
macro_rules! gen_rule {
    // @no_some means that the macro caller handles if optionalness of the return value
    ($rule_name:ident; $head_pat:pat $(, $p:pat = $e:expr )* => @no_some $out_pat:expr) => {
        fn $rule_name(expr: Expr) -> Option<Expr> {
            gen_rule!(@nest let $head_pat = expr $(, let $p = $e )* => return $out_pat);
            None
        }
    };

    // head (out is always some)
    ($rule_name:ident; $head_pat:pat $(, $p:pat = $e:expr )* => $out_pat:expr) => {
        fn $rule_name(expr: Expr) -> Option<Expr> {
            gen_rule!(@nest let $head_pat = expr $(, let $p = $e )* => return Some($out_pat));
            None
        }
    };

    // recursive caller
    (@nest let $head_pat:pat = $head_expr:expr, $( let $tail_pat:pat = $tail_e:expr),* => $out_pat:expr) => {
        if let $head_pat = $head_expr {
            gen_rule!(@nest $( let $tail_pat = $tail_e),* => $out_pat)
        }
    };

    // base case
    (@nest let $last_pat:pat = $last_e:expr => $body:expr) => {
        if let $last_pat = $last_e {
            $body;
        }
    };
}

#[macro_export]
macro_rules! try_apply {
    ($rule_name:ident, $expression:ident) => {
        if let Some(ret) = $rule_name($expression.clone()) {
            return Some(ret);
        }
    };
}

#[macro_export]
macro_rules! try_apply_all {
    ($( $rule_name:ident ),+ on $expression:ident) => {
        $(try_apply!($rule_name, $expression));+
    };
}

// macros to simplify binary operations
#[macro_export]
macro_rules! sum {
    ($lhs:expr, $rhs:expr) => {
        Expr::Sum(Box::new($lhs), Box::new($rhs))
    };
}

#[macro_export]
macro_rules! sub {
    ($lhs:expr, $rhs:expr) => {
        Expr::Sub(Box::new($lhs), Box::new($rhs))
    };
}

#[macro_export]
macro_rules! prod {
    ($lhs:expr, $rhs:expr) => {
        Expr::Prod(Box::new($lhs), Box::new($rhs))
    };
}

#[macro_export]
macro_rules! pow {
    ($lhs:expr, $rhs:expr) => {
        Expr::Pow(Box::new($lhs), Box::new($rhs))
    };
}

#[macro_export]
macro_rules! div {
    ($lhs:expr, $rhs:expr) => {
        Expr::Div(Box::new($lhs), Box::new($rhs))
    };
}

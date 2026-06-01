/// # Rule generation
/// usage: `gen_rule!(rule_name, precondition, consequent)`
/// generates a function named `rule_name` which takes an expression as an argument,
/// if it matches `precondition` it produces `consequent`
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

/// Tries to match an expression to a rule and returns the return value of said expression
/// if it is not None
#[macro_export]
macro_rules! try_apply {
    ($rule_name:ident, $expression:ident) => {
        if let Some(ret) = $rule_name($expression.clone()) {
            return Some(ret);
        }
    };
}

/// A wrapper around try_apply!(). Takes a list of funcitons and expands try_apply! for each of them
/// on the given expression
#[macro_export]
macro_rules! try_apply_all {
    ($( $rule_name:ident ),+ on $expression:ident) => {
        $(try_apply!($rule_name, $expression));+
    };
}

// macros to simplify binary operations
#[macro_export]
macro_rules! sum {
    ($lhs:expr, $( $rhs:expr ),+) => {
        Expr::Sum(Rc::new($lhs),
            Rc::new(sum!($( $rhs ),+))
        )
    };

    ($base:expr) => {
        $base
    }
}

#[macro_export]
macro_rules! sub {
    ($lhs:expr, $rhs:expr) => {
        Expr::Sub(Rc::new($lhs), Rc::new($rhs))
    };
}

#[macro_export]
macro_rules! prod {
    ($lhs:expr, $( $rhs:expr ),+) => {
        Expr::Prod(Rc::new($lhs),
            Rc::new(prod!($( $rhs ),+))
        )
    };

    ($base:expr) => {
        $base
    }

}

#[macro_export]
macro_rules! pow {
    ($lhs:expr, $rhs:expr) => {
        Expr::Pow(Rc::new($lhs), Rc::new($rhs))
    };
}

#[macro_export]
macro_rules! div {
    ($lhs:expr, $rhs:expr) => {
        Expr::Div(Rc::new($lhs), Rc::new($rhs))
    };
}

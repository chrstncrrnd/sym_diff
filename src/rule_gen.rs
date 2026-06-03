/// # Rule generation
/// usage: `gen_rule!(rule_name, precondition, consequent)`
/// generates a function named `rule_name` which takes an expression as an argument,
/// if it matches `precondition` it produces `consequent`
#[macro_export]
macro_rules! gen_rule {
    // function definition
    ($rule_name:ident; $($others:tt)*) => {
        fn $rule_name(expr: Expr) -> Option<Expr> {
            gen_rule!(@begin expr; $($others)*);
            None
        }
    };

    // we must take $val as a paramater (which is expr) since otherwise we get an error
    // we mark this with begin so that we can only match the head once
    // start of precondition
    (@begin $val:ident; $head:pat, $($others:tt)*) => {
        if let $head = $val{
            gen_rule!(@nest $($others)*)
        }
    };
    // end of precondition
    (@begin $val:ident; $head:pat => $($others:tt)*) => {
        if let $head = $val{
            gen_rule!(@end $($others)*);
        }
    };

    // inside precondition (if let case)
    (@nest $head_pat:pat = $head_expr:expr, $($others:tt)*) => {
        if let $head_pat = $head_expr {
            gen_rule!(@nest $($others)*);
        }
    };

    // end of precondition (if let case)
    (@nest $head_pat:pat = $head_expr:expr => $($others:tt)*) => {
        if let $head_pat = $head_expr {
            gen_rule!(@end $($others)*);
        }
    };

    // inside precondition (boolean case)
    (@nest $head_ident:ident == $head_lit:tt, $($others:tt)*) => {
        if $head_ident == $head_lit {
            gen_rule!(@nest $($others)*);
        }
    };


    // end of precondition (boolean case)
    (@nest $head_ident:ident == $head_lit:tt => $($others:tt)*) => {
        if $head_ident == $head_lit {
            gen_rule!(@end $($others)*);
        }
    };

    // we mark this with end since we have already consumed the =>
    // base case
    (@end $out_pat:expr) => {
        return Some($out_pat);
    };
    // @no_some means that the macro caller handles if optionalness of the return value
    (@end @no_some $out_pat:expr) => {
        return $out_pat;
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

    (@no_some $rule_name:ident, $expression:ident) => {
        if let Some(ret) = $rule_name($expression.clone()) {
            return ret;
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

    (@no_some $( $rule_name:ident ),+ on $expression:ident) => {
        $(try_apply!(@no_some $rule_name, $expression));+
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

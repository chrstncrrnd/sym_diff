/// # Rule generation
/// usage: `gen_rule!(rule_name; preconditions, consequent)`
/// generates a function named `rule_name` which takes an expression as an argument,
/// if it matches the `preconditions` it produces `consequent`
/// The preconditions are seperated by commas, the first precondition is matched to the
/// expression passed into the function and the following parts are matched as specified.
/// Preconditions are either "if-let" type or "boolean" type. An "if-let" type has only one equals
/// meanwhile the boolean type has two equals. The first precondition is always of "if-let" type.
/// ## Example:
/// `gen_rule!(test_rule; Expr::Sub(a, b), a == b => Expr::Num(0.0));`
/// In this case, the precondition has the two parts: `Expr::Sub(a, b)` and `a == b`.
/// We check that the expression, `expr`, passed into `test_rule(expr)` matches `Expr::Sub(a, b)`
/// then we check that a and b are equal via `a == b`.
#[macro_export]
macro_rules! gen_rule {
    // function definition
    ($rule_name:ident; $($others:tt)*) => {
        fn $rule_name(expr: Expr) -> Option<Expr> {
            gen_rule!(@begin expr; $($others)*);
            None
        }
    };

    // Function definition if we want to specify a custom return type.
    // The return type, of course, must be optional
    ($rule_name:ident -> $ret_ty:ty; $($others:tt)*) => {
        fn $rule_name(expr: Expr) -> $ret_ty {
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

    // @no_return means that the macro caller must be a panic for instance
    (@end @no_return $out_pat:expr) => {
        $out_pat;
    };
}

/// Tries to match an expression to a rule and returns the return value of said expression
/// if it is not None
#[macro_export]
macro_rules! try_apply {
    ($rule_name:ident, $expression:ident) => {
        if let Some(ret) = $rule_name($expression.clone()) {
            // eprintln!("Applied rule: {}", stringify!($rule_name));
            return Some(ret);
        }
    };

    (@no_some $rule_name:ident, $expression:ident) => {
        if let Some(ret) = $rule_name($expression.clone()) {
            // eprintln!("Applied rule: {}", stringify!($rule_name));
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

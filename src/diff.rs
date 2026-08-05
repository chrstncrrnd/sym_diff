use crate::functions::Func;
use crate::parser::Expr;
use crate::{div, gen_rule, pow, prod, sub, sum, try_apply, try_apply_all};
use std::rc::Rc;

gen_rule!(var_rule; Expr::Var => Expr::Num(1.0));

gen_rule!(const_rule; Expr::Num(_) => Expr::Num(0_f64));

gen_rule!(prod_linearity_left; Expr::Prod(ea, eb), Expr::Num(k) = *ea =>
    @no_some diff((*eb).clone()).map(|rhs| prod!(rhs, Expr::Num(k)))
);

gen_rule!(prod_linearity_right; Expr::Prod(ea, eb), Expr::Num(k) = *eb =>
    @no_some diff((*ea).clone()).map(|rhs| prod!(rhs, Expr::Num(k)))
);

gen_rule!(sum_linearity; Expr::Sum(ea, eb), Some(lhs) = diff((*ea).clone()), Some(rhs) = diff((*eb).clone()) => sum!(lhs, rhs));

gen_rule!(sub_linearity; Expr::Sub(ea, eb), Some(lhs) = diff((*ea).clone()), Some(rhs) = diff((*eb).clone()) => sub!(lhs, rhs));

gen_rule!(exponent; Expr::Pow(e, pow), Expr::E = *e, Some(d_rhs) = diff((*pow).clone()) => 
    prod!(d_rhs, Expr::Pow(e, pow)));

gen_rule!(expr_pow_num; Expr::Pow(base, exp), Expr::Num(p) = *exp, Some(u_prime) = diff((*base).clone())
    =>
    prod!(
        Expr::Num(p),
        u_prime,
        pow!(
            (*base).clone(),
            Expr::Num(p-1.0)
        )
    )
);


gen_rule!(num_pow_expr; Expr::Pow(a, exp), Expr::Num(k) = *a, Some(u_prime) = diff((*exp).clone()) => 
    prod!(
        Expr::F(Func::Log, Rc::new(Expr::Num(k))),
        Expr::Pow(a, exp),
        u_prime
    )
);

// this is a massive equation
gen_rule!(expr_pow_expr; Expr::Pow(f, g),
    Some(f_prime) = diff((*f).clone()),
    Some(g_prime) = diff((*g).clone())
    => prod!(
        Expr::Pow(f.clone(), g.clone()),
        sum!(
            prod!(
                g_prime,
                Expr::F(Func::Log, f.clone())
            ),
            prod!(
                (*g).clone(),
                div!(
                    f_prime,
                    (*f).clone()
                )
            )

        )
    )
);

gen_rule!(product_rule; Expr::Prod(a, b), Some(a_prime) = diff((*a).clone()), Some(b_prime) = diff((*b).clone()) =>
    sum!(
        prod!((*a).clone(), b_prime),
        prod!((*b).clone(), a_prime)
    )
);

gen_rule!(quotient_rule; Expr::Div(u, v), Some(u_prime) = diff((*u).clone()), Some(v_prime) = diff((*v).clone()) =>
    div!(
        sub!(
            prod!(
                (*v).clone(),
                u_prime
            ),
            prod!(
                (*u).clone(),
                v_prime
            )
        ),
        pow!(
            (*v).clone(),
            Expr::Num(2_f64)
        )
    )
);

gen_rule!(sine_rule; Expr::F(Func::Sin, arg), Some(arg_prime) = diff((*arg).clone()) =>
    prod!(arg_prime, Expr::F(Func::Cos, arg))
);

gen_rule!(cosine_rule; Expr::F(Func::Cos, arg), Some(arg_prime) = diff((*arg).clone()) =>
    prod!(Expr::Num(-1.0), arg_prime, Expr::F(Func::Sin, arg))
);

gen_rule!(log_rule; Expr::F(Func::Log, arg), Some(arg_prime) = diff((*arg).clone()) =>
    div!(
        arg_prime,
        (*arg).clone()
    )
);

gen_rule!(tan_rule; Expr::F(Func::Tan, arg), Some(arg_prime) = diff((*arg).clone()) =>
    prod!(
        arg_prime,
        pow!(
            Expr::F(Func::Sec, arg.clone()),
            Expr::Num(2.0)
        )
    )
);

gen_rule!(sec_rule; Expr::F(Func::Sec, arg), Some(arg_prime) = diff((*arg).clone()) =>
    prod!(
        arg_prime,
        Expr::F(Func::Sec, arg.clone()),
        Expr::F(Func::Tan, arg.clone())
    )
);

gen_rule!(cosec_rule; Expr::F(Func::Cosec, arg), Some(arg_prime) = diff((*arg).clone()) =>
    prod!(
        Expr::Num(-1.0),
        arg_prime,
        Expr::F(Func::Cosec, arg.clone()),
        Expr::F(Func::Cotan, arg.clone())
    )

);

gen_rule!(cotan_rule; Expr::F(Func::Cotan, arg), Some(arg_prime) = diff((*arg).clone()) =>
    prod!(
        Expr::Num(-1.0),
        arg_prime,
        pow!(
            Expr::F(Func::Cosec, arg),
            Expr::Num(2.0)
        )
    )
);

gen_rule!(const_eval_sum; Expr::Sum(ea, eb), Expr::Num(a) = *ea, Expr::Num(b) = *eb =>
    Expr::Num(a + b)
);

gen_rule!(const_eval_prod; Expr::Prod(ea, eb), Expr::Num(a) = *ea, Expr::Num(b) = *eb =>
    Expr::Num(a * b)
);

pub fn diff(expr: Expr) -> Option<Expr> {
    try_apply_all!(
        const_eval_sum,
        const_eval_prod,
        var_rule,
        const_rule,
        prod_linearity_left,
        prod_linearity_right,
        sum_linearity,
        exponent,
        expr_pow_num,
        num_pow_expr,
        expr_pow_expr,
        sub_linearity,
        product_rule,
        quotient_rule,
        sine_rule,
        cosine_rule,
        log_rule,
        tan_rule,
        sec_rule,
        cosec_rule,
        cotan_rule
        on expr);
    None
}

// really basic tests to make sure the diff works as intended.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parser::parse, simpl::simplify, tokenizer::Lexer};

    fn diff_str(s: &str) -> Expr {
        let expr = parse(Lexer::new(s.chars().peekable())).expect("expected valid expression");
        simplify(diff(expr).expect("expected differentiable expression"))
    }

    #[test]
    fn diff_var() {
        assert_eq!(diff_str("x"), Expr::Num(1.0));
    }

    #[test]
    fn diff_var_pow() {
        assert_eq!(diff_str("x**3").to_string(), "3x ** 2");
    }

    #[test]
    fn const_eval(){
        assert_eq!(diff_str("10 + 2").to_string(), "12");
    }
    
    #[test]
    fn const_eval_in_exponent(){
        assert_eq!(diff_str("x**(1+2)").to_string(),"3x ** 2");
    }
}



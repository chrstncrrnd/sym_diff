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

gen_rule!(pow_rule; Expr::Pow(base, exp), Expr::Num(p) = *exp, Some(u_prime) = diff((*base).clone())
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

pub fn diff(expr: Expr) -> Option<Expr> {
    try_apply_all!(
        var_rule,
        const_rule,
        prod_linearity_left,
        prod_linearity_right,
        sum_linearity,
        pow_rule,
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

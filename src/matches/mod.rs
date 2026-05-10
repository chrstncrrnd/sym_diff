// Interface:
// gen_rule!(k: num, u: expr, ku => k u')
// try_rule!(k: num, u: expr, ku => k u')

#[macro_export]
macro_rules! gen_rule {
    (let $($ident:ident : $type:ty),*; $stmt:stmt => $out:stmt) => {};
}

# Rust Symbolic Differentiation 


A simple rust program that symbolically differentiates a given expression. This is primarily
a meta-programming exercise since rules are generated and applied using macros, read docs in `rule_gen.rs` for more information.


# Usage
In order to compile/run this program you must have [rust](https://rust-lang.org) installed.

Running:

```sh
git clone https://github.com/chrstncrrnd/sym_diff
cd sym_diff
cargo run
```

## Roadmap:
- [x] ~Tokenizer~
- [x] ~Parser~
    - [x] ~Allow syntax such as `10x` (implicit multiplication)~
    - [x] ~Brackets parsing~
    - [x] ~Implicit multiplication on brackets `10(x+3)`~
- [x] ~Expression to text~
    - [x] ~In a nice way~
    - [ ] Update product output since `10x * 2 => 10x2` right now.
- [x] ~Basic functions~
    - [ ] Parser: allow for `sin 10 x` => `sin(10(x))`
    - [ ] Tokenizer: parse text differently such that `sinx` => `sin(x)` works
- [ ] Differentiation rules (Chain, product, etc...)
    - [x] ~Power rule~
    - [x] ~Linearity~
    - [x] ~Constants~
    - [x] ~Product rule~
    - [x] ~Quotient rule~
    - [x] ~Function rules~
        - [x] ~Sin~
        - [x] ~Cos~
        - [x] ~Tan~
        - [x] ~Sec~
        - [x] ~Cosec~
        - [x] ~Cotan~
        - [x] ~Log~
- [x] ~Basic simplification of expressions~
    - [x] ~`EXPR + 0` => `EXPR`~
    - [x] ~`EXPR * 1` => `EXPR`~
    - [x] ~`EXPR ** 1` => `EXPR`~
    - [x] ~`EXPR ** 0` => `EXPR` (`EXPR` != 0)~
    - [x] ~`6(2(EXPR))` => `12(EXPR)`~
- [ ] Differentiation strategies
- [x] ~Differentiation macros~
- [ ] LaTeX support for expression output
- [ ] LaTeX support for expression input
- [ ] Add tests
- [ ] Advanced simplification of expressions:
    - [ ] Trigonometric simplifications
    - [ ] Division using GCD (not just of numbers though)

## Errors to fix:
- [x] ~There should be a syntax error for `10 x *` (missing operand)~
- [x] ~There should be a syntax error for `10 * (x++3)` (double operator)~
- [ ] Find some way of properly parsing `(x)**2`.
- [ ] `10 x ** (1+2)` raises an error.

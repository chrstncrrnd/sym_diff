# Rust Symbolic Differentiation 


A simple rust program that symbolically differentiates a given expression.


## Roadmap:
- [x] ~Tokenizer~
- [x] ~Parser~
    - [x] ~Allow syntax such as `10x` (implicit multiplication)~
    - [x] ~Brackets parsing~
    - [x] ~Implicit multiplication on brackets `10(x+3)`~
- [x] ~Expression to text~
    - [ ] In a nice way
- [x] ~Basic functions~
    - [ ] Parser: allow for `sin 10 x` => `sin(10(x))`
    - [ ] Tokenizer: parse text differently such that `sinx` => `sin(x)` works
- [ ] Differentiation rules (Chain, product, etc...)
    - [x] ~Power rule~
    - [x] ~Linearity~
    - [x] ~Constants~
    - [ ] Product rule
    - [ ] Quotient rule
    - [ ] Function rules
- [ ] Simplification of expressions
    - [ ] `EXPR + 0` => `EXPR`
    - [ ] `EXPR * 1` => `EXPR`
    - [ ] `EXPR ** 1` => `EXPR`
    - [ ] `EXPR ** 0` => `EXPR` (`EXPR` != 0)
    - [ ] `6(2(EXPR))` => `12(EXPR)`
- [ ] Differentiation strategies
- [ ] Differentiation DSL
- [ ] LaTeX support for expression output
- [ ] LaTeX support for expression input
- [ ] Add tests

## Errors to fix:
- [x] ~There should be a syntax error for `10 x *` (missing operand)~
- [x] ~There should be a syntax error for `10 * (x++3)` (double operator)~
- [ ] Find some way of properly parsing `(x)**2`

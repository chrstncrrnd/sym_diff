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
- [ ] Add tests

## Errors to fix:
- [x] ~There should be a syntax error for `10 x *` (missing operand)~
- [x] ~There should be a syntax error for `10 * (x++3)` (double operator)~

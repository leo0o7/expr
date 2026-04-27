# expr

A small hand-written expression evaluator based on Dijkstra's shunting-yard algorithm.
It reads mathematical expressions in normal infix form and evaluates them directly using an operator stack and a value stack.
It does not first build an expression tree or convert the expression into another format.

It supports precedence, associativity, brackets, signed numbers, decimals, and implicit multiplication like `2(3 + 4)`.

## Usage

It reads one expression from stdin and prints the result:

```sh
echo "1 + 2 * (3 + 4)" | cargo run
```

Example output:

```sh
= 15
```

## Syntax support

- [x] Addition: `1 + 2`
- [x] Subtraction: `5 - 2`
- [x] Multiplication: `3 * 4`
- [x] Division: `8 / 2`
- [x] Exponentiation: `2 ^ 3`
- [x] Signs: `-2`, `+3`
- [x] Decimals: `1.5 + .5`
- [x] Brackets: `(1 + 2)`, `[1 + 2]`, `{1 + 2}`
- [x] Implicit multiplication: `2(3 + 4)`
- [ ] Scientific notation: `1e3`
- [ ] Functions: `sin(1)`, `sqrt(4)`
- [ ] Variables: `x + 1`
- [ ] Constants: `pi`, `e`

## Tests

Run tests:

```sh
cargo test
```

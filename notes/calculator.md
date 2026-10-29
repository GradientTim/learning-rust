# Calculator notes (`src/bin/calc.rs`)

## Status

- Tokenizer works, including multi-digit numbers (`peekable()` + `peek()`).
- Parser works left to right, so `1 + 2 + 3 * 5` gives `30` instead of `18`.
- Next: operator precedence (`*` and `/` before `+` and `-`).

## The core idea: terms

```
1   +   2   +   3 * 5
└┘      └┘      └───┘
term    term    term
```

- `+` and `-` **separate** terms.
- `*` and `/` **stay inside** a term.
- Finish each term first (all multiplying and dividing), then add the terms together.

## How to change the loop

Split the single running value into two variables:

- `result`: the sum of all **finished** terms
- `term`: the term currently being **built**

For each number, ask: does it **continue** the current term or **start a new one**?

- Last operator was `*` or `/`: change `term` with it (the term continues).
- Last operator was `+` or `-`: the old term is finished. Add it to `result`, then start a new `term` with this number.
- When the loop ends, one term is still unfinished. Don't forget it!

Check on paper: `1 + 2 + 3 * 5` must give `18`.

## Gotcha: subtraction

`10 - 2 * 3` should be `4`.

If the new term after `-` starts as just `2`, you get `2 * 3 = 6`, but how do you know to
subtract it instead of adding it? Two possible ways:

- remember the sign of the term, or
- store the number so that it already includes the minus.

Which one feels simpler?

## Optional improvements

- Rename `Token::Digit` to `Token::Number`, since it now holds whole numbers like `345`.
- The `_ => {}` in the inner match only exists because `last_operator` is a `Token` and could
  in theory be a `Digit`. A separate `enum Operator { Add, Sub, Mul, Div }` used as
  `Token::Operator(Operator)` makes that state impossible
  ("make invalid states unrepresentable").

## Milestones

1. [x] `"42"` gives 42 (single, multi-digit number)
2. [x] Tokenizer prints the right tokens for `"12 + 3 * 5"`
3. [x] `"1 + 2 + 3"` gives 6 (only `+` / `-`, left to right)
4. [ ] `"1 + 2 + 3 * 5"` gives 18 (precedence) <- **you are here**
5. [ ] `"(1 + 2) * 3"` gives 9 (parentheses)
6. [ ] Errors: `""`, `"1 +"`, `"5 / 0"` with `Result<i64, SomeError>`

## Looking ahead: parentheses

The two-variable trick works for two levels of precedence. For parentheses you need the
recursive version: `parse_expression` calls `parse_term`, which calls `parse_number`.
It's the same idea as the two variables, written as functions.
A `(...)` is then just a full expression in the spot where a number would be.
Search term if stuck: **recursive descent parsing**.

## Testing tip

Add tests at the bottom of `calc.rs` and run them with `cargo test --bin calc`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precedence() {
        assert_eq!(calculate("1 + 2 + 3 * 5"), 18);
    }
}
```

# Lexx

Lexx is a fast, extensible, greedy, single-pass text tokenizer written in Rust.

## Sample Output

For the string `"This is  \n1.0 thing."`:
```rust
use lexx::token::Token;
Token { token_type: 4, value: "This".to_string(), line: 1, column: 1, len: 4, precedence: 0 }
Token { token_type: 3, value: " ".to_string(), line: 1, column: 5, len: 1, precedence: 0 }
Token { token_type: 4, value: "is".to_string(), line: 1, column: 6, len: 2, precedence: 0 }
Token { token_type: 3, value: "  \n".to_string(), line: 1, column: 8, len: 3, precedence: 0 }
Token { token_type: 2, value: "1.0".to_string(), line: 2, column: 1, len: 3, precedence: 0 }
Token { token_type: 3, value: " ".to_string(), line: 2, column: 4, len: 1, precedence: 0 }
Token { token_type: 4, value: "thing".to_string(), line: 2, column: 5, len: 5, precedence: 0 }
Token { token_type: 5, value: ".".to_string(), line: 2, column: 10, len: 1, precedence: 0 }
```

## Structure

Lexx consists of 4 main components:
* `LexxInput` provides a stream of `char` characters
* `Matchers` identify parts of a string, such as integers or symbols
* `Token` is the result of a successful match
* `Lexx` itself, orchestrating the tokenization

## Functionality

Lexx uses a [`LexxInput`](crate::input::LexxInput) to provide chars that are fed to
[`Matcher`](crate::matcher::Matcher) instances until the longest match is found, if any. The
match will be returned as a [`Token`](token::Token) instance, which includes the token type, matched string, and line/column information. Custom [`LexxInput`](crate::input::LexxInput) implementations are supported; built-in options include [`InputString`](crate::input::InputString) and [`InputReader`](crate::input::InputReader).

Lexx implements [`Iterator`], so you can use it directly in a `for` loop.

Custom [`Matcher`](crate::matcher::Matcher)s can also be created. Lexx provides:
- [`WordMatcher`](crate::matcher_word::WordMatcher): matches alphabetic words
- [`IntegerMatcher`](crate::matcher_integer::IntegerMatcher): matches integers
- [`FloatMatcher`](crate::matcher_float::FloatMatcher): matches floats
- [`ExactMatcher`](crate::matcher_exact::ExactMatcher): matches exactly specified strings (e.g., operators or delimiters)
- [`SymbolMatcher`](crate::matcher_symbol::SymbolMatcher): matches all non-alphanumerics
- [`KeywordMatcher`](crate::matcher_keyword::KeywordMatcher): matches specific words, but not substrings
- [`WhitespaceMatcher`](crate::matcher_whitespace::WhitespaceMatcher): matches whitespace (spaces, tabs, newlines)

Matchers can be given precedence, allowing a matcher to return its result even if another matcher has a longer match. For example, both the [`WordMatcher`](crate::matcher_word::WordMatcher) and [`KeywordMatcher`](crate::matcher_keyword::KeywordMatcher) can be used at the same time.

Note: Matchers cannot find matches that start inside the valid matches of other matchers. For example, if matching `renewable`, the [`WordMatcher`](crate::matcher_word::WordMatcher) will consume the whole word, even if [`ExactMatcher`](crate::matcher_exact::ExactMatcher) is looking for `new` with higher precedence.

To successfully parse an entire stream, Lexx must have a matcher capable of tokenizing every encountered collection of characters. If a match fails, Lexx will return `Err(TokenNotFound)` with the unmatched text.

## Performance

Lexx is highly optimized for speed:
- On a large file (e.g., `Varney-the-Vampire.txt`, ~1.8MB), Lexx tokenizes the entire file in **~690 microseconds** per run (mean and median), with very low variance.
- Benchmarks show stable and predictable performance, suitable for high-throughput or interactive use cases.

### Running Benchmarks

To run the benchmarks and measure performance:
```sh
cargo bench
```
Benchmark results (including mean and median timings) will be output in the `target/criterion` directory.

## Panics

For speed, Lexx does not dynamically allocate buffer space. In `Lexx<CAP>`, `CAP` is the maximum possible token size. If that size is exceeded, a panic will be thrown.

## License

See [LICENSE](LICENSE).

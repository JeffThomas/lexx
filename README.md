# lexx

A fast, extensible, greedy, single-pass text tokenizer implemented in Rust. It uses ArrayVec for more efficient memory management.

## Overview

`lexx` is a fast and flexible tokenizer library that allows you to define and compose various token matching strategies. The library is designed to be easy to use while maintaining high performance.

## Features

- Single-pass tokenization
- No memory allocation during tokenization
- Composable matcher system
- Helper methods for token inspection
- Factory functions for common tokenizer configurations
- Builder pattern for easy setup

## Usage

Add `lexx` to your `Cargo.toml`:

```toml
[dependencies]
lexx = "0.1.0"
```

### Basic Example

```rust
use lexx::{factories, Lexxer};
use lexx::token::{TOKEN_TYPE_WORD, TOKEN_TYPE_WHITESPACE, TOKEN_TYPE_SYMBOL};

// Create a tokenizer for a code-like syntax
let mut lexer = factories::tokenize_code("if (x > 3.14) { return true; }");

// Iterate through tokens
for token_result in lexer {
    match token_result {
        Ok(Some(token)) => {
            println!("Token: {}, Type: {}, Line: {}, Column: {}", 
                      token.value, token.token_type, token.line, token.column);
        },
        Ok(None) => println!("End of input"),
        Err(e) => println!("Error: {:?}", e),
    }
}
```

### Using Token Helper Methods

```rust
use lexx::{factories, Lexxer};

let mut lexer = factories::tokenize_str("Hello 42 world!");

while let Ok(Some(token)) = lexer.next_token() {
    if token.is_word() {
        println!("Found word: {}", token.value);
    } else if token.is_integer() {
        println!("Found number: {}", token.value);
    } else if token.is_whitespace() {
        println!("Found whitespace");
    }
}
```

### Custom Tokenizer with Builder Pattern

```rust
use lexx::{Lexx, Lexxer, input::InputString};
use lexx::matcher::word::WordMatcher;
use lexx::matcher::whitespace::WhitespaceMatcher;
use lexx::matcher::symbol::SymbolMatcher;
use lexx::matcher::exact::ExactMatcher;
use lexx::token::TOKEN_TYPE_KEYWORD;

let input = InputString::new("let x = 10;".to_string());
let mut lexer = Lexx::<512>::new(Box::new(input), vec![])
    .with_matcher(Box::new(WhitespaceMatcher::new()))
    .with_matcher(Box::new(WordMatcher::new()))
    .with_matcher(Box::new(SymbolMatcher::new()))
    .with_matcher(Box::new(ExactMatcher::new("let", TOKEN_TYPE_KEYWORD)));

// Use the lexer...
```

### Collecting Tokens

```rust
use lexx::{factories, Lexxer};
use lexx::token::TOKEN_TYPE_SYMBOL;

let mut lexer = factories::tokenize_code("let x = 10; let y = 20;");

// Collect tokens until semicolon
match lexer.collect_until(TOKEN_TYPE_SYMBOL) {
    Ok(tokens) => {
        println!("First statement has {} tokens", tokens.len());
        
        // The semicolon is still in the stream
        if let Ok(Some(token)) = lexer.next_token() {
            println!("Found delimiter: {}", token.value);
        }
    },
    Err(e) => println!("Error: {:?}", e),
}
```

## Token Types

The library defines several token types:
- `TOKEN_TYPE_WHITESPACE` - Whitespace characters
- `TOKEN_TYPE_WORD` - Word tokens (a-z, A-Z, _)
- `TOKEN_TYPE_INTEGER` - Integer numbers
- `TOKEN_TYPE_FLOAT` - Floating point numbers
- `TOKEN_TYPE_SYMBOL` - Symbol characters
- `TOKEN_TYPE_KEYWORD` - Reserved keywords
- `TOKEN_TYPE_EXACT` - Exact string matches

## Documentation

For more detailed documentation, see the [API documentation](https://docs.rs/lexx).

## License

MIT License

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

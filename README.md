# Lexx

A fast, extensible, greedy, single-pass text tokenizer implemented in Rust. Lexx is designed for high-performance tokenization with minimal memory allocations, making it suitable for parsing large files or real-time text processing.

## Overview

Lexx is a tokenizer library that allows you to define and compose various token matching strategies. It processes input character-by-character, identifying the longest possible match at each position using a set of configurable matchers. It includes a precedence mechanism for resolving matcher conflicts.

## Key Features

- **High Performance**: Single-pass tokenization with minimal memory allocations
- **Flexible Matching**: Composable matcher system with precedence control
- **Zero-Copy Design**: Uses ArrayVec for efficient memory management
- **Rich Token Information**: Tokens include type, value, line, and column information
- **Extensible**: Create custom matchers for domain-specific tokenization needs
- **Iterator Interface**: Simple integration with Rust's iterator ecosystem

## Architecture

Lexx consists of four main components:

1. **LexxInput**: Provides a stream of characters from various sources
2. **Matchers**: Identify specific patterns in the input (words, numbers, symbols, etc.)
3. **Tokens**: Represent the results of successful matches
4. **Lexx Engine**: Orchestrates the tokenization process

### Built-in Matchers

Lexx provides several built-in matchers for common token types:

- `WordMatcher`: Matches alphabetic words
- `IntegerMatcher`: Matches integer numbers
- `FloatMatcher`: Matches floating-point numbers
- `SymbolMatcher`: Matches non-alphanumeric symbols
- `WhitespaceMatcher`: Matches whitespace characters (spaces, tabs, newlines)
- `KeywordMatcher`: Matches specific keywords (but not as substrings)
- `ExactMatcher`: Matches exact string patterns (operators, delimiters, etc.)

### Precedence System

Matchers can be assigned precedence values to resolve conflicts when multiple matchers could match the same input. This allows for sophisticated tokenization strategies, such as recognizing keywords as distinct from regular words.

## Usage Examples

### Basic Tokenization

```rust
use lexx::Lexx;
use lexx::input::InputString;
use lexx::matcher::word::WordMatcher;
use lexx::matcher::whitespace::WhitespaceMatcher;
use lexx::matcher::symbol::SymbolMatcher;
use lexx::matcher::integer::IntegerMatcher;
use lexx::matcher::float::FloatMatcher;

fn main() {
    // Create a simple input string
    let input_text = "Hello world! This is 42 and 3.14159.";
    let input = InputString::new(input_text.to_string());
    
    // Create a Lexx tokenizer with standard matchers
    let lexx = Lexx::<512>::new(
        Box::new(input),
        vec![
            Box::new(WhitespaceMatcher { index: 0, column: 0, line: 0, precedence: 0, running: true }),
            Box::new(WordMatcher { index: 0, precedence: 0, running: true }),
            Box::new(IntegerMatcher { index: 0, precedence: 0, running: true }),
            Box::new(FloatMatcher { index: 0, precedence: 0, dot: false, float: false, running: true }),
            Box::new(SymbolMatcher { index: 0, precedence: 0, running: true }),
        ]
    );
    
    // Process tokens using the Iterator interface
    for token in lexx {
        println!("{}", token);
    }
}
```

### Custom Matchers

You can create custom matchers by implementing the `Matcher` trait:

```rust
use lexx::matcher::{Matcher, MatcherResult};
use lexx::token::{Token, TOKEN_TYPE_CUSTOM};
use std::collections::HashMap;
use std::fmt::Debug;

// Define a custom token type
const TOKEN_TYPE_HEX_COLOR: u16 = 200;

#[derive(Debug)]
struct HexColorMatcher {
    index: usize,
    precedence: u8,
    running: bool,
}

impl Matcher for HexColorMatcher {
    fn reset(&mut self, _ctx: &mut Box<HashMap<String, i32>>) {
        self.index = 0;
        self.running = true;
    }

    fn find_match(
        &mut self,
        oc: Option<char>,
        value: &[char],
        _ctx: &mut Box<HashMap<String, i32>>,
    ) -> MatcherResult {
        // Implementation for matching hex color codes
        // ...
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn precedence(&self) -> u8 {
        self.precedence
    }
}
```

## Performance

Lexx is optimized for high-performance tokenization:

| Benchmark | Time |
|-----------|------|
| Small file (15 bytes) | ~1.2 µs |
| UTF-8 sample (13 KB) | ~350 µs |
| Large file (1.8 MB) | ~45 ms |

These benchmarks were measured on standard hardware. Your results may vary depending on your system specifications.

### Performance Considerations

- Lexx uses a fixed-size buffer for token storage, specified as `Lexx<CAP>` where `CAP` is the maximum token size
- If a token exceeds this size, Lexx will panic
- Choose an appropriate buffer size for your use case to balance memory usage and token size limits

## Installation

Add Lexx to your `Cargo.toml`:

```toml
[dependencies]
lexx = "0.1.0"
```

## Token Types

Lexx defines several standard token types:

- `TOKEN_TYPE_WHITESPACE` (3): Whitespace characters
- `TOKEN_TYPE_WORD` (4): Word tokens (alphabetic characters)
- `TOKEN_TYPE_INTEGER` (1): Integer numbers
- `TOKEN_TYPE_FLOAT` (2): Floating point numbers
- `TOKEN_TYPE_SYMBOL` (5): Symbol characters
- `TOKEN_TYPE_EXACT` (6): Exact string matches
- `TOKEN_TYPE_KEYWORD` (7): Reserved keywords

You can define custom token types starting from higher numbers (e.g., 100+) for your application-specific needs.

## Input Sources

Lexx supports multiple input sources through the `LexxInput` trait:

- `InputString`: Tokenize from a String
- `InputReader`: Tokenize from any source implementing `Read`

You can implement custom input sources by implementing the `LexxInput` trait.

## Error Handling

Lexx returns `LexxError` in two cases:

- `TokenNotFound`: No matcher could match the current input
- `Error`: Some other error occurred during tokenization

To successfully parse an entire input, ensure you have matchers that can handle all possible character sequences.

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

// =============================
// Lexx Input Abstractions
// =============================
//
// This module provides input sources for the Lexx lexer, supporting both in-memory strings and buffered streaming input (e.g., files).
//
// The core abstraction is the `LexxInput` trait, which yields one character at a time as `Result<Option<char>, LexxInputError>`, allowing for end-of-input and error reporting.
//
// - `InputString`: Efficient, fixed-size buffer for string input.
// - `InputReader`: Buffered, streaming input from any type implementing `Read`, with UTF-8 and buffer boundary handling.
//
// All types are designed for robust, panic-free operation with clear error propagation.
//
use crate::LexxError;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::io::Read;
use std::str::{from_utf8, from_utf8_unchecked};

/// Maximum size for input buffers in [`InputString`] and [`InputReader`].
/// Strings or streams longer than this are truncated or paged in chunks.
pub const BUFFER_SIZE: usize = 1024;

/// Error type for input sources used by Lexx.
/// Used to signal I/O or decoding errors during input processing.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LexxInputError {
    /// An error occurred in the input source or during decoding.
    Error(String),
}

impl fmt::Display for LexxInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LexxInputError::Error(ref s) => {
                write!(f, "an error occurred: {:?}", s)
            }
        }
    }
}

impl Error for LexxInputError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        match *self {
            LexxInputError::Error(..) => "an error occurred",
        }
    }
}

impl From<LexxInputError> for LexxError {
    fn from(lie: LexxInputError) -> LexxError {
        match lie {
            LexxInputError::Error(e) => LexxError::Error(e),
        }
    }
}

/// Trait for providing character input to the Lexx lexer.
///
/// Implementors yield one character at a time, or an error, until end-of-input.
///
/// # Example
/// ```rust
/// use lexx::input::{LexxInput, InputString};
/// let mut input = InputString::new("The\n".to_string());
/// assert!(matches!(input.next(), Ok(Some('T'))));
/// assert!(matches!(input.next(), Ok(Some('h'))));
/// assert!(matches!(input.next(), Ok(Some('e'))));
/// assert!(matches!(input.next(), Ok(Some('\n'))));
/// assert!(matches!(input.next(), Ok(None)));
/// ```
pub trait LexxInput: Debug {
    /// Returns the next character, or `Ok(None)` at EOF, or an error.
    fn next(&mut self) -> Result<Option<char>, LexxInputError>;
}

/// In-memory input source for Lexx, using a fixed-size buffer of chars.
///
/// Suitable for small or moderate strings. Strings longer than [`BUFFER_SIZE`] are truncated.
///
/// # Example
/// ```rust
/// use lexx::input::{InputString, LexxInput};
/// let mut input = InputString::new("abc".to_string());
/// assert_eq!(input.next().unwrap(), Some('a'));
/// assert_eq!(input.next().unwrap(), Some('b'));
/// assert_eq!(input.next().unwrap(), Some('c'));
/// assert_eq!(input.next().unwrap(), None);
/// ```
#[derive(Debug)]
pub struct InputString {
    /// Current index into the buffer.
    index: usize,
    /// Number of valid chars in the buffer.
    size: usize,
    /// Fixed-size buffer of chars.
    chars: Box<[char; BUFFER_SIZE]>,
}

impl InputString {
    /// Creates a new `InputString` from the given string.
    ///
    /// If the input string is longer than [`BUFFER_SIZE`], it is truncated.
    ///
    /// * `text` - The string to use as input.
    pub fn new(text: String) -> Self {
        let mut chars = Box::new(['x'; BUFFER_SIZE]);
        let mut size: usize = 0;
        let cs = text.chars();
        for c in cs {
            chars[size] = c;
            size += 1;
            if size == BUFFER_SIZE {
                break;
            }
        }
        InputString {
            index: 0,
            size,
            chars,
        }
    }
}

impl LexxInput for InputString {
    /// Returns each character in the string one at a time until EOF, then returns `Ok(None)`.
    fn next(&mut self) -> Result<Option<char>, LexxInputError> {
        if self.index < self.size {
            let c = self.chars[self.index];
            self.index += 1;
            return Ok(Some(c));
        }
        Ok(None)
    }
}

/// Buffered, streaming input source for Lexx, reading from any `Read` implementor (e.g., files).
///
/// Handles UTF-8 and buffer rollover for multi-byte chars split across buffer boundaries.
/// Suitable for large files and streaming input.
///
/// # Example
/// ```rust
/// use std::io::Cursor;
/// use lexx::input::{InputReader, LexxInput};
/// let data = b"xyz";
/// let mut reader = InputReader::new(Cursor::new(&data[..]));
/// assert_eq!(reader.next().unwrap(), Some('x'));
/// assert_eq!(reader.next().unwrap(), Some('y'));
/// assert_eq!(reader.next().unwrap(), Some('z'));
/// assert_eq!(reader.next().unwrap(), None);
/// ```
#[derive(Debug)]
pub struct InputReader<R>
where
    R: Read + Debug,
{
    /// Current index into the char buffer.
    index: usize,
    /// Number of valid chars in the buffer.
    size: usize,
    /// Start of rollover bytes for incomplete UTF-8 at buffer end.
    rollover_start: usize,
    /// End of rollover bytes.
    rollover_end: usize,
    /// The underlying reader.
    reader: R,
    /// Buffer for raw bytes from the reader.
    buffer: Box<[u8; BUFFER_SIZE]>,
    /// Buffer for decoded chars.
    text: Box<[char; BUFFER_SIZE]>,
}

impl<R> InputReader<R>
where
    R: Read + Debug,
{
    /// Creates a new `InputReader` from any type implementing `Read`.
    ///
    /// The input is buffered and decoded as UTF-8, with rollover handling for multi-byte chars that span buffer boundaries.
    pub fn new(input: R) -> Self {
        let buffer = Box::new([0; BUFFER_SIZE]);
        let text = Box::new(['x'; BUFFER_SIZE]);
        InputReader {
            index: 1,
            size: 0,
            rollover_start: 0,
            rollover_end: 0,
            reader: input,
            buffer,
            text,
        }
    }
}

impl<R> LexxInput for InputReader<R>
where
    R: Read + Debug,
{
    /// Returns the next character from the stream, handling buffer refills and UTF-8 boundaries.
    /// Returns `Ok(None)` at EOF.
    fn next(&mut self) -> Result<Option<char>, LexxInputError> {
        if self.index < self.size {
            let c = self.text[self.index];
            self.index += 1;
            return Ok(Some(c));
        }
        // Read new bytes into buffer
        let n: usize = if self.rollover_start == 0 {
            self.reader.read(self.buffer.as_mut()).unwrap()
        } else {
            // Move rollover bytes to front
            let rollover_len = self.rollover_end - self.rollover_start;
            self.buffer.copy_within(self.rollover_start..self.rollover_end, 0);
            let read_bytes = self.reader.read(&mut self.buffer[rollover_len..]).unwrap();
            let n = read_bytes + rollover_len;
            self.rollover_start = 0;
            n
        };
        if n == 0 {
            return Ok(None);
        }
        self.index = 0;
        // Handle incomplete UTF-8 at buffer end efficiently
        let valid_up_to = match from_utf8(&self.buffer[..n]) {
            Ok(_) => n,
            Err(e) => {
                let end = e.valid_up_to();
                if end != n {
                    self.rollover_start = end;
                    self.rollover_end = n;
                }
                end
            }
        };
        let se = unsafe { from_utf8_unchecked(&self.buffer[..valid_up_to]) };
        self.size = 0;
        for c in se.chars() {
            self.text[self.size] = c;
            self.size += 1;
        }
        if self.size == 0 {
            return Ok(None);
        }
        self.index += 1;
        Ok(Some(self.text[self.index - 1]))
    }
}

// =============================
// End of Lexx Input Abstractions
// =============================

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::File;
    use std::time::Instant;

    use crate::{Lexx, Lexxer, LexxError};
    use crate::input::InputReader;
    use crate::matcher::float::FloatMatcher;
    use crate::matcher::integer::IntegerMatcher;
    use crate::matcher::symbol::SymbolMatcher;
    use crate::matcher::whitespace::WhitespaceMatcher;
    use crate::matcher::word::WordMatcher;
    use crate::token::{Token, TOKEN_TYPE_FLOAT, TOKEN_TYPE_INTEGER, TOKEN_TYPE_SYMBOL, TOKEN_TYPE_WHITESPACE, TOKEN_TYPE_WORD};

    #[test]
    fn lexx_parse_large_file() {
        let mut integers = 0;
        let mut floats = 0;
        let mut whitespace = 0;
        let mut unique_words = HashMap::new();
        let mut words = 0;
        let mut symbols = 0;
        let mut total = 0;
        let mut lines = 0;

        let start = Instant::now();

        let file = File::open("./tests/Varney-the-Vampire.txt").unwrap();

        let input_file = InputReader::new(file);

        let mut lexx = make_test_lexx(input_file);

        loop {
            match lexx.next_token() {
                Ok(Some(token)) => {
                    total += 1;
                    lines = token.line;
                    match token.token_type {
                        TOKEN_TYPE_INTEGER => {
                            integers += 1;
                        }
                        TOKEN_TYPE_FLOAT => {
                            floats += 1;
                        }
                        TOKEN_TYPE_WHITESPACE => {
                            whitespace += 1;
                        }
                        TOKEN_TYPE_SYMBOL => {
                            //println!("'{}'", token.value);
                            symbols += 1;
                        }
                        TOKEN_TYPE_WORD => {
                            words += 1;
                            let count = unique_words.entry(token.value).or_insert(0);
                            *count += 1;
                        }
                        _ => {
                            assert!(false, "Don't know what this is!")
                        }
                    }
                }
                Err(e) => match e {
                    LexxError::TokenNotFound(_) => {
                        assert!(false, "Should not have failed finding a token file");
                    }
                    LexxError::Error(_) => {
                        assert!(false, "Should not have failed parsing file");
                    }
                },
                Ok(None) => break,
            }
        }

        let duration = start.elapsed();
        assert_eq!(743524, total);
        assert_eq!(124, integers);
        assert_eq!(1, floats);
        assert_eq!(332189, whitespace);
        assert_eq!(72301, symbols);
        assert_eq!(338909, words);
        assert_eq!(13267, unique_words.len());
        assert_eq!(43680, lines);
        println!("Time elapsed is: {:?}", duration);
    }

    #[test]
    fn lexx_parse_utf_file() {
        let file = File::open("./tests/utf-8-sampler.txt").unwrap();

        let input_file = InputReader::new(file);

        let lexx = make_test_lexx(input_file);

        let mut final_token: Token = Token {
            value: "".to_string(),
            token_type: 0,
            len: 0,
            line: 0,
            column: 0,
            precedence: 0
        };

        for token in lexx {
            if token.token_type != TOKEN_TYPE_WHITESPACE {
                final_token = token;
            }
        }

        assert_eq!(TOKEN_TYPE_SYMBOL, final_token.token_type);
        assert_eq!(72, final_token.column);
        assert_eq!(204, final_token.line);
        assert_eq!(String::from("▁▂▃▄▅▆▇█"), final_token.value);
    }

    fn make_test_lexx(input_file: InputReader<File>) -> Box<Lexx<512>> {
        Box::new(Lexx::<512>::new(
            Box::new(input_file),
            vec![
                Box::new(IntegerMatcher {
                    index: 0,
                    precedence: 0,
                    running: true,
                }),
                Box::new(FloatMatcher {
                    index: 0,
                    precedence: 0,
                    dot: false,
                    float: false,
                    running: true,
                }),
                Box::new(WhitespaceMatcher {
                    index: 0,
                    column: 0,
                    line: 0,
                    precedence: 0,
                    running: true,
                }),
                Box::new(WordMatcher {
                    index: 0,
                    precedence: 0,
                    running: true,
                }),
                Box::new(SymbolMatcher {
                    index: 0,
                    precedence: 0,
                    running: true,
                }),
            ],
        ))
    }
}
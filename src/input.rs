use crate::LexxError;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::io::Read;
use std::str::{from_utf8, from_utf8_unchecked};

/// maximum size for input strings for the [InputReader](crate::input::InputReader)
/// and the size of the buffer windows used by [InputString](crate::input::InputString).
pub const BUFFER_SIZE: usize = 1024;

/// An input error
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LexxInputError {
    /// An Error
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

/// LexxInput simply feeds out [Option<char>]s until there are no more chars, and it will return [None]
/// It doesn't need to be any more complex than that, but it also uses a [Result] to allow more
/// complex errors to be reported.
///
/// # Example
///
/// ```rust
/// use lexx::input::InputString;
/// use crate::lexx::input::LexxInput;
///
/// let mut lexx_input = InputString::new(String::from("The\n"));
///
/// assert!(matches!(lexx_input.next(), Ok(Some(c)) if c == 'T'));
/// assert!(matches!(lexx_input.next(), Ok(Some(c)) if c == 'h'));
/// assert!(matches!(lexx_input.next(), Ok(Some(c)) if c == 'e'));
/// assert!(matches!(lexx_input.next(), Ok(Some(c)) if c == '\n'));
/// assert!(matches!(lexx_input.next(), Ok(None)));
/// ```
pub trait LexxInput: Debug {
    /// returns the next LexxInputResult
    fn next(&mut self) -> Result<Option<char>, LexxInputError>;
}


/// Implements [LexxInput](LexxInput) for a passed in [String].
#[derive(Debug)]
pub struct InputString {
    /// Index into the string
    index: usize,
    /// Size of the string in chars
    size: usize,
    /// char buffer the string is translated into on creation
    chars: Box<[char; BUFFER_SIZE]>,
}

impl InputString {
    /// Creates a new InputString, if the passed in string is larger than
    /// [BUFFER_SIZE](BUFFER_SIZE) the string will be truncated to fit.
    ///
    /// # Arguments
    ///
    /// * `text` - The [String] that will be output
    ///
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
    /// Returns each character in the string one at a time until EOF when it will return [Ok(None)]
    fn next(&mut self) -> Result<Option<char>, LexxInputError> {
        if self.index < self.size {
            let c = self.chars[self.index];
            self.index += 1;
            return Ok(Some(c));
        }
        return Ok(None);
    }
}


/// Implements [LexxInput](LexxInput) for the [Read](std::io::Read) trait.
/// It uses a paged buffer to load the file. [BUFFER_SIZE] sets the size of the buffer used.
/// The stream needs to be UTF8.
#[derive(Debug)]
pub struct InputReader<R>
    where
        R: Read + Debug,
{
    /// current index into the buffer
    index: usize,
    /// current amount of data read into the buffer
    size: usize,
    /// rollover start, a rollover happens when a multi-byte utf code gets cut off by the end of the
    /// buffer. We need to save the partial code and append it to the start of the buffer on the
    /// next load.
    rollover_start: usize,
    /// rollover end
    rollover_end: usize,
    /// stream handle
    reader: R,
    /// byte buffer the file is read into
    buffer: Box<[u8; BUFFER_SIZE]>,
    /// char buffer the byte buffer is translated into
    text: Box<[char; BUFFER_SIZE]>,
}

impl<R> InputReader<R>
    where
        R: Read + Debug,
{
    /// creates a new InputReader
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
    /// gets the next char from the buffer, re-loads the buffer as needed
    fn next(&mut self) -> Result<Option<char>, LexxInputError> {
        if self.index < self.size {
            let c = self.text[self.index];
            self.index += 1;
            return Ok(Some(c));
        }
        let n: usize;
        if self.rollover_start == 0 {
            n = self.reader.read(self.buffer.as_mut()).unwrap();
        } else {
            self.buffer.as_mut().copy_within(self.rollover_start..self.rollover_end, 0);
            n = self.reader.read(self.buffer[(self.rollover_end-self.rollover_start)..].as_mut()).unwrap()
                + (self.rollover_end-self.rollover_start);
            self.rollover_start = 0;
        }

        if n == 0 {
            return Ok(None);
        }
        let se: &str;
        {
            match from_utf8(&self.buffer[..n]) {
                Ok(s) => {
                    se = s;
                }
                Err(e) => {
                    let end = e.valid_up_to();
                    // This is safe due to the above check
                    se = unsafe { from_utf8_unchecked(&self.buffer[..n][..end]) };
                    self.rollover_start = end;
                    self.rollover_end = n;
                }
            }
        }
        self.size = 0;
        self.index = 1;
        let cs = se.chars();
        for c in cs {
            self.text[self.size] = c;
            self.size += 1;
        }
        return Ok(Some(self.text[0]));
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::File;
    use std::time::Instant;

    use crate::{Lexx, Lexxer, LexxError};
    use crate::input::InputReader;
    use crate::matcher_integer::IntegerMatcher;
    use crate::matcher_float::FloatMatcher;
    use crate::matcher_symbol::SymbolMatcher;
    use crate::matcher_whitespace::WhitespaceMatcher;
    use crate::matcher_word::WordMatcher;
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

        let file = File::open("Varney-the-Vampire.txt").unwrap();

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
        let file = File::open("utf-8-sampler.txt").unwrap();

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
//! Lexxor is a fast, extensible, greedy, single-pass text tokenizer.
//!
//! Sample output for the string "This is  \n1.0 thing."
//! ```
//! use lexxor::token::Token;
//! Token{ token_type: 4, value: "This".to_string(), line: 1, column: 1, len: 4, precedence: 0};
//! Token{ token_type: 3, value: " ".to_string(), line: 1, column: 5, len: 1, precedence: 0};
//! Token{ token_type: 4, value: "is".to_string(), line: 1, column: 6, len: 2, precedence: 0};
//! Token{ token_type: 3, value: "  \n".to_string(), line: 1, column: 8, len: 3, precedence: 0};
//! Token{ token_type: 2, value: "1.0".to_string(), line: 2, column: 1, len: 3, precedence: 0};
//! Token{ token_type: 3, value: " ".to_string(), line: 2, column: 4, len: 1, precedence: 0};
//! Token{ token_type: 4, value: "thing".to_string(), line: 2, column: 5, len: 5, precedence: 0};
//! Token{ token_type: 5, value: ".".to_string(), line: 2, column: 10, len: 1, precedence: 0};
//! ```
//! Lexxor uses a [`LexxorInput`] to provide chars that are fed to
//! [`Matcher`] instances until the longest match is found, if any. The
//! match will be returned as a [`Token`] instance. The
//! [`Token`] includes a type and the string matched as well as the
//! line and column where the match was made. A custom [`LexxorInput`]
//! can be passed to Lexxor but the library comes with implementations for
//! [`InputString`](input::InputString) and
//! [`InputReader`](input::InputReader) types.
//!
//! Lexxor implements [`Iterator`] so it can be used with `for` loops.
//!
//! Custom [`Matcher`]s can also be made though Lexxor comes with:
//! - [`WordMatcher`](matcher::word::WordMatcher) matches alphabetic characters such as `ABCdef` and `word`
//! - [`IntegerMatcher`](matcher::integer::IntegerMatcher) matches integers such as `3` or `14537`
//! - [`FloatMatcher`](matcher::float::FloatMatcher) matches floats such as `434.312` or `0.001`
//! - [`ExactMatcher`](matcher::exact::ExactMatcher) given a vector of strings matches exactly those strings.
//!   You can initialize it with a type to return so you can use multiple ones for different things. For example, one
//!   [`ExactMatcher`](matcher::exact::ExactMatcher) can
//!   be used to find operators such as `==` and `+` while another could be used to find block identifiers
//!   such as `(` and `)`.
//! - [`SymbolMatcher`](matcher::symbol::SymbolMatcher) matches all non-alphanumerics `*&)_#@` or `.`.
//!   This is a good catch-all matcher.
//! - [`KeywordMatcher`](matcher::keyword::KeywordMatcher) matches specific passed-in words such as
//!   `new` or `specific`. It differs from the [`ExactMatcher`](matcher::exact::ExactMatcher) in that it
//!   will not match substrings, such as the `new` in `renewable` or `newfangled`.
//! - [`WhitespaceMatcher`](matcher::whitespace::WhitespaceMatcher) matches whitespace such as `  ` or `\t\r\n`
//!
//! [`Matcher`]s can be given a precedence that can make a matcher return its
//! results even if another matcher has a longer match. For example, both the [`WordMatcher`](matcher::word::WordMatcher)
//! and [`KeywordMatcher`](matcher::keyword::KeywordMatcher) are used at the same time.
//!
//! Note that matchers cannot find matches that start inside the valid matches of other matchers.
//! For matching `renewable`, the [`WordMatcher`](matcher::word::WordMatcher)
//! will make the match even if the [`ExactMatcher`](matcher::exact::ExactMatcher)
//! is looking for `new` with a higher precedence because the [`WordMatcher`](matcher::word::WordMatcher)
//! will consume all of `renewable` without giving other matchers the chance to look inside of it.
//!
//! Also, while the [`ExactMatcher`](matcher::exact::ExactMatcher)
//! could find the `new` inside `newfangled`, the [`WordMatcher`](matcher::word::WordMatcher)
//! would match `newfangled` instead since it is longer, unless the [`ExactMatcher`](matcher::exact::ExactMatcher) is
//! given a higher precedence in which case it would get to return `new` and the next match would
//! start at `fangled`.
//!
//! To successfully parse an entire stream, Lexxor must have a matcher with which to tokenize every
//! encountered collection of characters. If a match fails, Lexxor will return `Err(
//! [TokenNotFound](LexxError::TokenNotFound))` with the text that could not be matched.
//!
//! # Panics
//!
//! For speed, Lexxor does not dynamically allocate buffer space. In `Lexxor<CAP>`, `CAP` is the maximum
//! possible token size; if that size is exceeded, a panic will be thrown.
//!
//! # Example
//!
//! ```rust
//! use lexxor::{matcher, Lexxor, Lexxer};
//! use lexxor::input::InputString;
//! use lexxor::matcher::exact::ExactMatcher;
//! use lexxor::matcher::symbol::SymbolMatcher;
//! use lexxor::matcher::whitespace::WhitespaceMatcher;
//! use lexxor::matcher::word::WordMatcher;
//! use lexxor::token::{TOKEN_TYPE_EXACT, TOKEN_TYPE_WORD, TOKEN_TYPE_WHITESPACE, TOKEN_TYPE_SYMBOL};
//!
//! let lexxor_input = InputString::new(String::from("The quick\n\nbrown fox."));
//!
//! let mut lexxor: Box<dyn Lexxer> = Box::new(Lexxor::<512>::new(
//!   Box::new(lexxor_input),
//!   vec![
//!     Box::new(WordMatcher{ index: 0, precedence: 0, running: true }),
//!     Box::new(WhitespaceMatcher { index: 0, column: 0,line: 0,precedence: 0, running: true }),
//!     Box::new(SymbolMatcher { index:0, precedence: 0, running: true }),
//!     // with a precedence of 1 this will match "quick" instead of the word matcher
//!     // We can change the TOKEN_TYPE value returned if we want to have more than one
//!     // ExactMatcher that return different token types.
//!     Box::new(ExactMatcher::build_exact_matcher(vec!["quick"], TOKEN_TYPE_EXACT, 1)),
//!   ]
//! ));
//!
//! assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "The" && t.token_type == TOKEN_TYPE_WORD && t.line == 1 && t.column == 1));
//! assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
//! // Because the ExactMatcher is looking for `quick` with a precedence higher than
//! // that of the WordMatcher it will return a match for `quick`.
//! assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "quick" && t.token_type == TOKEN_TYPE_EXACT && t.line == 1 && t.column == 5));
//! assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
//! assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "brown" && t.token_type == TOKEN_TYPE_WORD && t.line == 3 && t.column == 1));
//! assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
//! assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "fox" && t.token_type == TOKEN_TYPE_WORD && t.line == 3 && t.column == 7));
//! assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "." && t.token_type == TOKEN_TYPE_SYMBOL && t.line == 3 && t.column == 10));
//! assert!(matches!(lexxor.next_token(), Ok(None)));
//!
//! lexxor.set_input(Box::new(InputString::new(String::from("Hello world!"))));
//! for token in lexxor {
//!     println!("{}", token.value);
//! }
//! ```

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

/// The [LexxorInput] for lexxor
pub mod input;
/// The [Matcher] trait for lexxor
pub mod matcher;
/// [RollingCharBuffer](RollingCharBuffer) is a fast, fixed size
/// [char] buffer that can be used as a LIFO or FIFO stack.
pub mod rolling_char_buffer;
/// The results of a match
pub mod token;

use arrayvec::ArrayVec;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use crate::input::LexxorInput;
// use crate::matcher::Matcher;
// use crate::matcher::MatcherResult::{Failed, Matched, Running};
use crate::matcher::Matcher;
use crate::matcher::MatcherResult::{Failed, Matched, Running};
use crate::rolling_char_buffer::{RollingCharBuffer, RollingCharBufferError};
use token::Token;

/// Errors Lexxorcan return
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LexxError {
    /// no matcher matched the current character(s)
    TokenNotFound(String),
    /// some other error
    Error(String),
}

impl fmt::Display for LexxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LexxError::TokenNotFound(ref s) => {
                write!(f, "a parser could not be found for: {:?}", s)
            }
            LexxError::Error(ref s) => {
                write!(f, "an error occurred: {:?}", s)
            }
        }
    }
}

impl Error for LexxError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        match *self {
            LexxError::TokenNotFound(..) => "no token could be found",
            LexxError::Error(..) => "an error occurred",
        }
    }
}

/// The lexer itself. Implements [Lexxer] so you can use `Box<dyn Lexxer>` and don't
/// have to define the `CAP` in var declarations.
///
/// # Overview
///
/// `Lexx` is a fast, extensible, greedy, single-pass text tokenizer. It works by passing input characters
/// from a [`LexxorInput`] to a set of [`Matcher`] instances.
/// Each matcher attempts to find the longest valid token at the current position. The matchers can be
/// prioritized using precedence, allowing for flexible tokenization strategies (e.g., keywords vs. words).
///
/// The lexer maintains line and column information, supports lookahead, and allows rewinding tokens back into the stream.
///
/// # Type Parameter
///
/// * `CAP` - The maximum token size supported by this lexer instance. For speed, no dynamic allocation is performed.
///   If a token exceeds this size, a panic will occur.
///
/// # Fields
///
/// * `matchers` - The array of matchers used to generate tokens.
/// * `input` - The input source to be tokenized.
/// * `cache` - Buffer for excess input characters and for supporting rewind.
/// * `value` - Buffer for the current token being matched.
/// * `lexxor_result` - Stores the result of lookahead operations.
/// * `found_token` - Most recent acceptable token found during matching.
/// * `line`, `column` - Current line and column in the input.
/// * `ctx` - Shared context for use by custom matchers.
///
/// # Example Usage
///
/// See the crate-level documentation for a complete example.
#[derive(Debug)]
pub struct Lexxor<const CAP: usize> {
    /// The array of matcher used to generate tokens
    matchers: Vec<Box<dyn Matcher>>,
    /// The input the matchers will be run against
    input: Box<dyn LexxorInput>,
    /// When more chars are pulled from the input than the matchers use the
    /// excess is stored in this buffer. In this way the Input doesn't need to be re-indexed.
    /// This is also used by the Rewind feature.
    cache: Box<RollingCharBuffer<CAP>>,
    /// While the match is being made the chars are stored in this buffer.
    value: Box<ArrayVec<char, CAP>>,
    /// If [Lexxor::look_ahead] is called the results are also stored here.
    pub lexxor_result: Option<Result<Option<Token>, LexxError>>,
    /// While matches are being made the most recent acceptable token is stored here.
    pub found_token: Option<Token>,
    /// The current line in the input.
    pub line: usize,
    /// The current column in the input.
    pub column: usize,
    /// A general use hashmap that can be used by custom matchers for context sharing.
    pub ctx: Box<HashMap<String, i32>>,
}

impl<const CAP: usize> Lexxor<CAP> {
    /// Creates a new Lexx
    ///
    /// # Arguments
    ///
    /// * `input` - An instance of [LexxorInput] that provides
    ///   the char stream that will be lexed.
    /// * `matchers` - a [vec] of [Matcher]s that will be used to
    ///   generate Tokens.
    ///
    /// # Examples
    ///
    /// See [lexxor](crate)
    ///
    pub fn new(input: Box<dyn LexxorInput>, matchers: Vec<Box<dyn Matcher>>) -> Self {
        let cache = Box::new(RollingCharBuffer::<CAP>::new());
        Lexxor {
            matchers,
            input,
            cache,
            value: Box::new(ArrayVec::<char, CAP>::new()),
            lexxor_result: None,
            found_token: None,
            line: 1,
            column: 1,
            ctx: Box::new(HashMap::new()),
        }
    }

    fn get_token(&mut self) -> Result<Option<Token>, LexxError> {
        let mut precedence = 0;
        self.value.clear();
        for m in &mut self.matchers {
            m.reset(&mut self.ctx);
        }
        self.found_token = None;
        loop {
            let c = if self.cache.is_empty() {
                self.input.next()?
            } else {
                Some(self.cache.read().unwrap())
            };
            let mut found_token: Option<Token> = None;
            let mut running = false;
            if let Some(ch) = c {
                self.value.push(ch);
            }
            for m in &mut self.matchers {
                if m.is_running() {
                    match m.find_match(c, &self.value[..], &mut self.ctx) {
                        Running() => running = true,
                        Matched(token) => {
                            if let Some(ref _ft) = found_token {
                                if precedence <= token.precedence {
                                    precedence = token.precedence;
                                    found_token = Some(token);
                                }
                            } else {
                                precedence = token.precedence;
                                found_token = Some(token);
                            }
                        }
                        Failed() => {}
                    }
                }
            }
            if let Some(token) = found_token {
                match &self.found_token {
                    Some(ft) if ft.precedence <= token.precedence => self.found_token = Some(token),
                    None => self.found_token = Some(token),
                    _ => {}
                }
            }
            if !running {
                if let Some(mut token) = self.found_token.take() {
                    if self.value.len() > token.len {
                        if let Err(e) = self.cache.prepend(&self.value[token.len..]) {
                            panic!("Ran out of buffer space: {}", e)
                        }
                    }
                    let l = self.line;
                    let c = self.column;
                    if token.line > 0 {
                        self.line += token.line;
                        self.column = token.column;
                    } else {
                        self.column += token.column;
                    }
                    token.line = l;
                    token.column = c;
                    return Ok(Some(token));
                } else {
                    return if c.is_none() {
                        Ok(None)
                    } else {
                        Err(LexxError::TokenNotFound(format!(
                            "Could not resolve token at {}, {}: '{:?}'.",
                            self.line, self.column, c
                        )))
                    };
                }
            }
            if c.is_none() {
                return Ok(None);
            }
        }
    }
}
impl<const CAP: usize> Lexxer for Lexxor<CAP> {
    ///
    /// Returns the next `[Result<Option<Token>, LexxError>](Result)`.
    ///
    /// The `[Option]` will be `None` if there is no remaining input (EOF)
    ///
    /// # Examples
    ///
    /// See `[lexxor](crate)`
    ///
    fn next_token(&mut self) -> Result<Option<Token>, LexxError> {
        if self.lexxor_result.is_some() {
            let lr = self.lexxor_result.clone().unwrap();
            self.lexxor_result = None;
            return lr;
        }
        self.get_token()
    }

    ///
    /// Returns the next `[Result<Option<Token>, LexxError>](Result)`. However the next call to `[Lexx::next_token]`
    /// will return a clone of the same `[Result<Option<Token>, LexxError>](Result)`. Likewise `[Lexx::look_ahead]`
    /// can be called repeatedly to get a copy of the same `[Result<Option<Token>, LexxError>](Result)`.
    ///
    /// * `Matched` - The next `[Token](Token)` found in the input.
    /// * `EndOfInput` - No more chars in the given `[LexxorInput](LexxorInput)`.
    /// * `Failed` - Something went wrong or no match could be made.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexxor::token::{TOKEN_TYPE_EXACT, TOKEN_TYPE_WORD, TOKEN_TYPE_WHITESPACE, TOKEN_TYPE_SYMBOL};
    /// use lexxor::input::InputString;
    /// use lexxor::{Lexxor, Lexxer};
    /// use lexxor::matcher::exact::ExactMatcher;
    /// use lexxor::matcher::symbol::SymbolMatcher;
    /// use lexxor::matcher::whitespace::WhitespaceMatcher;
    /// use lexxor::matcher::word::WordMatcher;
    ///
    /// let lexxor_input = InputString::new(String::from("The quick\n\nbrown fox."));
    ///
    /// let mut lexxor: Box<dyn Lexxer> = Box::new(Lexxor::<512>::new(
    /// Box::new(lexxor_input),
    /// vec![
    ///     Box::new(WordMatcher{ index: 0, precedence: 0, running: true }),
    ///     Box::new(WhitespaceMatcher { index: 0, column: 0,line: 0,precedence: 0, running: true}),
    ///     Box::new(SymbolMatcher { index:0, precedence: 0, running: true }),
    ///     // with a precedence of 1 this will match "quick" instead of the word matcher
    ///     // We can change the TOKEN_TYPE value returned if we want to have more than one
    ///     // ExactMatcher that return different token types.
    ///     Box::new(ExactMatcher::build_exact_matcher(vec!["quick"], TOKEN_TYPE_EXACT, 1)),
    /// ]));
    ///
    /// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "The" && t.token_type == TOKEN_TYPE_WORD && t.line == 1 && t.column == 1));
    /// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
    /// // Because the ExactMatcher is looking for `quick` with a precedence higher than
    /// // that of the WordMatcher it will return a match for `quick`.
    /// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "quick" && t.token_type == TOKEN_TYPE_EXACT && t.line == 1 && t.column == 5));
    /// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
    /// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "brown" && t.token_type == TOKEN_TYPE_WORD && t.line == 3 && t.column == 1));
    /// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
    /// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "fox" && t.token_type == TOKEN_TYPE_WORD && t.line == 3 && t.column == 7));
    /// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "." && t.token_type == TOKEN_TYPE_SYMBOL && t.line == 3 && t.column == 10));
    /// assert!(matches!(lexxor.next_token(), Ok(None)));
    /// ```
    ///
    fn look_ahead(&mut self) -> Result<Option<Token>, LexxError> {
        if self.lexxor_result.is_some() {
            self.lexxor_result.clone().unwrap()
        } else {
            self.lexxor_result = Some(self.get_token());
            self.lexxor_result.clone().unwrap()
        }
    }

    ///
    /// Stuffs the token back into the stream to be re-tokenized
    /// (not really, but that's the effect it has). The line and column
    /// values will be reset to this tokens values.
    ///
    /// This does not actually have to be the same token you just pulled out, nothing
    /// checks to make sure, you can shove anything in here you like as long
    /// as the cache buffer doesn't overflow. Be careful with line and column values
    /// if you want to mess with the order though.
    ///
    /// * `token` - The Token who's `value` will be pushed into the cache to be re-tokenized
    ///
    fn rewind(&mut self, token: Token) -> Result<usize, RollingCharBufferError> {
        self.line = token.line;
        self.column = token.column;
        self.cache
            .prepend(&token.value.chars().collect::<Vec<char>>())
    }

    ///
    /// Change the input to something else, resets all the tracking variables.
    /// If you're done tokenizing something you can tokenize something else with
    /// all the same matchers without having to make a new Lexx.
    ///
    /// * `input` - An instance of `[LexxorInput](LexxorInput)` that provides the char stream that will be lexed.
    ///
    fn set_input(&mut self, input: Box<dyn LexxorInput>) {
        self.input = input;
        self.line = 1;
        self.column = 1;
        self.cache.clear();
        self.lexxor_result = None;
    }
}

/// A trait for [Lexxor], so you can use `Box<dyn Lexxer>` and don't have to define the
/// `CAP` in var declarations.
pub trait Lexxer {
    ///
    /// Returns the next `[Result<Option<Token>, LexxError>](Result)`.
    ///
    /// The [Option] will be `None` if there is no remaining input (EOF)
    ///
    /// # Examples
    ///
    /// See [lexxor](crate)
    ///
    fn next_token(&mut self) -> Result<Option<Token>, LexxError>;

    ///
    /// Returns the next `[Result<Option<Token>, LexxError>](Result)`. However, the next call to [Lexxor::next_token]
    /// will return a clone of the same `[Result<Option<Token>, LexxError>](Result)`. Likewise, [Lexxor::look_ahead]
    /// can be called repeatedly to get a copy of the same `[Result<Option<Token>, LexxError>](Result)`.
    ///
    /// * `Matched` - The next `[Token](Token)` found in the input.
    /// * `EndOfInput` - No more chars in the given `[LexxorInput](LexxorInput)`.
    /// * `Failed` - Something went wrong, or no match could be made.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexxor::token::{TOKEN_TYPE_WORD};
    /// use lexxor::token::TOKEN_TYPE_WHITESPACE;
    /// use lexxor::input::InputString;
    /// use lexxor::{Lexxor, Lexxer};
    /// use lexxor::matcher::whitespace::WhitespaceMatcher;
    /// use lexxor::matcher::word::WordMatcher;
    ///
    /// let lexxor_input = InputString::new(String::from("The quick brown fox"));
    ///
    /// let mut lexxor: Box<dyn Lexxer> = Box::new(Lexxor::<512>::new(
    /// Box::new(lexxor_input),
    /// vec![
    ///     Box::new(WordMatcher{ index: 0, precedence: 0, running: true }),
    ///     Box::new(WhitespaceMatcher { index: 0, column: 0,line: 0,precedence: 0, running: true}),
    /// ]
    /// ));
    ///
    /// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "The" && t.token_type == TOKEN_TYPE_WORD && t.line == 1 && t.column == 1));
    /// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
    /// assert!(matches!(lexxor.look_ahead(), Ok(Some(t)) if t.value == "quick" && t.token_type == TOKEN_TYPE_WORD && t.line == 1 && t.column == 5));
    /// assert!(matches!(lexxor.look_ahead(), Ok(Some(t)) if t.value == "quick" && t.token_type == TOKEN_TYPE_WORD && t.line == 1 && t.column == 5));
    /// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "quick" && t.token_type == TOKEN_TYPE_WORD && t.line == 1 && t.column == 5));
    /// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
    /// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "brown" && t.token_type == TOKEN_TYPE_WORD && t.line == 1 && t.column == 11));
    /// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
    /// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "fox" && t.token_type == TOKEN_TYPE_WORD && t.line == 1 && t.column == 17));
    /// assert!(matches!(lexxor.next_token(), Ok(None)));
    /// ```
    ///
    fn look_ahead(&mut self) -> Result<Option<Token>, LexxError>;

    ///
    /// Stuffs the token back into the stream to be re-tokenized
    /// (not really, but that's the effect this has). The line and column
    /// values will be reset to this tokens values.
    ///
    /// This does not actually have to be the same token you just pulled out, nothing
    /// checks to make sure, you can shove anything in here you like as long
    /// as the cache buffer doesn't overflow. Be careful with line and column values
    /// if you want to mess with the order though.
    ///
    /// * `token` - The Token who's `value` will be pushed into the cache to be re-tokenized
    ///
    fn rewind(&mut self, token: Token) -> Result<usize, RollingCharBufferError>;

    ///
    /// Change the input to something else, resets all the tracking variables.
    /// If you're done tokenizing something you can tokenize something else with
    /// all the same matchers without having to make a new Lexx.
    ///
    /// * `input` - An instance of `[LexxorInput](LexxorInput)` that provides the char stream that will be lexed.
    ///
    fn set_input(&mut self, input: Box<dyn LexxorInput>);
}

impl Iterator for dyn Lexxer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
            .unwrap_or_else(|e| panic!("{}", e.to_string()))
    }
}

impl<const CAP: usize> Iterator for Lexxor<CAP> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
            .unwrap_or_else(|e| panic!("{}", e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::input::InputString;
    use crate::matcher::exact::ExactMatcher;
    use crate::matcher::whitespace::WhitespaceMatcher;
    use crate::matcher::word::WordMatcher;
    use crate::token::{TOKEN_TYPE_EXACT, TOKEN_TYPE_WHITESPACE};
    use crate::{Lexxer, Lexxor, Token};

    #[test]
    fn lexxor_test_precedence() {
        let mut lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from("fox"))),
            vec![
                Box::new(ExactMatcher::build_exact_matcher(
                    vec!["fox"],
                    TOKEN_TYPE_EXACT,
                    1,
                )),
                Box::new(WordMatcher {
                    index: 0,
                    precedence: 0,
                    running: true,
                }),
            ],
        );

        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "fox" && t.token_type == TOKEN_TYPE_EXACT)
        );

        lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from("fox"))),
            vec![
                Box::new(WordMatcher {
                    index: 0,
                    precedence: 0,
                    running: true,
                }),
                Box::new(ExactMatcher::build_exact_matcher(
                    vec!["fox"],
                    TOKEN_TYPE_EXACT,
                    1,
                )),
            ],
        );

        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "fox" && t.token_type == TOKEN_TYPE_EXACT)
        );
    }

    #[test]
    fn lexxor_test_look_ahead() {
        let mut lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from("The lazy dog"))),
            vec![
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
            ],
        );

        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "The"));
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE)
        );
        assert!(
            matches!(lexxor.look_ahead(), Ok(Some(t)) if t.value == "lazy" && t.line == 1 && t.column == 5 && t.len == 4)
        );
        assert!(
            matches!(lexxor.look_ahead(), Ok(Some(t)) if t.value == "lazy" && t.line == 1 && t.column == 5 && t.len == 4)
        );
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "lazy" && t.line == 1 && t.column == 5 && t.len == 4)
        );
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE)
        );
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "dog" && t.line == 1 && t.column == 10 && t.len == 3)
        );
    }

    #[test]
    fn lexxor_test_rewind() {
        let mut lexxor = Lexxor::<20>::new(
            Box::new(InputString::new(String::from("The lazy dog"))),
            vec![
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
            ],
        );

        let mut the: Option<Token> = None;
        if let Ok(t) = lexxor.next_token() {
            assert_eq!(t.as_ref().unwrap().value, "The");
            the = t;
        }
        let mut whitespace: Option<Token> = None;
        if let Ok(t) = lexxor.next_token() {
            assert_eq!(t.as_ref().unwrap().token_type, TOKEN_TYPE_WHITESPACE);
            whitespace = t;
        }
        let mut lazy: Option<Token> = None;
        if let Ok(t) = lexxor.next_token() {
            assert_eq!(t.as_ref().unwrap().value, "lazy");
            lazy = t;
        }
        if let Some(t) = lazy {
            assert_eq!(lexxor.rewind(t), Ok(15))
        }
        if let Some(t) = whitespace {
            assert_eq!(lexxor.rewind(t), Ok(14))
        }
        if let Some(t) = the {
            assert_eq!(lexxor.rewind(t), Ok(11))
        }
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "The"));
        let w = lexxor.next_token();
        assert!(matches!(w, Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "lazy" && t.line == 1 && t.column == 5 && t.len == 4)
        );
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE)
        );
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "dog" && t.line == 1 && t.column == 10 && t.len == 3)
        );
    }
}

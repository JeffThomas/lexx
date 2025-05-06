/// The `whitespace` module provides the `WhitespaceMatcher`, which matches sequences of whitespace characters in the input stream.
/// Whitespace characters are those for which Rust’s `char::is_whitespace()` returns true (such as spaces, tabs, and newlines).
/// The matcher recognizes contiguous runs of whitespace and produces tokens of type `TOKEN_TYPE_WHITESPACE`.
///
/// This module is useful for lexers that need to identify and handle whitespace regions, which are often ignored or treated specially in tokenization.
use crate::matcher::{Matcher, MatcherResult};
pub use crate::token::{TOKEN_TYPE_WHITESPACE, Token};
use std::collections::HashMap;

/// The WhitespaceMatcher matches any series of characters that are `is_whitespace()`.
///
/// # Example
///
/// ```rust
/// use lexx::{Lexx, Lexxer};
/// use lexx::token::{TOKEN_TYPE_WHITESPACE, TOKEN_TYPE_WORD};
/// use lexx::input::InputString;
/// use lexx::matcher::whitespace::WhitespaceMatcher;
/// use lexx::matcher::word::WordMatcher;
///
/// let lexx_input = InputString::new(String::from(" a \t\nb \r\n\t c"));
///
/// let mut lexx: Box<dyn Lexxer> = Box::new(Lexx::<512>::new(
///     Box::new(lexx_input),
///     vec![
///         Box::new(WhitespaceMatcher { index: 0, column: 0,line: 0,precedence: 0, running: true}),
///         Box::new(WordMatcher { index: 0, precedence: 0, running: true }),
///     ]
/// ));
///
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE && t.line == 1 && t.column == 1));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "a" && t.token_type == TOKEN_TYPE_WORD && t.line == 1 && t.column == 2));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE && t.line == 1 && t.column == 3));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "b" && t.token_type == TOKEN_TYPE_WORD && t.line == 2 && t.column == 1));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE && t.line == 2 && t.column == 2));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "c" && t.token_type == TOKEN_TYPE_WORD && t.line == 3 && t.column == 3));
/// assert!(matches!(lexx.next_token(), Ok(None)));
/// ```
#[derive(Clone, Debug, Copy)]
pub struct WhitespaceMatcher {
    /// Current size of the ongoing match.
    pub index: usize,
    /// column count
    pub column: usize,
    /// line count
    pub line: usize,
    /// This matchers precedence.
    pub precedence: u8,
    /// If the matcher is currently running.
    pub running: bool,
}

impl Matcher for WhitespaceMatcher {
    fn reset(&mut self, _ctx: &mut Box<HashMap<String, i32>>) {
        self.index = 0;
        self.line = 0;
        self.column = 0;
        self.running = true;
    }

    fn find_match(
        &mut self,
        oc: Option<char>,
        value: &[char],
        _ctx: &mut Box<HashMap<String, i32>>,
    ) -> MatcherResult {
        match oc {
            Some(c) if c.is_whitespace() => {
                self.index += 1;
                self.column += 1;
                if c == '\r' {
                    self.column = 1;
                } else if c == '\n' {
                    self.column = 1;
                    self.line += 1;
                }
                MatcherResult::Running()
            }
            _ => {
                self.running = false;
                self.generate_whitspace_token(value)
            }
        }
    }
    fn is_running(&self) -> bool {
        self.running
    }
    fn precedence(&self) -> u8 {
        self.precedence
    }
}

impl WhitespaceMatcher {
    #[inline(always)]
    fn generate_whitspace_token(&mut self, value: &[char]) -> MatcherResult {
        if self.index > 0 {
            MatcherResult::Matched(Token {
                value: value[0..self.index].iter().collect(),
                token_type: TOKEN_TYPE_WHITESPACE,
                len: self.index,
                line: self.line,
                column: self.column,
                precedence: self.precedence,
            })
        } else {
            MatcherResult::Failed()
        }
    }
}

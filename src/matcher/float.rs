/// The float matcher matches floating point numbers. To qualify as floating point, the numbers must
/// start and end with a numeric digit and have a period within them. For example `1.0`. Thus,
/// `.1` and `1.` do not qualify as floating point numbers.
use crate::matcher::{Matcher, MatcherResult};
use crate::token::{Token, TOKEN_TYPE_FLOAT};
use std::collections::HashMap;

///
/// # Example
///
/// ```rust
/// use lexx::{Lexx, Lexxer};
/// use lexx::token::{TOKEN_TYPE_WHITESPACE, TOKEN_TYPE_FLOAT, TOKEN_TYPE_INTEGER, TOKEN_TYPE_SYMBOL};
/// use lexx::input::InputString;
/// use lexx::matcher::float::FloatMatcher;
/// use lexx::matcher::integer::IntegerMatcher;
/// use lexx::matcher::symbol::SymbolMatcher;
/// use lexx::matcher::whitespace::WhitespaceMatcher;
///
/// let mut lexx: Box<dyn Lexxer> = Box::new(Lexx::<512>::new(
/// Box::new(InputString::new(String::from("1.0 5 0.012345 .9 00.00 100.0 4."))),
/// vec![
///     Box::new(FloatMatcher{ index: 0, precedence: 0, dot: false, float:false, running: true }),
///     Box::new(SymbolMatcher { index: 0, precedence: 0, running: true }),
///     Box::new(IntegerMatcher { index: 0, precedence: 0, running: true }),
///     Box::new(WhitespaceMatcher { index: 0, column: 0,line: 0,precedence: 0, running: true}),
/// ]
/// ));
///
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "1.0" && t.token_type == TOKEN_TYPE_FLOAT && t.line == 1 && t.column == 1));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "5" && t.token_type == TOKEN_TYPE_INTEGER && t.line == 1 && t.column == 5));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "0.012345" && t.token_type == TOKEN_TYPE_FLOAT && t.line == 1 && t.column == 7));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "." && t.token_type == TOKEN_TYPE_SYMBOL && t.line == 1 && t.column == 16));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "9" && t.token_type == TOKEN_TYPE_INTEGER && t.line == 1 && t.column == 17));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "00.00" && t.token_type == TOKEN_TYPE_FLOAT && t.line == 1 && t.column == 19));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "100.0" && t.token_type == TOKEN_TYPE_FLOAT && t.line == 1 && t.column == 25));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "4" && t.token_type == TOKEN_TYPE_INTEGER && t.line == 1 && t.column == 31));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "." && t.token_type == TOKEN_TYPE_SYMBOL && t.line == 1 && t.column == 32));
/// assert!(matches!(lexx.next_token(), Ok(None)));
/// ```
#[derive(Clone, Debug, Copy)]
pub struct FloatMatcher {
    /// Current size of the ongoing match.
    pub index: usize,
    /// This is the matcher precedence.
    pub precedence: u8,
    /// If the dot has been seen or not.
    pub dot: bool,
    /// If this match now qualifies as a float.
    pub float: bool,
    /// If the matcher is currently running.
    pub running: bool,
}

impl Matcher for FloatMatcher {
    fn reset(&mut self, _ctx: &mut Box<HashMap<String, i32>>) {
        self.index = 0;
        self.dot = false;
        self.float = false;
        self.running = true;
    }

    fn find_match(
        &mut self,
        oc: Option<char>,
        value: &[char],
        _ctx: &mut Box<HashMap<String, i32>>,
    ) -> MatcherResult {
        match oc {
            None => self.generate_float_token(value),
            Some(c) => {
                if c == '.' && !self.dot && self.index > 0 {
                    self.index += 1;
                    self.dot = true;
                    return MatcherResult::Running();
                }
                if c.is_numeric() {
                    self.index += 1;
                    if self.dot {
                        self.float = true;
                    }
                    MatcherResult::Running()
                } else {
                    self.generate_float_token(value)
                }
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

impl FloatMatcher {
    #[inline(always)]
    fn generate_float_token(&mut self, value: &[char]) -> MatcherResult {
        self.running = false;
        if self.index > 0 && self.float {
            MatcherResult::Matched(Token {
                value: value[0..self.index].iter().collect(),
                token_type: TOKEN_TYPE_FLOAT,
                len: self.index,
                line: 0,
                column: self.index,
                precedence: self.precedence,
            })
        } else {
            MatcherResult::Failed()
        }
    }
}

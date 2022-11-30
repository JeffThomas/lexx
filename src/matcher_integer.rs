use crate::matcher::{Matcher, MatcherResult};
use crate::token::{Token, TOKEN_TYPE_INTEGER};
use std::collections::HashMap;

/// The integer matcher matches integer numbers. To qualify as integer the numbers must
/// start and end with a numeric digit.
///
/// # Example
///
/// ```rust
/// use lexx::{Lexx, Lexxer};
/// use lexx::token::{TOKEN_TYPE_WHITESPACE, TOKEN_TYPE_FLOAT, TOKEN_TYPE_INTEGER, TOKEN_TYPE_SYMBOL};
/// use lexx::matcher_float::FloatMatcher;
/// use lexx::matcher_symbol::SymbolMatcher;
/// use lexx::matcher_integer::IntegerMatcher;
/// use lexx::matcher_whitespace::WhitespaceMatcher;
/// use lexx::input::InputString;
///
/// let mut lexx: Box<dyn Lexxer> = Box::new(Lexx::<512>::new(
/// Box::new(InputString::new(String::from("1.0 5 0.012345 .9 00.00 100.0 4."))),
/// vec![
///     Box::new(FloatMatcher{ index: 0, precedence: 0, dot: false, float:false, running: true }),
///     Box::new(SymbolMatcher { index: 0, precedence: 0, running: true }),
///     Box::new(IntegerMatcher { index: 0, precedence: 0, running: true }),
///     Box::new(WhitespaceMatcher { index: 0, column: 0,line: 0,precedence: 0, running: true }),
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
pub struct IntegerMatcher {
    /// Current size of the ongoing match.
    pub index: usize,
    /// This matchers precedence.
    pub precedence: u8,
    /// If the matcher is currently running.
    pub running: bool,
}

impl Matcher for IntegerMatcher {
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
        return match oc {
            None => self.generate_integer_token(value),
            Some(c) => {
                if c.is_numeric() {
                    self.index += 1;
                    MatcherResult::Running()
                } else {
                    self.generate_integer_token(value)
                }
            }
        };
    }
    fn is_running(&self) -> bool {
        self.running
    }
    fn precedence(&self) -> u8 {
        self.precedence
    }
}

impl IntegerMatcher {
    #[inline(always)]
    fn generate_integer_token(&mut self, value: &[char]) -> MatcherResult {
        self.running = false;
        if self.index > 0 {
            MatcherResult::Matched(Token {
                value: value[0..self.index].into_iter().collect(),
                token_type: TOKEN_TYPE_INTEGER,
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

#[cfg(test)]
mod tests {
    use crate::matcher_integer::IntegerMatcher;
    use crate::matcher_whitespace::WhitespaceMatcher;
    use crate::{Lexx, LexxError, Lexxer};
    use crate::input::InputString;

    #[test]
    fn matcher_integer_matches_integer() {
        let mut lexx: Box<dyn Lexxer> = Box::new(Lexx::<512>::new(
            Box::new(InputString::new(String::from("4"))),
            vec![Box::new(IntegerMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        ));

        match lexx.next_token() {
            Err(e) => match e {
                LexxError::TokenNotFound(_) => {
                    assert!(false, "Should not have failed parsing file");
                }
                LexxError::Error(_) => {
                    assert!(false, "Should not have failed parsing file");
                }
            },
            Ok(Some(t)) => {
                assert_eq!(t.value, "4")
            }
            Ok(None) => {
                assert!(false, "Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_integer_matches_big_integer() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("6346357587454"))),
            vec![Box::new(IntegerMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        match lexx.next_token() {
            Err(e) => match e {
                LexxError::TokenNotFound(_) => {
                    assert!(false, "Should not have failed parsing file");
                }
                LexxError::Error(_) => {
                    assert!(false, "Should not have failed parsing file");
                }
            },
            Ok(Some(t)) => {
                assert_eq!(t.value, "6346357587454")
            }
            Ok(None) => {
                assert!(false, "Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_integer_matches_integer_not_float() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("5.5"))),
            vec![Box::new(IntegerMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        match lexx.next_token() {
            Err(e) => match e {
                LexxError::TokenNotFound(_) => {
                    assert!(false, "Should not have failed parsing file");
                }
                LexxError::Error(_) => {
                    assert!(false, "Should not have failed parsing file");
                }
            },
            Ok(Some(t)) => {
                assert_eq!(t.value, "5")
            }
            Ok(None) => {
                assert!(false, "Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_integer_matches_multiple() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("2 3 4"))),
            vec![
                Box::new(IntegerMatcher {
                    index: 0,
                    precedence: 0,
                    running: true,
                }),
                Box::new(WhitespaceMatcher {
                    index: 0,
                    column: 0,
                    line: 0,
                    precedence: 0,
                    running: true,
                }),
            ],
        );

        match lexx.next_token() {
            Err(e) => match e {
                LexxError::TokenNotFound(_) => {
                    assert!(false, "Should not have failed parsing file");
                }
                LexxError::Error(_) => {
                    assert!(false, "Should not have failed parsing file");
                }
            },
            Ok(Some(t)) => {
                assert_eq!(t.value, "2")
            }
            Ok(None) => {
                assert!(false, "Should not hit None");
            }
        }

        assert!(
            matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == crate::token::TOKEN_TYPE_WHITESPACE)
        );

        match lexx.next_token() {
            Err(e) => match e {
                LexxError::TokenNotFound(_) => {
                    assert!(false, "Should not have failed parsing file");
                }
                LexxError::Error(_) => {
                    assert!(false, "Should not have failed parsing file");
                }
            },
            Ok(Some(t)) => {
                assert_eq!(t.value, "3")
            }
            Ok(None) => {
                assert!(false, "Should not hit None");
            }
        }

        assert!(
            matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == crate::token::TOKEN_TYPE_WHITESPACE)
        );

        match lexx.next_token() {
            Err(e) => match e {
                LexxError::TokenNotFound(_) => {
                    assert!(false, "Should not have failed parsing file");
                }
                LexxError::Error(_) => {
                    assert!(false, "Should not have failed parsing file");
                }
            },
            Ok(Some(t)) => {
                assert_eq!(t.value, "4")
            }
            Ok(None) => {
                assert!(false, "Should not hit None");
            }
        }
    }
}

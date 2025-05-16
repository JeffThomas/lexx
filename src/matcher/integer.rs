/// The integer matcher matches integer numbers. To qualify as integer the numbers must
/// start and end with a numeric digit.
use crate::matcher::{Matcher, MatcherResult};
use crate::token::{TOKEN_TYPE_INTEGER, Token};
use std::collections::HashMap;

///
/// # Example
///
/// ```rust
/// use lexxor::{Lexxor, Lexxer};
/// use lexxor::token::{TOKEN_TYPE_WHITESPACE, TOKEN_TYPE_FLOAT, TOKEN_TYPE_INTEGER, TOKEN_TYPE_SYMBOL};
/// use lexxor::input::InputString;
/// use lexxor::matcher::float::FloatMatcher;
/// use lexxor::matcher::integer::IntegerMatcher;
/// use lexxor::matcher::symbol::SymbolMatcher;
/// use lexxor::matcher::whitespace::WhitespaceMatcher;
///
/// let mut lexxor: Box<dyn Lexxer> = Box::new(Lexxor::<512>::new(
/// Box::new(InputString::new(String::from("1.0 5 0.012345 .9 00.00 100.0 4."))),
/// vec![
///     Box::new(FloatMatcher{ index: 0, precedence: 0, dot: false, float:false, running: true }),
///     Box::new(SymbolMatcher { index: 0, precedence: 0, running: true }),
///     Box::new(IntegerMatcher { index: 0, precedence: 0, running: true }),
///     Box::new(WhitespaceMatcher { index: 0, column: 0,line: 0,precedence: 0, running: true }),
/// ]
/// ));
///
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "1.0" && t.token_type == TOKEN_TYPE_FLOAT && t.line == 1 && t.column == 1));
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "5" && t.token_type == TOKEN_TYPE_INTEGER && t.line == 1 && t.column == 5));
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "0.012345" && t.token_type == TOKEN_TYPE_FLOAT && t.line == 1 && t.column == 7));
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "." && t.token_type == TOKEN_TYPE_SYMBOL && t.line == 1 && t.column == 16));
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "9" && t.token_type == TOKEN_TYPE_INTEGER && t.line == 1 && t.column == 17));
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "00.00" && t.token_type == TOKEN_TYPE_FLOAT && t.line == 1 && t.column == 19));
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "100.0" && t.token_type == TOKEN_TYPE_FLOAT && t.line == 1 && t.column == 25));
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "4" && t.token_type == TOKEN_TYPE_INTEGER && t.line == 1 && t.column == 31));
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "." && t.token_type == TOKEN_TYPE_SYMBOL && t.line == 1 && t.column == 32));
/// assert!(matches!(lexxor.next_token(), Ok(None)));
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
        match oc {
            None => self.generate_integer_token(value),
            Some(c) => {
                if c.is_numeric() {
                    self.index += 1;
                    MatcherResult::Running()
                } else {
                    self.generate_integer_token(value)
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

impl IntegerMatcher {
    #[inline(always)]
    fn generate_integer_token(&mut self, value: &[char]) -> MatcherResult {
        self.running = false;
        if self.index > 0 {
            MatcherResult::Matched(Token {
                value: value[0..self.index].iter().collect(),
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
    use crate::input::InputString;
    use crate::matcher::Matcher;
    use crate::matcher::integer::IntegerMatcher;
    use crate::matcher::whitespace::WhitespaceMatcher;
    use crate::{Lexxor, LexxError, Lexxer};

    #[test]
    fn matcher_integer_matches_integer() {
        let mut lexxor: Box<dyn Lexxer> = Box::new(Lexxor::<512>::new(
            Box::new(InputString::new(String::from("4"))),
            vec![Box::new(IntegerMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        ));

        match lexxor.next_token() {
            Err(e) => match e {
                LexxError::TokenNotFound(_) => {
                    unreachable!("Should not have failed parsing file");
                }
                LexxError::Error(_) => {
                    unreachable!("Should not have failed parsing file");
                }
            },
            Ok(Some(t)) => {
                assert_eq!(t.value, "4")
            }
            Ok(None) => {
                unreachable!("Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_integer_matches_big_integer() {
        let mut lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from("6346357587454"))),
            vec![Box::new(IntegerMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        match lexxor.next_token() {
            Err(e) => match e {
                LexxError::TokenNotFound(_) => {
                    unreachable!("Should not have failed parsing file");
                }
                LexxError::Error(_) => {
                    unreachable!("Should not have failed parsing file");
                }
            },
            Ok(Some(t)) => {
                assert_eq!(t.value, "6346357587454")
            }
            Ok(None) => {
                unreachable!("Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_integer_matches_integer_not_float() {
        let mut lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from("5.5"))),
            vec![Box::new(IntegerMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        match lexxor.next_token() {
            Err(e) => match e {
                LexxError::TokenNotFound(_) => {
                    unreachable!("Should not have failed parsing file");
                }
                LexxError::Error(_) => {
                    unreachable!("Should not have failed parsing file");
                }
            },
            Ok(Some(t)) => {
                assert_eq!(t.value, "5")
            }
            Ok(None) => {
                unreachable!("Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_integer_matches_multiple() {
        let mut lexxor = Lexxor::<512>::new(
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

        match lexxor.next_token() {
            Err(e) => match e {
                LexxError::TokenNotFound(_) => {
                    unreachable!("Should not have failed parsing file");
                }
                LexxError::Error(_) => {
                    unreachable!("Should not have failed parsing file");
                }
            },
            Ok(Some(t)) => {
                assert_eq!(t.value, "2")
            }
            Ok(None) => {
                unreachable!("Should not hit None");
            }
        }

        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == crate::token::TOKEN_TYPE_WHITESPACE)
        );

        match lexxor.next_token() {
            Err(e) => match e {
                LexxError::TokenNotFound(_) => {
                    unreachable!("Should not have failed parsing file");
                }
                LexxError::Error(_) => {
                    unreachable!("Should not have failed parsing file");
                }
            },
            Ok(Some(t)) => {
                assert_eq!(t.value, "3")
            }
            Ok(None) => {
                unreachable!("Should not hit None");
            }
        }

        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == crate::token::TOKEN_TYPE_WHITESPACE)
        );

        match lexxor.next_token() {
            Err(e) => match e {
                LexxError::TokenNotFound(_) => {
                    unreachable!("Should not have failed parsing file");
                }
                LexxError::Error(_) => {
                    unreachable!("Should not have failed parsing file");
                }
            },
            Ok(Some(t)) => {
                assert_eq!(t.value, "4")
            }
            Ok(None) => {
                unreachable!("Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_integer_resets_properly() {
        let mut matcher = IntegerMatcher {
            index: 5,
            precedence: 0,
            running: false,
        };

        let mut ctx = Box::new(std::collections::HashMap::<String, i32>::new());
        matcher.reset(&mut ctx);

        assert_eq!(matcher.index, 0);
        assert!(matcher.running);
    }

    #[test]
    fn matcher_integer_handles_non_numeric_input() {
        let mut lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from("abc"))),
            vec![Box::new(IntegerMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        // Should fail to match anything
        match lexxor.next_token() {
            Err(LexxError::TokenNotFound(_)) => {
                // This is expected
            }
            _ => {
                unreachable!("Should have failed to match");
            }
        }
    }

    #[test]
    fn matcher_integer_handles_empty_input() {
        let mut lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from(""))),
            vec![Box::new(IntegerMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        // Should return None for empty input
        assert!(matches!(lexxor.next_token(), Ok(None)));
    }

    #[test]
    fn matcher_integer_returns_correct_token_type() {
        let mut lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from("123"))),
            vec![Box::new(IntegerMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        match lexxor.next_token() {
            Ok(Some(t)) => {
                assert_eq!(t.value, "123");
                assert_eq!(t.token_type, crate::token::TOKEN_TYPE_INTEGER);
                assert_eq!(t.precedence, 0);
            }
            _ => {
                unreachable!("Should have matched an integer");
            }
        }
    }

    #[test]
    fn matcher_integer_respects_precedence() {
        let custom_precedence = 5;
        let mut lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from("42"))),
            vec![Box::new(IntegerMatcher {
                index: 0,
                precedence: custom_precedence,
                running: true,
            })],
        );

        match lexxor.next_token() {
            Ok(Some(t)) => {
                assert_eq!(t.precedence, custom_precedence);
            }
            _ => {
                unreachable!("Should have matched an integer");
            }
        }
    }
}

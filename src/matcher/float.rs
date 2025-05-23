/// The float matcher matches floating point numbers. To qualify as floating point, the numbers must
/// start and end with a numeric digit and have a period within them. For example `1.0`. Thus,
/// `.1` and `1.` do not qualify as floating point numbers.
use crate::matcher::{Matcher, MatcherResult};
use crate::token::{TOKEN_TYPE_FLOAT, Token};
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
///     Box::new(WhitespaceMatcher { index: 0, column: 0,line: 0,precedence: 0, running: true}),
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

#[cfg(test)]
mod tests {
    use crate::input::InputString;
    use crate::matcher::Matcher;
    use crate::matcher::float::FloatMatcher;
    use crate::matcher::integer::IntegerMatcher;
    use crate::matcher::symbol::SymbolMatcher;
    use crate::matcher::whitespace::WhitespaceMatcher;
    use crate::token::TOKEN_TYPE_FLOAT;
    use crate::{LexxError, Lexxer, Lexxor};

    #[test]
    fn matcher_float_matches_simple_float() {
        let mut lexxor: Box<dyn Lexxer> = Box::new(Lexxor::<512>::new(
            Box::new(InputString::new(String::from("1.0"))),
            vec![Box::new(FloatMatcher {
                index: 0,
                precedence: 0,
                dot: false,
                float: false,
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
                assert_eq!(t.value, "1.0");
                assert_eq!(t.token_type, TOKEN_TYPE_FLOAT);
            }
            Ok(None) => {
                unreachable!("Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_float_matches_decimal_float() {
        let mut lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from("0.012345"))),
            vec![Box::new(FloatMatcher {
                index: 0,
                precedence: 0,
                dot: false,
                float: false,
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
                assert_eq!(t.value, "0.012345");
                assert_eq!(t.token_type, TOKEN_TYPE_FLOAT);
            }
            Ok(None) => {
                unreachable!("Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_float_matches_leading_zeros() {
        let mut lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from("00.00"))),
            vec![Box::new(FloatMatcher {
                index: 0,
                precedence: 0,
                dot: false,
                float: false,
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
                assert_eq!(t.value, "00.00");
                assert_eq!(t.token_type, TOKEN_TYPE_FLOAT);
            }
            Ok(None) => {
                unreachable!("Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_float_does_not_match_dot_first() {
        let mut lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from(".9"))),
            vec![
                Box::new(FloatMatcher {
                    index: 0,
                    precedence: 0,
                    dot: false,
                    float: false,
                    running: true,
                }),
                Box::new(SymbolMatcher {
                    index: 0,
                    precedence: 0,
                    running: true,
                }),
            ],
        );

        // Should match the dot as a symbol
        match lexxor.next_token() {
            Ok(Some(t)) => {
                assert_eq!(t.value, ".");
                assert_eq!(t.token_type, crate::token::TOKEN_TYPE_SYMBOL);
            }
            _ => {
                unreachable!("Should have matched a symbol");
            }
        }
    }

    #[test]
    fn matcher_float_does_not_match_dot_last() {
        let mut lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from("4."))),
            vec![
                Box::new(FloatMatcher {
                    index: 0,
                    precedence: 0,
                    dot: false,
                    float: false,
                    running: true,
                }),
                Box::new(SymbolMatcher {
                    index: 0,
                    precedence: 0,
                    running: true,
                }),
                Box::new(IntegerMatcher {
                    index: 0,
                    precedence: 0,
                    running: true,
                }),
            ],
        );

        // Should not match as a float
        match lexxor.next_token() {
            Ok(Some(t)) => {
                assert_ne!(t.token_type, TOKEN_TYPE_FLOAT);
            }
            _ => {
                unreachable!("Should have matched something");
            }
        }
    }

    #[test]
    fn matcher_float_matches_multiple() {
        let mut lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from("1.0 2.5 3.14"))),
            vec![
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
            ],
        );

        // First float
        match lexxor.next_token() {
            Ok(Some(t)) => {
                assert_eq!(t.value, "1.0");
                assert_eq!(t.token_type, TOKEN_TYPE_FLOAT);
            }
            _ => {
                unreachable!("Should have matched a float");
            }
        }

        // Whitespace
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == crate::token::TOKEN_TYPE_WHITESPACE)
        );

        // Second float
        match lexxor.next_token() {
            Ok(Some(t)) => {
                assert_eq!(t.value, "2.5");
                assert_eq!(t.token_type, TOKEN_TYPE_FLOAT);
            }
            _ => {
                unreachable!("Should have matched a float");
            }
        }

        // Whitespace
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == crate::token::TOKEN_TYPE_WHITESPACE)
        );

        // Third float
        match lexxor.next_token() {
            Ok(Some(t)) => {
                assert_eq!(t.value, "3.14");
                assert_eq!(t.token_type, TOKEN_TYPE_FLOAT);
            }
            _ => {
                unreachable!("Should have matched a float");
            }
        }
    }

    #[test]
    fn matcher_float_resets_properly() {
        let mut matcher = FloatMatcher {
            index: 5,
            precedence: 0,
            dot: true,
            float: true,
            running: false,
        };

        let mut ctx = Box::new(std::collections::HashMap::<String, i32>::new());
        matcher.reset(&mut ctx);

        assert_eq!(matcher.index, 0);
        assert!(!matcher.dot);
        assert!(!matcher.float);
        assert!(matcher.running);
    }
}


use crate::matcher::{Matcher, MatcherResult};
use crate::token::{Token, TOKEN_TYPE_WORD};
use std::collections::HashMap;

/// The SymbolMatcher matches any series of characters that do NOT match `is_whitespace()` or
/// `c.is_alphanumeric()`. That is, any character that is not a number, letter or whitespace
/// will be matched by this matcher.
///
/// # Example
///
/// ```rust
/// use lexx::{Lexx, Lexxer};
/// use lexx::token::{TOKEN_TYPE_EXACT, TOKEN_TYPE_SYMBOL};
/// use lexx::input::InputString;
/// use lexx::matcher_exact::ExactMatcher;
/// use lexx::matcher_symbol::SymbolMatcher;
///
/// let lexx_input = InputString::new(String::from("^%$gxv llj)9^%d$rrr"));
///
/// let mut lexx: Box<dyn Lexxer> = Box::new(Lexx::<512>::new(
///     Box::new(lexx_input),
///     vec![
///         Box::new(SymbolMatcher { index: 0, precedence: 0, running: true }),
///         // Note the precedence of 1 will cause the ExactMatcher to be be returned
///         // when the SymbolMatcher would have matched the same, or a longer thing.
///         Box::new(ExactMatcher::build_exact_matcher(vec!["^", "$gxv ", "gxv ", "llj)9", "d$rrr"], TOKEN_TYPE_EXACT, 1)),
///     ]
/// ));
///
/// // Because of the precedence settings the ExactMatcher matched "^"
/// // even though the SymbolMtcher would have matched "^%$"
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "^" && t.token_type == TOKEN_TYPE_EXACT && t.line == 1 && t.column == 1));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "%$" && t.token_type == TOKEN_TYPE_SYMBOL && t.line == 1 && t.column == 2));
/// // NOTE that "$gxv " is NOT found because the symbol matcher matched "%$"
/// // the ExactMatcher gave up at '%' and never saw '$gxv '
/// // matchers can not find matches that start inside the valid matches of other matchers.
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "gxv " && t.token_type == TOKEN_TYPE_EXACT && t.line == 1 && t.column == 4));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "llj)9" && t.token_type == TOKEN_TYPE_EXACT && t.line == 1 && t.column == 8));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "^" && t.token_type == TOKEN_TYPE_EXACT && t.line == 1 && t.column == 13));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "%" && t.token_type == TOKEN_TYPE_SYMBOL && t.line == 1 && t.column == 14));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "d$rrr" && t.token_type == TOKEN_TYPE_EXACT && t.line == 1 && t.column == 15));
/// assert!(matches!(lexx.next_token(), Ok(None)));
/// ```
#[derive(Clone, Debug, Copy)]
pub struct WordMatcher {
    /// Current size of the ongoing match.
    pub index: usize,
    /// This matchers precedence.
    pub precedence: u8,
    /// If the matcher is currently running.
    pub running: bool,
}

impl Matcher for WordMatcher {
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
            None => {
                self.running = false;
                self.generate_word_token(value)
            }
            Some(c) => {
                if c.is_alphabetic() {
                    self.index += 1;
                    MatcherResult::Running()
                } else {
                    self.running = false;
                    self.generate_word_token(value)
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

impl WordMatcher {
    #[inline(always)]
    fn generate_word_token(&mut self, value: &[char]) -> MatcherResult {
        if self.index > 0 {
            MatcherResult::Matched(Token {
                value: value[0..self.index].into_iter().collect(),
                token_type: TOKEN_TYPE_WORD,
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
    use crate::matcher_whitespace::WhitespaceMatcher;
    use crate::matcher_word::WordMatcher;
    use crate::token::TOKEN_TYPE_WORD;
    use crate::{Lexx, LexxError, Lexxer};
    use crate::input::InputString;

    #[test]
    fn matcher_word_matches_word() {
        let mut lexx: Box<dyn Lexxer> = Box::new(Lexx::<512>::new(
            Box::new(InputString::new(String::from("The"))),
            vec![Box::new(WordMatcher {
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
                assert_eq!(t.value, "The");
                assert_eq!(t.token_type, TOKEN_TYPE_WORD)
            }
            Ok(None) => {
                assert!(false, "Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_word_matches_word_with_symbol() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("Stop!"))),
            vec![Box::new(WordMatcher {
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
                assert_eq!(t.value, "Stop");
                assert_eq!(t.token_type, TOKEN_TYPE_WORD)
            }
            Ok(None) => {
                assert!(false, "Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_word_matches_word_with_number() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("Stop1"))),
            vec![Box::new(WordMatcher {
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
                assert_eq!(t.value, "Stop");
                assert_eq!(t.token_type, TOKEN_TYPE_WORD)
            }
            Ok(None) => {
                assert!(false, "Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_word_matches_multiple_words() {
        use crate::token::TOKEN_TYPE_WHITESPACE;
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("The quick brown fox qquick"))),
            vec![
                Box::new(WordMatcher {
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

        assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "The"));
        assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "quick"));
        assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "brown"));
        assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "fox"));
        assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));

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
                assert_eq!(t.value, "qquick");
                assert_eq!(t.line, 1);
                assert_eq!(t.column, 21);
            }
            Ok(None) => {
                assert!(false, "Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_word_matches_multiple_words_and_lines() {
        use crate::token::TOKEN_TYPE_WHITESPACE;
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from(
                "The quick\rbrown\nfox jumped\r\nover the lazy dog",
            ))),
            vec![
                Box::new(WordMatcher {
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

        assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "The"));
        assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "quick"));
        assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "brown"));
        assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "fox"));
        assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "jumped"));
        assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "over"));
        assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "the"));
        assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "lazy"));
        assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
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
                assert_eq!(t.value, "dog");
                assert_eq!(t.line, 3);
                assert_eq!(t.column, 15);
            }
            Ok(None) => {
                assert!(false, "Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_word_does_not_match_number() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("512"))),
            vec![Box::new(WordMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        match lexx.next_token() {
            Err(e) => match e {
                LexxError::TokenNotFound(e) => {
                    assert_eq!(e, "Could not resolve token at 1, 1: 'Some('5')'.");
                }
                LexxError::Error(_) => {
                    assert!(false, "Should not get an error");
                }
            },
            Ok(_t) => {
                assert!(false, "should not have matched 512");
            }
        }
    }

    #[test]
    fn matcher_word_does_not_match_space() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from(" "))),
            vec![Box::new(WordMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        match lexx.next_token() {
            Err(e) => match e {
                LexxError::TokenNotFound(e) => {
                    assert_eq!(e, "Could not resolve token at 1, 1: 'Some(' ')'.");
                }
                LexxError::Error(_) => {
                    assert!(false, "Should not have failed parsing file");
                }
            },
            Ok(_t) => {
                assert!(false, "should not have matched space");
            }
        }
    }

    #[test]
    fn matcher_word_does_not_match_symbol() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("%"))),
            vec![Box::new(WordMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        match lexx.next_token() {
            Err(e) => match e {
                LexxError::TokenNotFound(e) => {
                    assert_eq!(e, "Could not resolve token at 1, 1: 'Some('%')'.");
                }
                LexxError::Error(_) => {
                    assert!(false, "Should not have failed parsing file");
                }
            },
            Ok(_t) => {
                assert!(false, "should not have matched 5");
            }
        }
    }
}

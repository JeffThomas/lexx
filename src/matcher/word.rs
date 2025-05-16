/// The `word` module provides the `WordMatcher`, which matches word tokens in the input stream.
/// A word is typically defined as a sequence of alphabetic characters (such as identifiers or keywords).
/// The matcher recognizes words and produces tokens of type `TOKEN_TYPE_WORD`.
///
/// This module is useful for lexers that need to identify and extract words or identifiers from text.
use crate::matcher::{Matcher, MatcherResult};
use crate::token::{TOKEN_TYPE_WORD, Token};
use std::collections::HashMap;

/// The `WordMatcher` is a matcher that matches word tokens in the input stream.
///
/// # Example
///
/// ```rust
/// use lexxor::{Lexxor, Lexxer};
/// use lexxor::token::{TOKEN_TYPE_EXACT, TOKEN_TYPE_SYMBOL};
/// use lexxor::input::InputString;
/// use lexxor::matcher::exact::ExactMatcher;
/// use lexxor::matcher::symbol::SymbolMatcher;
///
/// let lexxor_input = InputString::new(String::from("^%$gxv llj)9^%d$rrr"));
///
/// let mut lexxor: Box<dyn Lexxer> = Box::new(Lexxor::<512>::new(
///     Box::new(lexxor_input),
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
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "^" && t.token_type == TOKEN_TYPE_EXACT && t.line == 1 && t.column == 1));
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "%$" && t.token_type == TOKEN_TYPE_SYMBOL && t.line == 1 && t.column == 2));
/// // NOTE that "$gxv " is NOT found because the symbol matcher matched "%$"
/// // the ExactMatcher gave up at '%' and never saw '$gxv '
/// // matchers can not find matches that start inside the valid matches of other matchers.
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "gxv " && t.token_type == TOKEN_TYPE_EXACT && t.line == 1 && t.column == 4));
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "llj)9" && t.token_type == TOKEN_TYPE_EXACT && t.line == 1 && t.column == 8));
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "^" && t.token_type == TOKEN_TYPE_EXACT && t.line == 1 && t.column == 13));
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "%" && t.token_type == TOKEN_TYPE_SYMBOL && t.line == 1 && t.column == 14));
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "d$rrr" && t.token_type == TOKEN_TYPE_EXACT && t.line == 1 && t.column == 15));
/// assert!(matches!(lexxor.next_token(), Ok(None)));
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
        match oc {
            Some(c) if c.is_alphabetic() => {
                self.index += 1;
                MatcherResult::Running()
            }
            _ => {
                self.running = false;
                self.generate_word_token(value)
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

impl WordMatcher {
    #[inline(always)]
    fn generate_word_token(&mut self, value: &[char]) -> MatcherResult {
        if self.index > 0 {
            MatcherResult::Matched(Token {
                value: value[0..self.index].iter().collect(),
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
    use crate::input::InputString;
    use crate::matcher::Matcher;
    use crate::matcher::whitespace::WhitespaceMatcher;
    use crate::matcher::word::WordMatcher;
    use crate::token::TOKEN_TYPE_WORD;
    use crate::{LexxError, Lexxer, Lexxor};

    #[test]
    fn matcher_word_matches_word() {
        let mut lexxor: Box<dyn Lexxer> = Box::new(Lexxor::<512>::new(
            Box::new(InputString::new(String::from("The"))),
            vec![Box::new(WordMatcher {
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
                assert_eq!(t.value, "The");
                assert_eq!(t.token_type, TOKEN_TYPE_WORD)
            }
            Ok(None) => {
                unreachable!("Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_word_matches_word_with_symbol() {
        let mut lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from("Stop!"))),
            vec![Box::new(WordMatcher {
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
                assert_eq!(t.value, "Stop");
                assert_eq!(t.token_type, TOKEN_TYPE_WORD)
            }
            Ok(None) => {
                unreachable!("Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_word_matches_word_with_number() {
        let mut lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from("Stop1"))),
            vec![Box::new(WordMatcher {
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
                assert_eq!(t.value, "Stop");
                assert_eq!(t.token_type, TOKEN_TYPE_WORD)
            }
            Ok(None) => {
                unreachable!("Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_word_matches_multiple_words() {
        use crate::token::TOKEN_TYPE_WHITESPACE;
        let mut lexxor = Lexxor::<512>::new(
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

        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "The"));
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE)
        );
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "quick"));
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE)
        );
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "brown"));
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE)
        );
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "fox"));
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE)
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
                assert_eq!(t.value, "qquick");
                assert_eq!(t.line, 1);
                assert_eq!(t.column, 21);
            }
            Ok(None) => {
                unreachable!("Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_word_matches_multiple_words_and_lines() {
        use crate::token::TOKEN_TYPE_WHITESPACE;
        let mut lexxor = Lexxor::<512>::new(
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

        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "The"));
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE)
        );
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "quick"));
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE)
        );
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "brown"));
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE)
        );
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "fox"));
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE)
        );
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "jumped"));
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE)
        );
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "over"));
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE)
        );
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "the"));
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE)
        );
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "lazy"));
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE)
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
                assert_eq!(t.value, "dog");
                assert_eq!(t.line, 3);
                assert_eq!(t.column, 15);
            }
            Ok(None) => {
                unreachable!("Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_word_does_not_match_number() {
        let mut lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from("512"))),
            vec![Box::new(WordMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        match lexxor.next_token() {
            Err(e) => match e {
                LexxError::TokenNotFound(e) => {
                    assert_eq!(e, "Could not resolve token at 1, 1: 'Some('5')'.");
                }
                LexxError::Error(_) => {
                    unreachable!("Should not get an error");
                }
            },
            Ok(_t) => {
                unreachable!("should not have matched 512");
            }
        }
    }

    #[test]
    fn matcher_word_does_not_match_space() {
        let mut lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from(" "))),
            vec![Box::new(WordMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        match lexxor.next_token() {
            Err(e) => match e {
                LexxError::TokenNotFound(e) => {
                    assert_eq!(e, "Could not resolve token at 1, 1: 'Some(' ')'.");
                }
                LexxError::Error(_) => {
                    unreachable!("Should not have failed parsing file");
                }
            },
            Ok(_t) => {
                unreachable!("should not have matched space");
            }
        }
    }

    #[test]
    fn matcher_word_does_not_match_symbol() {
        let mut lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from("%"))),
            vec![Box::new(WordMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        match lexxor.next_token() {
            Err(e) => match e {
                LexxError::TokenNotFound(e) => {
                    assert_eq!(e, "Could not resolve token at 1, 1: 'Some('%')'.");
                }
                LexxError::Error(_) => {
                    unreachable!("Should not have failed parsing file");
                }
            },
            Ok(_t) => {
                unreachable!("should not have matched 5");
            }
        }
    }

    #[test]
    fn test_word_with_unicode_letters() {
        // Test that the WordMatcher correctly handles Unicode alphabetic characters
        let mut lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from("résumé привет こんにちは"))),
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

        // Should match "résumé" (French word with accents)
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "résumé" && t.token_type == TOKEN_TYPE_WORD)
        );

        // Should match whitespace
        assert!(matches!(lexxor.next_token(), Ok(Some(_))));

        // Should match "привет" (Russian word)
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "привет" && t.token_type == TOKEN_TYPE_WORD)
        );

        // Should match whitespace
        assert!(matches!(lexxor.next_token(), Ok(Some(_))));

        // Should match "こんにちは" (Japanese word)
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "こんにちは" && t.token_type == TOKEN_TYPE_WORD)
        );

        // No more tokens
        assert!(matches!(lexxor.next_token(), Ok(None)));
    }

    #[test]
    fn test_word_with_apostrophes() {
        // Some languages consider apostrophes part of words, but our implementation doesn't
        let mut lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from("don't can't I'll"))),
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

        // Should match "don" but not the apostrophe
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "don" && t.token_type == TOKEN_TYPE_WORD)
        );

        // Should fail on apostrophe
        assert!(matches!(
            lexxor.next_token(),
            Err(LexxError::TokenNotFound(_))
        ));

        // Should match "t"
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "t" && t.token_type == TOKEN_TYPE_WORD)
        );

        // Should match whitespace
        assert!(matches!(lexxor.next_token(), Ok(Some(_))));

        // Similar pattern for "can't"
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "can" && t.token_type == TOKEN_TYPE_WORD)
        );
        assert!(matches!(
            lexxor.next_token(),
            Err(LexxError::TokenNotFound(_))
        ));
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "t" && t.token_type == TOKEN_TYPE_WORD)
        );
    }

    #[test]
    fn test_word_with_mixed_content() {
        // Test words with mixed content (letters, numbers, symbols)
        let mut lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from("abc123 def!ghi"))),
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

        // Should match "abc" but not the numbers
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "abc" && t.token_type == TOKEN_TYPE_WORD)
        );

        // Should fail on numbers
        assert!(matches!(
            lexxor.next_token(),
            Err(LexxError::TokenNotFound(_))
        ));
        assert!(matches!(
            lexxor.next_token(),
            Err(LexxError::TokenNotFound(_))
        ));
        assert!(matches!(
            lexxor.next_token(),
            Err(LexxError::TokenNotFound(_))
        ));

        // Should match whitespace
        assert!(matches!(lexxor.next_token(), Ok(Some(_))));

        // Should match "def" but not the exclamation mark
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "def" && t.token_type == TOKEN_TYPE_WORD)
        );

        // Should fail on exclamation mark
        assert!(matches!(
            lexxor.next_token(),
            Err(LexxError::TokenNotFound(_))
        ));

        // Should match "ghi"
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "ghi" && t.token_type == TOKEN_TYPE_WORD)
        );
    }

    #[test]
    fn test_word_with_precedence() {
        // Test that precedence is respected when multiple matchers could match
        let mut lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from("keyword"))),
            vec![
                // Lower precedence word matcher
                Box::new(WordMatcher {
                    index: 0,
                    precedence: 0,
                    running: true,
                }),
                // Higher precedence exact matcher for the same word
                Box::new(crate::matcher::exact::ExactMatcher::build_exact_matcher(
                    vec!["keyword"],
                    crate::token::TOKEN_TYPE_EXACT,
                    1, // Higher precedence
                )),
            ],
        );

        // The exact matcher should win due to higher precedence
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "keyword" && t.token_type == crate::token::TOKEN_TYPE_EXACT)
        );
    }

    #[test]
    fn test_word_at_end_of_input() {
        // Test that a word at the end of input is properly matched
        let mut lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from("end"))),
            vec![Box::new(WordMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        // Should match "end"
        assert!(
            matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "end" && t.token_type == TOKEN_TYPE_WORD)
        );

        // No more tokens
        assert!(matches!(lexxor.next_token(), Ok(None)));
    }

    #[test]
    fn test_reset_functionality() {
        use std::collections::HashMap;

        // Test that the reset function properly resets the matcher state
        let mut matcher = WordMatcher {
            index: 10, // Simulate some previous matching
            precedence: 0,
            running: false,
        };

        // Reset the matcher
        let mut ctx = Box::new(HashMap::new());
        matcher.reset(&mut ctx);

        // Verify that the matcher state has been reset
        assert_eq!(matcher.index, 0);
        assert!(matcher.running);
    }

    #[test]
    fn test_empty_input() {
        // Test behavior with empty input
        let mut lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from(""))),
            vec![Box::new(WordMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        // Should return None for empty input
        assert!(matches!(lexxor.next_token(), Ok(None)));
    }
}

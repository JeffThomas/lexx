/// The `symbol` module provides the `SymbolMatcher`, which matches symbol tokens in the input stream.
/// Symbols are typically non-alphanumeric characters (such as punctuation or operators) that are not part of words or numbers.
/// The matcher recognizes contiguous runs of symbol characters and produces tokens of type `TOKEN_TYPE_SYMBOL`.
///
/// This module is useful for lexers that need to identify and extract symbols (e.g., operators, punctuation) from text.
use crate::matcher::{Matcher, MatcherResult};
use crate::token::{Token, TOKEN_TYPE_SYMBOL};
use std::collections::HashMap;

/// The `SymbolMatcher` is a matcher that matches symbol tokens in the input stream.
///
/// # Example
///
/// ```rust
/// use lexx::{Lexx, Lexxer};
/// use lexx::token::{TOKEN_TYPE_EXACT, TOKEN_TYPE_SYMBOL};
/// use lexx::input::InputString;
/// use lexx::matcher::exact::ExactMatcher;
/// use lexx::matcher::symbol::SymbolMatcher;
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
pub struct SymbolMatcher {
    /// Current size of the ongoing match.
    pub index: usize,
    /// This matchers precedence.
    pub precedence: u8,
    /// If the matcher is currently running.
    pub running: bool,
}

impl Matcher for SymbolMatcher {
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
            Some(c) if !c.is_whitespace() && !c.is_alphanumeric() => {
                self.index += 1;
                MatcherResult::Running()
            }
            _ => self.generate_symbol_token(value),
        }
    }
    fn is_running(&self) -> bool {
        self.running
    }
    fn precedence(&self) -> u8 {
        self.precedence
    }
}

impl SymbolMatcher {
    #[inline(always)]
    fn generate_symbol_token(&mut self, value: &[char]) -> MatcherResult {
        self.running = false;
        if self.index > 0 {
            MatcherResult::Matched(Token {
                value: value[0..self.index].iter().collect(),
                token_type: TOKEN_TYPE_SYMBOL,
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
    use crate::matcher::symbol::SymbolMatcher;
    use crate::matcher::whitespace::WhitespaceMatcher;
    use crate::matcher::Matcher;
    use crate::{Lexx, LexxError, Lexxer};
    use crate::token::TOKEN_TYPE_SYMBOL;

    #[test]
    fn matcher_symbol_matches_single_symbol() {
        let mut lexx: Box<dyn Lexxer> = Box::new(Lexx::<512>::new(
            Box::new(InputString::new(String::from("@"))),
            vec![Box::new(SymbolMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        ));

        match lexx.next_token() {
            Err(e) => match e {
                LexxError::TokenNotFound(_) => {
                    unreachable!("Should not have failed parsing file");
                }
                LexxError::Error(_) => {
                    unreachable!("Should not have failed parsing file");
                }
            },
            Ok(Some(t)) => {
                assert_eq!(t.value, "@");
                assert_eq!(t.token_type, TOKEN_TYPE_SYMBOL);
            }
            Ok(None) => {
                unreachable!("Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_symbol_matches_multiple_symbols() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("!@#$%^&*"))),
            vec![Box::new(SymbolMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        match lexx.next_token() {
            Err(e) => match e {
                LexxError::TokenNotFound(_) => {
                    unreachable!("Should not have failed parsing file");
                }
                LexxError::Error(_) => {
                    unreachable!("Should not have failed parsing file");
                }
            },
            Ok(Some(t)) => {
                assert_eq!(t.value, "!@#$%^&*");
                assert_eq!(t.token_type, TOKEN_TYPE_SYMBOL);
            }
            Ok(None) => {
                unreachable!("Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_symbol_stops_at_alphanumeric() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("@#$abc"))),
            vec![Box::new(SymbolMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        match lexx.next_token() {
            Ok(Some(t)) => {
                assert_eq!(t.value, "@#$");
                assert_eq!(t.token_type, TOKEN_TYPE_SYMBOL);
            }
            _ => {
                unreachable!("Should have matched symbols");
            }
        }
    }

    #[test]
    fn matcher_symbol_stops_at_whitespace() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("@#$ %^&"))),
            vec![
                Box::new(SymbolMatcher {
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

        // First symbol group
        match lexx.next_token() {
            Ok(Some(t)) => {
                assert_eq!(t.value, "@#$");
                assert_eq!(t.token_type, TOKEN_TYPE_SYMBOL);
            }
            _ => {
                unreachable!("Should have matched symbols");
            }
        }

        // Whitespace
        assert!(
            matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == crate::token::TOKEN_TYPE_WHITESPACE)
        );

        // Second symbol group
        match lexx.next_token() {
            Ok(Some(t)) => {
                assert_eq!(t.value, "%^&");
                assert_eq!(t.token_type, TOKEN_TYPE_SYMBOL);
            }
            _ => {
                unreachable!("Should have matched symbols");
            }
        }
    }

    #[test]
    fn matcher_symbol_handles_empty_input() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from(""))),
            vec![Box::new(SymbolMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        // Should return None for empty input
        assert!(matches!(lexx.next_token(), Ok(None)));
    }

    #[test]
    fn matcher_symbol_handles_non_symbol_input() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("abc123"))),
            vec![Box::new(SymbolMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        // Should fail to match anything
        match lexx.next_token() {
            Err(LexxError::TokenNotFound(_)) => {
                // This is expected
            }
            _ => {
                unreachable!("Should have failed to match");
            }
        }
    }

    #[test]
    fn matcher_symbol_resets_properly() {
        let mut matcher = SymbolMatcher {
            index: 5,
            precedence: 0,
            running: false,
        };
        
        let mut ctx = Box::new(std::collections::HashMap::<String, i32>::new());
        matcher.reset(&mut ctx);
        
        assert_eq!(matcher.index, 0);
        assert_eq!(matcher.running, true);
    }

    #[test]
    fn matcher_symbol_returns_correct_token_type() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("+-*/"))),
            vec![Box::new(SymbolMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        match lexx.next_token() {
            Ok(Some(t)) => {
                assert_eq!(t.value, "+-*/");
                assert_eq!(t.token_type, TOKEN_TYPE_SYMBOL);
                assert_eq!(t.precedence, 0);
            }
            _ => {
                unreachable!("Should have matched symbols");
            }
        }
    }

    #[test]
    fn matcher_symbol_respects_precedence() {
        let custom_precedence = 5;
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("++"))),
            vec![Box::new(SymbolMatcher {
                index: 0,
                precedence: custom_precedence,
                running: true,
            })],
        );

        match lexx.next_token() {
            Ok(Some(t)) => {
                assert_eq!(t.precedence, custom_precedence);
            }
            _ => {
                unreachable!("Should have matched symbols");
            }
        }
    }

    #[test]
    fn matcher_symbol_handles_unicode_symbols() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("§¶†‡"))),
            vec![Box::new(SymbolMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        match lexx.next_token() {
            Ok(Some(t)) => {
                assert_eq!(t.value, "§¶†‡");
                assert_eq!(t.token_type, TOKEN_TYPE_SYMBOL);
            }
            _ => {
                unreachable!("Should have matched symbols");
            }
        }
    }
}

/// The `exact` module provides the `ExactMatcher`, which matches strings exactly as specified.
/// It allows users to define a list of strings to match against, ensuring that only exact matches
/// are recognized, regardless of their position in the input stream.
use crate::matcher::{Matcher, MatcherResult};
use crate::token::Token;
use std::collections::HashMap;

/// An exact match to be made
#[derive(Clone, Debug)]
pub struct Target {
    /// If this match can still be made or not
    pub matching: bool,
    /// What this match is
    pub target: Box<Vec<char>>,
}

/// The Exact matcher does exactly what you'd expect. You give it a list of strings to match against
/// and it looks EXACTLY for those strings, not being picky about where those strings are.
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
#[derive(Clone, Debug)]
    pub struct ExactMatcher {
    /// Current size of the ongoing match.
    pub index: usize,
    /// This matchers precedence.
    pub precedence: u8,
    /// If the matcher is currently running.
    pub running: bool,
    /// What is the currently found match index, if a longer one is found it will replace this one.
    pub found: Option<usize>,
    /// The array of possible matches to check.
    pub targets: Box<Vec<Target>>,
    /// What token type to return if a match is made.
    pub token_type: u16,
}

impl Matcher for ExactMatcher {
    fn reset(&mut self, _ctx: &mut Box<HashMap<String, i32>>) {
        for t in self.targets.iter_mut() {
            t.matching = true
        }
        self.found = None;
        self.index = 0;
        self.running = true;
    }

    fn find_match(
        &mut self,
        oc: Option<char>,
        _value: &[char],
        _ctx: &mut Box<HashMap<String, i32>>,
    ) -> MatcherResult {
        match oc {
            None => {
                self.running = false;
                for (i, target) in self.targets.iter_mut().enumerate() {
                    if target.matching && target.target.get(self.index).is_none() {
                        self.found = Some(i);
                    }
                }
                self.generate_exact_token()
            }
            Some(c) => {
                self.running = false;
                for (i, target) in self.targets.iter_mut().enumerate() {
                    if target.matching {
                        match target.target.get(self.index) {
                            Some(&m) if m == c => {
                                self.running = true;
                            }
                            Some(_) | None => {
                                target.matching = false;
                                if target.target.get(self.index).is_none() && self.index > 0 {
                                    self.found = Some(i);
                                }
                            }
                        }
                    }
                }
                self.index += 1;
                if !self.running {
                    self.generate_exact_token()
                } else {
                    MatcherResult::Running()
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

impl ExactMatcher {
    /// Build an exact matcher
    ///
    /// # Arguments
    ///
    /// * `matches` - a [vec] of [&str](std::str)s that will be matched
    /// * `token_type` - the token type to produce
    /// * `precedence` - the precedence for this matcher
    ///
    pub fn build_exact_matcher(
        matches: Vec<&str>,
        token_type: u16,
        precedence: u8,
    ) -> ExactMatcher {
        // Pre-allocate with the exact capacity needed
        let mut targets = Vec::with_capacity(matches.len());
        
        for m in matches {
            // Only allocate the vector once with the exact capacity needed
            let mut chars = Vec::with_capacity(m.len());
            // Extend is more efficient than pushing chars one by one
            chars.extend(m.chars());
            
            targets.push(Target {
                matching: true,
                target: Box::new(chars),
            });
        }
        
        ExactMatcher {
            index: 0,
            precedence,
            found: None,
            running: true,
            token_type,
            targets: Box::new(targets),
        }
    }

    #[inline(always)]
    fn generate_exact_token(&mut self) -> MatcherResult {
        match self.found {
            None => MatcherResult::Failed(),
            Some(_) => {
                let i = self.found.unwrap();
                let target = &self.targets.get(i).unwrap().target;
                let token_value: String = target.clone().into_iter().collect();
                let len = token_value.len();
                MatcherResult::Matched(Token {
                    value: token_value,
                    token_type: self.token_type,
                    len,
                    line: 0,
                    column: len,
                    precedence: self.precedence,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::TOKEN_TYPE_EXACT;
    use crate::{Lexx, LexxError, Lexxer};
    use crate::input::InputString;
    use crate::matcher::exact::ExactMatcher;
    use crate::matcher::symbol::SymbolMatcher;
    use crate::matcher::whitespace::WhitespaceMatcher;

    #[test]
    fn matcher_exact_matches_word() {
        let mut lexx: Box<dyn Lexxer> = Box::new(Lexx::<512>::new(
            Box::new(InputString::new(String::from("The"))),
            vec![Box::new(ExactMatcher::build_exact_matcher(
                vec!["The"],
                TOKEN_TYPE_EXACT,
                0,
            ))],
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
                assert_eq!(t.token_type, TOKEN_TYPE_EXACT)
            }
            Ok(None) => {
                assert!(false, "Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_exact_matches_multiple_words() {
        use crate::token::TOKEN_TYPE_WHITESPACE;
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("The quick brown fox qquick"))),
            vec![
                Box::new(ExactMatcher::build_exact_matcher(
                    vec!["brown", "The", "fox", "quick", "qquick"],
                    TOKEN_TYPE_EXACT,
                    0,
                )),
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
    fn matcher_exact_matches_multiple_words_and_lines() {
        use crate::token::TOKEN_TYPE_WHITESPACE;
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from(
                "The quick\rbrown\rfox jumped\rover the lazy dog",
            ))),
            vec![
                Box::new(ExactMatcher::build_exact_matcher(
                    vec![
                        "brown", "The", "fox", "quick", "dog", "over", "jumped", "lazy", "the",
                    ],
                    TOKEN_TYPE_EXACT,
                    0,
                )),
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
                assert_eq!(t.line, 1);
                assert_eq!(t.column, 42);
            }
            Ok(None) => {
                assert!(false, "Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_exact_matches_partial_word() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("Then"))),
            vec![Box::new(ExactMatcher::build_exact_matcher(
                vec!["The"],
                TOKEN_TYPE_EXACT,
                0,
            ))],
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
                assert_eq!(t.value, "The");
                assert_eq!(t.token_type, TOKEN_TYPE_EXACT)
            }
            Ok(None) => {
                assert!(false, "Should not hit None");
            }
        }
    }

    #[test]
    fn example_test() {
        use crate::token::{TOKEN_TYPE_EXACT, TOKEN_TYPE_SYMBOL};
        use crate::Lexx;

        let lexx_input = InputString::new(String::from("^%$gxv llj)9^%d$rrr"));

        let mut lexx = Lexx::<512>::new(
            Box::new(lexx_input),
            vec![
                Box::new(SymbolMatcher {
                    index: 0,
                    precedence: 0,
                    running: true,
                }),
                // Note the precedence of 1 will cause the ExactMatcher to be be returned when
                // when the SymbolMatcher would have matched the same thing.
                Box::new(ExactMatcher::build_exact_matcher(
                    vec!["^", "$gxv ", "gxv ", "llj)9", "d$rrr"],
                    TOKEN_TYPE_EXACT,
                    1,
                )),
            ],
        );

        assert!(
            matches!(lexx.next_token(), Ok(Some(t)) if t.value == "^" && t.token_type == TOKEN_TYPE_EXACT && t.line == 1 && t.column == 1)
        );
        assert!(
            matches!(lexx.next_token(), Ok(Some(t)) if t.value == "%$" && t.token_type == TOKEN_TYPE_SYMBOL && t.line == 1 && t.column == 2)
        );
        // NOTE that "$gxv " is NOT found because the symbol matcher ate "%$"
        assert!(
            matches!(lexx.next_token(), Ok(Some(t)) if t.value == "gxv " && t.token_type == TOKEN_TYPE_EXACT && t.line == 1 && t.column == 4)
        );
        assert!(
            matches!(lexx.next_token(), Ok(Some(t)) if t.value == "llj)9" && t.token_type == TOKEN_TYPE_EXACT && t.line == 1 && t.column == 8)
        );
        assert!(
            matches!(lexx.next_token(), Ok(Some(t)) if t.value == "^" && t.token_type == TOKEN_TYPE_EXACT && t.line == 1 && t.column == 13)
        );
        assert!(
            matches!(lexx.next_token(), Ok(Some(t)) if t.value == "%" && t.token_type == TOKEN_TYPE_SYMBOL && t.line == 1 && t.column == 14)
        );
        assert!(
            matches!(lexx.next_token(), Ok(Some(t)) if t.value == "d$rrr" && t.token_type == TOKEN_TYPE_EXACT && t.line == 1 && t.column == 15)
        );
    }
}

use crate::matcher::{Matcher, MatcherResult};
use crate::token::Token;
use std::collections::HashMap;

/// An exact keyword to be made
#[derive(Clone, Debug)]
pub struct Target {
    /// If this match can still be made or not
    pub matching: bool,
    /// What this match is
    pub target: Box<Vec<char>>,
}

/// The Keyword matcher is very similar to the [ExactMatcher](crate::matcher_exact::ExactMatcher) in that you give it a list of matches
/// to make and it looks EXACTLY for those matches. The difference between this matcher and the
/// [ExactMatcher](crate::matcher_exact::ExactMatcher) is that for `THIS` matcher an exact match must end with a non alpha-numeric
/// character. For example if you give this matcher "match" as a keyword it will NOT match
/// "matches", "matchers" or "match1", "1matcher", "2match" etc.
/// It will match "match ", " match." "---match---" and so on.
///
/// # Example
///
/// ```rust
/// use lexx::{Lexx, Lexxer};
/// use lexx::token::{TOKEN_TYPE_WHITESPACE, TOKEN_TYPE_KEYWORD, TOKEN_TYPE_WORD};
/// use lexx::input::InputString;
/// use lexx::matcher_keyword::KeywordMatcher;
/// use lexx::matcher_whitespace::WhitespaceMatcher;
/// use lexx::matcher_word::WordMatcher;
///
/// let lexx_input = InputString::new(String::from("matcher matching match dog"));
///
/// let mut lexx: Box<dyn Lexxer> = Box::new(Lexx::<512>::new(
///     Box::new(lexx_input),
///     vec![
///         Box::new(WordMatcher { index: 0, precedence: 0, running: true }),
///         Box::new(WhitespaceMatcher { index: 0, column: 0,line: 0,precedence: 0, running: true}),
///         // Note the precedence of 1 will cause the ExactMatcher to be be returned
///         // when the SymbolMatcher would have matched the same, or a longer thing.
///         Box::new(KeywordMatcher::build_matcher_keyword(vec!["match", "dog"], TOKEN_TYPE_KEYWORD, 1)),
///     ]
/// ));
///
/// // Because of the precedence settings the ExactMatcher matched "^"
/// // even though the SymbolMtcher would have matched "^%$"
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "matcher" && t.token_type == TOKEN_TYPE_WORD && t.line == 1 && t.column == 1));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "matching" && t.token_type == TOKEN_TYPE_WORD && t.line == 1 && t.column == 9));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "match" && t.token_type == TOKEN_TYPE_KEYWORD && t.line == 1 && t.column == 18));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "dog" && t.token_type == TOKEN_TYPE_KEYWORD && t.line == 1 && t.column == 24));
/// assert!(matches!(lexx.next_token(), Ok(None)));
/// ```
#[derive(Clone, Debug)]
pub struct KeywordMatcher {
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

impl Matcher for KeywordMatcher {
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
        return match oc {
            None => {
                self.running = false;
                let mut i: usize = 0;
                for target in self.targets.iter_mut() {
                    if target.matching && matches!(target.target.get(self.index), None) {
                        self.found = Some(i)
                    }
                    i += 1
                }
                self.generate_keyword_token()
            }
            Some(c) => {
                self.running = false;
                let mut i: usize = 0;
                for target in self.targets.iter_mut() {
                    if target.matching {
                        match target.target.get(self.index) {
                            None => {
                                target.matching = false;
                                if self.index > 0 && !c.is_alphabetic() {
                                    self.found = Some(i);
                                }
                            }
                            Some(m) => {
                                if *m == c {
                                    self.running = true;
                                } else {
                                    target.matching = false;
                                }
                            }
                        }
                    }
                    i += 1;
                }
                self.index += 1;
                if !self.running {
                    self.generate_keyword_token()
                } else {
                    MatcherResult::Running()
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

impl KeywordMatcher {
    /// Build an keyword matcher
    ///
    /// # Arguments
    ///
    /// * `matches` - a [vec] of [&str](std::str)s that will be matched
    /// * `token_type` - the token type to produce
    /// * `precedence` - the precedence for this matcher
    ///
    pub fn build_matcher_keyword(
        matches: Vec<&str>,
        token_type: u16,
        precedence: u8,
    ) -> KeywordMatcher {
        let mut targets: Box<Vec<Target>> = Box::new(vec![]);
        for m in matches {
            let mut target = Target {
                matching: true,
                target: Box::new(vec![]),
            };
            for c in m.chars() {
                target.target.push(c)
            }
            targets.push(target)
        }
        KeywordMatcher {
            index: 0,
            precedence,
            found: None,
            running: true,
            targets,
            token_type,
        }
    }

    #[inline(always)]
    fn generate_keyword_token(&mut self) -> MatcherResult {
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
    use crate::matcher_keyword::KeywordMatcher;
    use crate::matcher_whitespace::WhitespaceMatcher;
    use crate::token::TOKEN_TYPE_KEYWORD;
    use crate::{Lexx, LexxError, Lexxer};
    use crate::input::InputString;

    #[test]
    fn matcher_exact_matches_word() {
        let mut lexx: Box<dyn Lexxer> = Box::new(Lexx::<512>::new(
            Box::new(InputString::new(String::from("The"))),
            vec![Box::new(KeywordMatcher::build_matcher_keyword(
                vec!["The"],
                TOKEN_TYPE_KEYWORD,
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
                assert_eq!(t.token_type, TOKEN_TYPE_KEYWORD)
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
                Box::new(KeywordMatcher::build_matcher_keyword(
                    vec!["brown", "The", "fox", "quick", "qquick"],
                    TOKEN_TYPE_KEYWORD,
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
                assert_eq!(t.value, "The")
            }
            Ok(None) => {
                assert!(false, "Should not hit None");
            }
        }

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
                assert_eq!(t.value, "quick")
            }
            Ok(None) => {
                assert!(false, "Should not hit None");
            }
        }

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
                assert_eq!(t.value, "brown")
            }
            Ok(None) => {
                assert!(false, "Should not hit None");
            }
        }

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
                assert_eq!(t.value, "fox")
            }
            Ok(None) => {
                assert!(false, "Should not hit None");
            }
        }

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
                "The quick\rbrown\nfox jumped\rover the lazy dog",
            ))),
            vec![
                Box::new(KeywordMatcher::build_matcher_keyword(
                    vec![
                        "brown", "The", "fox", "quick", "dog", "over", "jumped", "lazy", "the",
                    ],
                    TOKEN_TYPE_KEYWORD,
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
                assert_eq!(t.line, 2);
                assert_eq!(t.column, 26);
            }
            Ok(None) => {
                assert!(false, "Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_exact_does_not_match_partial_word() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("Then"))),
            vec![Box::new(KeywordMatcher::build_matcher_keyword(
                vec!["The"],
                TOKEN_TYPE_KEYWORD,
                0,
            ))],
        );

        match lexx.next_token() {
            Err(e) => match e {
                LexxError::TokenNotFound(e) => {
                    assert_eq!(e, "Could not resolve token at 1, 1: 'Some('n')'.");
                }
                LexxError::Error(_) => {
                    assert!(false, "Should not throw error");
                }
            },
            Ok(Some(t)) => {
                assert_eq!(t.value, "The");
                assert!(false, "should not have matched 'The'");
            }
            Ok(None) => {
                assert!(false, "Should not hit None");
            }
        }
    }
}

/// The Keyword matcher is very similar to the [`ExactMatcher`](crate::matcher_exact::ExactMatcher) in that you give it a list of matches
/// to make and it looks EXACTLY for those matches. The difference between this matcher and the
/// [`ExactMatcher`](crate::matcher_exact::ExactMatcher) is that for `THIS` matcher an exact match must end with a non alpha-numeric
/// character. For example if you give this matcher "match" as a keyword it will NOT match
/// "matches", "matchers" or "match1", "1matcher", "2match" etc.
/// It will match "match ", " match." "---match---" and so on.
///
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

///
/// # Example
///
/// ```rust
/// use lexxor::{Lexxor, Lexxer};
/// use lexxor::token::{TOKEN_TYPE_WHITESPACE, TOKEN_TYPE_KEYWORD, TOKEN_TYPE_WORD};
/// use lexxor::input::InputString;
/// use lexxor::matcher::keyword::KeywordMatcher;
/// use lexxor::matcher::whitespace::WhitespaceMatcher;
/// use lexxor::matcher::word::WordMatcher;
///
/// let lexxor_input = InputString::new(String::from("matcher matching match dog"));
///
/// let mut lexxor: Box<dyn Lexxer> = Box::new(Lexxor::<512>::new(
///     Box::new(lexxor_input),
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
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "matcher" && t.token_type == TOKEN_TYPE_WORD && t.line == 1 && t.column == 1));
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "matching" && t.token_type == TOKEN_TYPE_WORD && t.line == 1 && t.column == 9));
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "match" && t.token_type == TOKEN_TYPE_KEYWORD && t.line == 1 && t.column == 18));
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
/// assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "dog" && t.token_type == TOKEN_TYPE_KEYWORD && t.line == 1 && t.column == 24));
/// assert!(matches!(lexxor.next_token(), Ok(None)));
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
        match oc {
            None => {
                self.running = false;
                // Check if any target has matched completely
                for (i, target) in self.targets.iter().enumerate() {
                    if target.matching && target.target.get(self.index).is_none() {
                        self.found = Some(i);
                        break; // Early return once we find a match
                    }
                }
                self.generate_keyword_token()
            }
            Some(c) => {
                // Start with assumption we're not running
                self.running = false;

                // Fast path: if no targets are matching, return immediately
                if !self.targets.iter().any(|t| t.matching) {
                    return self.generate_keyword_token();
                }

                let mut found_potential_match = false;

                for (i, target) in self.targets.iter_mut().enumerate() {
                    if !target.matching {
                        continue; // Skip targets that are already not matching
                    }

                    match target.target.get(self.index) {
                        None => {
                            target.matching = false;
                            // Only consider it a match if we've matched at least one character
                            // and the next character is not alphanumeric
                            if self.index > 0 && !c.is_alphanumeric() {
                                self.found = Some(i);
                                found_potential_match = true;
                            }
                        }
                        Some(m) => {
                            if *m == c {
                                self.running = true; // We have at least one match
                            } else {
                                target.matching = false;
                            }
                        }
                    }
                }

                self.index += 1;

                if !self.running && !found_potential_match {
                    self.generate_keyword_token()
                } else if self.running {
                    MatcherResult::Running()
                } else {
                    self.generate_keyword_token()
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

impl KeywordMatcher {
    /// Build a keyword matcher
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

        KeywordMatcher {
            index: 0,
            precedence,
            found: None,
            running: true,
            token_type,
            targets: Box::new(targets),
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
    use crate::input::InputString;
    use crate::matcher::keyword::KeywordMatcher;
    use crate::matcher::whitespace::WhitespaceMatcher;
    use crate::token::TOKEN_TYPE_KEYWORD;
    use crate::{Lexxor, LexxError, Lexxer};

    #[test]
    fn matcher_exact_matches_word() {
        let mut lexxor: Box<dyn Lexxer> = Box::new(Lexxor::<512>::new(
            Box::new(InputString::new(String::from("The"))),
            vec![Box::new(KeywordMatcher::build_matcher_keyword(
                vec!["The"],
                TOKEN_TYPE_KEYWORD,
                0,
            ))],
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
                assert_eq!(t.token_type, TOKEN_TYPE_KEYWORD)
            }
            Ok(None) => {
                unreachable!("Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_exact_matches_multiple_words() {
        use crate::token::TOKEN_TYPE_WHITESPACE;

        let mut lexxor = Lexxor::<512>::new(
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
                assert_eq!(t.value, "The")
            }
            Ok(None) => {
                unreachable!("Should not hit None");
            }
        }

        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));

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
                assert_eq!(t.value, "quick")
            }
            Ok(None) => {
                unreachable!("Should not hit None");
            }
        }

        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));

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
                assert_eq!(t.value, "brown")
            }
            Ok(None) => {
                unreachable!("Should not hit None");
            }
        }

        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));

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
                assert_eq!(t.value, "fox")
            }
            Ok(None) => {
                unreachable!("Should not hit None");
            }
        }

        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));

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
    fn matcher_exact_matches_multiple_words_and_lines() {
        use crate::token::TOKEN_TYPE_WHITESPACE;

        let mut lexxor = Lexxor::<512>::new(
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

        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "The"));
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "quick"));
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "brown"));
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "fox"));
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "jumped"));
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "over"));
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "the"));
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.value == "lazy"));
        assert!(matches!(lexxor.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE));
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
                assert_eq!(t.line, 2);
                assert_eq!(t.column, 25);
            }
            Ok(None) => {
                unreachable!("Should not hit None");
            }
        }
    }

    #[test]
    fn matcher_exact_does_not_match_partial_word() {
        let mut lexxor = Lexxor::<512>::new(
            Box::new(InputString::new(String::from("Then"))),
            vec![Box::new(KeywordMatcher::build_matcher_keyword(
                vec!["The"],
                TOKEN_TYPE_KEYWORD,
                0,
            ))],
        );

        match lexxor.next_token() {
            Err(e) => match e {
                LexxError::TokenNotFound(e) => {
                    assert_eq!(e, "Could not resolve token at 1, 1: 'Some('n')'.");
                }
                LexxError::Error(_) => {
                    unreachable!("Should not throw error");
                }
            },
            Ok(Some(t)) => {
                assert_eq!(t.value, "The");
                unreachable!("should not have matched 'The'");
            }
            Ok(None) => {
                unreachable!("Should not hit None");
            }
        }
    }
}

use crate::matcher::{Matcher, MatcherResult};
use crate::util::token::Token;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Target {
    pub matching: bool,
    pub target: Box<Vec<char>>,
}

#[derive(Clone, Debug)]
pub struct ExactMatcher {
    pub index: usize,
    pub precedence: u8,
    pub running: bool,
    pub found: Option<usize>,
    pub targets: Box<Vec<Target>>,
    pub token_type: u16,
}

impl Matcher for ExactMatcher {
    fn reset(&mut self, _ctx: &mut Box<HashMap<String, i32>>) {
        for mut t in self.targets.iter_mut() {
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
                self.generate_exact_token()
            }
            Some(c) => {
                self.running = false;
                let mut i: usize = 0;
                for target in self.targets.iter_mut() {
                    if target.matching {
                        match target.target.get(self.index) {
                            None => {
                                target.matching = false;
                                if self.index > 0 {
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
                    self.generate_exact_token()
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

impl ExactMatcher {
    pub fn build_exact_matcher(
        matches: Vec<&str>,
        token_type: u16,
        precedence: u8,
    ) -> ExactMatcher {
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
        ExactMatcher {
            index: 0,
            precedence,
            found: None,
            running: true,
            targets,
            token_type,
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
                    column: 0,
                    precedence: self.precedence,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::input::inputs::string_input::InputString;
    use crate::matcher::matchers::exact_matcher::ExactMatcher;
    use crate::matcher::matchers::whitespace_matcher::WhitespaceMatcher;
    use crate::util::token::TOKEN_TYPE_EXACT;
    use crate::Lexx;
    use crate::LexxResult::{EndOfInput, Failed, Matched};

    #[test]
    fn exact_matcher_matches_word() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("The"))),
            vec![Box::new(ExactMatcher::build_exact_matcher(
                vec!["The"],
                TOKEN_TYPE_EXACT,
                0,
            ))],
        );

        match lexx.next() {
            EndOfInput() => {
                assert!(false, "should not have finished EOF");
            }
            Failed() => {
                assert!(false, "should not have Failed");
            }
            Matched(t) => {
                assert_eq!(t.value, "The");
                assert_eq!(t.token_type, TOKEN_TYPE_EXACT)
            }
        }
    }

    #[test]
    fn exact_matcher_matches_multiple_words() {
        use crate::util::token::TOKEN_TYPE_WHITESPACE;
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
                    precedence: 0,
                    running: true,
                }),
            ],
        );

        assert!(matches!(lexx.next(), Matched(t) if t.value == "The"));
        assert!(matches!(lexx.next(), Matched(t) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexx.next(), Matched(t) if t.value == "quick"));
        assert!(matches!(lexx.next(), Matched(t) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexx.next(), Matched(t) if t.value == "brown"));
        assert!(matches!(lexx.next(), Matched(t) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexx.next(), Matched(t) if t.value == "fox"));
        assert!(matches!(lexx.next(), Matched(t) if t.token_type == TOKEN_TYPE_WHITESPACE));

        match lexx.next() {
            EndOfInput() => {
                assert!(false, "should not have finished EOF");
            }
            Failed() => {
                assert!(false, "should not have Failed");
            }
            Matched(t) => {
                assert_eq!(t.value, "qquick");
                assert_eq!(t.line, 1);
                assert_eq!(t.column, 21);
            }
        }
    }

    #[test]
    fn exact_matcher_matches_multiple_words_and_lines() {
        use crate::util::token::TOKEN_TYPE_WHITESPACE;
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
                    precedence: 0,
                    running: true,
                }),
            ],
        );

        assert!(matches!(lexx.next(), Matched(t) if t.value == "The"));
        assert!(matches!(lexx.next(), Matched(t) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexx.next(), Matched(t) if t.value == "quick"));
        assert!(matches!(lexx.next(), Matched(t) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexx.next(), Matched(t) if t.value == "brown"));
        assert!(matches!(lexx.next(), Matched(t) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexx.next(), Matched(t) if t.value == "fox"));
        assert!(matches!(lexx.next(), Matched(t) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexx.next(), Matched(t) if t.value == "jumped"));
        assert!(matches!(lexx.next(), Matched(t) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexx.next(), Matched(t) if t.value == "over"));
        assert!(matches!(lexx.next(), Matched(t) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexx.next(), Matched(t) if t.value == "the"));
        assert!(matches!(lexx.next(), Matched(t) if t.token_type == TOKEN_TYPE_WHITESPACE));
        assert!(matches!(lexx.next(), Matched(t) if t.value == "lazy"));
        assert!(matches!(lexx.next(), Matched(t) if t.token_type == TOKEN_TYPE_WHITESPACE));
        match lexx.next() {
            EndOfInput() => {
                assert!(false, "should not have finished EOF");
            }
            Failed() => {
                assert!(false, "should not have Failed");
            }
            Matched(t) => {
                assert_eq!(t.value, "dog");
                assert_eq!(t.line, 4);
                assert_eq!(t.column, 15);
            }
        }
    }

    #[test]
    fn exact_matcher_matches_partial_word() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("Then"))),
            vec![Box::new(ExactMatcher::build_exact_matcher(
                vec!["The"],
                TOKEN_TYPE_EXACT,
                0,
            ))],
        );

        match lexx.next() {
            EndOfInput() => {
                assert!(false, "should not have finished EOF");
            }
            Failed() => {
                assert!(false, "should not have Failed");
            }
            Matched(t) => {
                assert_eq!(t.value, "The");
                assert_eq!(t.token_type, TOKEN_TYPE_EXACT)
            }
        }
    }
}

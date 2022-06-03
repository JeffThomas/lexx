use crate::matcher::{Matcher, MatcherResult};
use crate::util::token::{Token, TOKEN_TYPE_WORD};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct WordMatcher {
    pub index: usize,
    pub precedence: u8,
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
                column: 0,
                precedence: self.precedence,
            })
        } else {
            MatcherResult::Failed()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::input::inputs::string_input::InputString;
    use crate::matcher::matchers::whitespace_matcher::WhitespaceMatcher;
    use crate::matcher::matchers::word_matcher::WordMatcher;
    use crate::util::token::TOKEN_TYPE_WORD;
    use crate::Lexx;
    use crate::LexxResult::{EndOfInput, Failed, Matched};

    #[test]
    fn word_matcher_matches_word() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("The"))),
            vec![Box::new(WordMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
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
                assert_eq!(t.token_type, TOKEN_TYPE_WORD)
            }
        }
    }

    #[test]
    fn word_matcher_matches_word_with_symbol() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("Stop!"))),
            vec![Box::new(WordMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        match lexx.next() {
            EndOfInput() => {
                assert!(false, "should not have finished EOF");
            }
            Failed() => {
                assert!(false, "should not have Failed");
            }
            Matched(t) => {
                assert_eq!(t.value, "Stop");
                assert_eq!(t.token_type, TOKEN_TYPE_WORD)
            }
        }
    }

    #[test]
    fn word_matcher_matches_word_with_number() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("Stop1"))),
            vec![Box::new(WordMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        match lexx.next() {
            EndOfInput() => {
                assert!(false, "should not have finished EOF");
            }
            Failed() => {
                assert!(false, "should not have Failed");
            }
            Matched(t) => {
                assert_eq!(t.value, "Stop");
                assert_eq!(t.token_type, TOKEN_TYPE_WORD)
            }
        }
    }

    #[test]
    fn word_matcher_matches_multiple_words() {
        use crate::util::token::TOKEN_TYPE_WHITESPACE;
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
    fn word_matcher_matches_multiple_words_and_lines() {
        use crate::util::token::TOKEN_TYPE_WHITESPACE;
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
    fn word_matcher_does_not_match_number() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("512"))),
            vec![Box::new(WordMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        match lexx.next() {
            EndOfInput() => {
                assert!(false, "should not have finished EOF");
            }
            Failed() => {
                // should fail;
            }
            Matched(_) => {
                assert!(false, "should not have matched 512");
            }
        }
    }

    #[test]
    fn word_matcher_does_not_match_space() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from(" "))),
            vec![Box::new(WordMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        match lexx.next() {
            EndOfInput() => {
                assert!(false, "should not have finished EOF");
            }
            Failed() => {
                // should fail;
            }
            Matched(_) => {
                assert!(false, "should not have matched space");
            }
        }
    }

    #[test]
    fn word_matcher_does_not_match_symbol() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("%"))),
            vec![Box::new(WordMatcher {
                index: 0,
                precedence: 0,
                running: true,
            })],
        );

        match lexx.next() {
            EndOfInput() => {
                assert!(false, "should not have finished EOF");
            }
            Failed() => {
                // should fail;
            }
            Matched(_) => {
                assert!(false, "should not have matched 5");
            }
        }
    }
}

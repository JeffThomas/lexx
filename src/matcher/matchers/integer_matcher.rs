use crate::matcher::{Matcher, MatcherResult};
use crate::util::token::{Token, TOKEN_TYPE_INTEGER};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct IntegerMatcher {
    pub index: usize,
    pub precedence: u8,
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
        return match oc {
            None => self.generate_integer_token(value),
            Some(c) => {
                if c.is_numeric() {
                    self.index += 1;
                    MatcherResult::Running()
                } else {
                    self.generate_integer_token(value)
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

impl IntegerMatcher {
    #[inline(always)]
    fn generate_integer_token(&mut self, value: &[char]) -> MatcherResult {
        self.running = false;
        if self.index > 0 {
            MatcherResult::Matched(Token {
                value: value[0..self.index].into_iter().collect(),
                token_type: TOKEN_TYPE_INTEGER,
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
    use crate::matcher::matchers::integer_matcher::IntegerMatcher;
    use crate::matcher::matchers::whitespace_matcher::WhitespaceMatcher;
    use crate::Lexx;
    use crate::LexxResult::{EndOfInput, Failed, Matched};

    #[test]
    fn integer_matcher_matches_integer() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("4"))),
            vec![Box::new(IntegerMatcher {
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
                assert_eq!(t.value, "4")
            }
        }
    }

    #[test]
    fn integer_matcher_matches_big_integer() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("6346357587454"))),
            vec![Box::new(IntegerMatcher {
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
                assert_eq!(t.value, "6346357587454")
            }
        }
    }

    #[test]
    fn integer_matcher_matches_integer_not_float() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("5.5"))),
            vec![Box::new(IntegerMatcher {
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
                assert_eq!(t.value, "5")
            }
        }
    }

    #[test]
    fn integer_matcher_matches_multiple() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("2 3 4"))),
            vec![
                Box::new(IntegerMatcher {
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

        match lexx.next() {
            EndOfInput() => {
                assert!(false, "should not have finished EOF");
            }
            Failed() => {
                assert!(false, "should not have Failed");
            }
            Matched(t) => {
                assert_eq!(t.value, "2")
            }
        }

        assert!(
            matches!(lexx.next(), Matched(t) if t.token_type == crate::util::token::TOKEN_TYPE_WHITESPACE)
        );

        match lexx.next() {
            EndOfInput() => {
                assert!(false, "should not have finished EOF");
            }
            Failed() => {
                assert!(false, "should not have Failed");
            }
            Matched(t) => {
                assert_eq!(t.value, "3")
            }
        }

        assert!(
            matches!(lexx.next(), Matched(t) if t.token_type == crate::util::token::TOKEN_TYPE_WHITESPACE)
        );

        match lexx.next() {
            EndOfInput() => {
                assert!(false, "should not have finished EOF");
            }
            Failed() => {
                assert!(false, "should not have Failed");
            }
            Matched(t) => {
                assert_eq!(t.value, "4")
            }
        }
    }
}

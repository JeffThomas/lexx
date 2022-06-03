use crate::matcher::{Matcher, MatcherResult};
use crate::util::token::{Token, TOKEN_TYPE_FLOAT};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct FloatMatcher {
    pub index: usize,
    pub precedence: u8,
    pub dot: bool,
    pub float: bool,
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
        return match oc {
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
        };
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
                value: value[0..self.index].into_iter().collect(),
                token_type: TOKEN_TYPE_FLOAT,
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

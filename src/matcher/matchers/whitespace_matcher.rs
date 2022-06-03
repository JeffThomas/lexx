use crate::matcher::{Matcher, MatcherResult};
pub use crate::util::token::{Token, TOKEN_TYPE_WHITESPACE};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct WhitespaceMatcher {
    pub index: usize,
    pub precedence: u8,
    pub running: bool,
}

impl Matcher for WhitespaceMatcher {
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
                self.generate_whitspace_token(value)
            }
            Some(c) => {
                if c.is_whitespace() {
                    self.index += 1;
                    MatcherResult::Running()
                } else {
                    self.running = false;
                    self.generate_whitspace_token(value)
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

impl WhitespaceMatcher {
    #[inline(always)]
    fn generate_whitspace_token(&mut self, value: &[char]) -> MatcherResult {
        if self.index > 0 {
            MatcherResult::Matched(Token {
                value: value[0..self.index].into_iter().collect(),
                token_type: TOKEN_TYPE_WHITESPACE,
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

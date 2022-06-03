use crate::matcher::{Matcher, MatcherResult};
use crate::util::token::{Token, TOKEN_TYPE_SYMBOL};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct SymbolMatcher {
    pub index: usize,
    pub precedence: u8,
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
        return match oc {
            None => self.generate_symbol_token(value),
            Some(c) => {
                if !c.is_whitespace() && !c.is_alphanumeric() {
                    self.index += 1;
                    MatcherResult::Running()
                } else {
                    self.generate_symbol_token(value)
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

impl SymbolMatcher {
    #[inline(always)]
    fn generate_symbol_token(&mut self, value: &[char]) -> MatcherResult {
        self.running = false;
        if self.index > 0 {
            MatcherResult::Matched(Token {
                value: value[0..self.index].into_iter().collect(),
                token_type: TOKEN_TYPE_SYMBOL,
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

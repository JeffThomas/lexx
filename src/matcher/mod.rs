pub mod matchers;

use crate::util::token::Token;
use std::collections::HashMap;

pub enum MatcherResult {
    Running(),
    Failed(),
    Matched(Token),
}

pub trait Matcher {
    fn reset(&mut self, ctx: &mut Box<HashMap<String, i32>>);
    fn find_match(
        &mut self,
        oc: Option<char>,
        value: &[char],
        ctx: &mut Box<HashMap<String, i32>>,
    ) -> MatcherResult;
    fn is_running(&self) -> bool;
    fn precedence(&self) -> u8;
}

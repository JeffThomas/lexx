use crate::matcher::{Matcher, MatcherResult};
use crate::token::{Token, TOKEN_TYPE_SYMBOL};
use std::collections::HashMap;

/// The SymbolMatcher matches any series of characters that do NOT match `is_whitespace()` or
/// `c.is_alphanumeric()`. That is, any character that is not a number, letter or whitespace
/// will be matched by this matcher.
///
/// # Example
///
/// ```rust
/// use lexx::{Lexx, Lexxer};
/// use lexx::token::{TOKEN_TYPE_EXACT, TOKEN_TYPE_SYMBOL};
/// use lexx::matcher_exact::ExactMatcher;
/// use lexx::matcher_symbol::SymbolMatcher;
/// use lexx::input::InputString;
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
#[derive(Clone, Debug, Copy)]
pub struct SymbolMatcher {
    /// Current size of the ongoing match.
    pub index: usize,
    /// This matchers precedence.
    pub precedence: u8,
    /// If the matcher is currently running.
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
        match oc {
            Some(c) if !c.is_whitespace() && !c.is_alphanumeric() => {
                self.index += 1;
                MatcherResult::Running()
            }
            _ => self.generate_symbol_token(value),
        }
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
                column: self.index,
                precedence: self.precedence,
            })
        } else {
            MatcherResult::Failed()
        }
    }
}

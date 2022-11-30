///
///
/// [Matcher]s are used by [crate::Lexx] to identify [Token]s in the [char] stream.
///
/// [Matcher]s are [reset](Matcher::reset) and then [find_match](Matcher::find_match) is called
/// repeatedly until either a match is found or the match fails.
///
/// # Example
///
/// ```rust
///
/// use std::collections::HashMap;
/// use lexx::matcher::Matcher;
/// use lexx::matcher::MatcherResult::Failed;
/// use lexx::matcher::MatcherResult::Running;
/// use lexx::matcher::MatcherResult::Matched;
/// use lexx::token::TOKEN_TYPE_WORD;
/// use lexx::matcher_word::WordMatcher;
///
/// let mut ctx: Box<HashMap<String, i32>> = Box::new(HashMap::new());
///
/// let mut matcher_word: WordMatcher = WordMatcher{ index: 0, precedence: 0, running: true };
///
/// assert!(matches!(matcher_word.find_match(Some('w'), &['w'], &mut ctx), Running()));
/// assert!(matches!(matcher_word.find_match(Some('o'), &['w','o'], &mut ctx), Running()));
/// assert!(matches!(matcher_word.find_match(Some('r'), &['w','o','r'], &mut ctx), Running()));
/// assert!(matches!(matcher_word.find_match(Some('d'), &['w','o','r','d'], &mut ctx), Running()));
/// // None signals the end of input
/// assert!(matches!(matcher_word.find_match(None, &['w','o','r','d'], &mut ctx), Matched(t) if t.value == "word"));
/// ```

use crate::token::Token;
use std::collections::HashMap;
use std::fmt::Debug;

/// The result of a match
#[derive(Debug)]
pub enum MatcherResult {
    /// The matcher is still running, it has not failed or found a match yet
    Running(),
    /// The matcher failed to find a match
    Failed(),
    /// A successful match
    Matched(Token),
}

///
///
/// [Matcher]s are used by [crate::Lexx] to identify [Token]s in the [char] stream.
///
/// [Matcher]s are [reset](Matcher::reset) and then [find_match](Matcher::find_match) is called
/// repeatedly until either a match is found or the match fails.
///
/// # Example
///
/// ```rust
///
/// use std::collections::HashMap;
/// use lexx::matcher::Matcher;
/// use lexx::matcher::MatcherResult::Failed;
/// use lexx::matcher::MatcherResult::Running;
/// use lexx::matcher::MatcherResult::Matched;
/// use lexx::token::TOKEN_TYPE_WORD;
/// use lexx::matcher_word::WordMatcher;
///
/// let mut ctx: Box<HashMap<String, i32>> = Box::new(HashMap::new());
///
/// let mut matcher_word: WordMatcher = WordMatcher{ index: 0, precedence: 0, running: true };
///
/// assert!(matches!(matcher_word.find_match(Some('w'), &['w'], &mut ctx), Running()));
/// assert!(matches!(matcher_word.find_match(Some('o'), &['w','o'], &mut ctx), Running()));
/// assert!(matches!(matcher_word.find_match(Some('r'), &['w','o','r'], &mut ctx), Running()));
/// assert!(matches!(matcher_word.find_match(Some('d'), &['w','o','r','d'], &mut ctx), Running()));
/// // None signals the end of input
/// assert!(matches!(matcher_word.find_match(None, &['w','o','r','d'], &mut ctx), Matched(t) if t.value == "word"));
/// ```
pub trait Matcher: Debug {
    /// Puts the matcher in a starting state to beging accepting [char]s
    fn reset(&mut self, ctx: &mut Box<HashMap<String, i32>>);
    /// The function that does all the work, it is called repeatedly
    /// with new [Option<char>] values until it returns a [MatcherResult::Matched] or
    /// [MatcherResult::Failed].
    /// * `oc` - The current [Option<char>] to match against. An [Option::None] input indicates
    /// the end of input, giving the matcher a chance to match what it's already seen or not. For
    /// example `1234[EOF]` is a valid integer for the [IntegerMatcher](crate::matcher_integer::IntegerMatcher)
    /// * `value` - Is an array of [char] characters that have already been sent to the [Matcher],
    /// so that the [Matcher]s don't have to keep their own history. [Matcher]s can use this to make
    /// the [String] value for their [Token](crate::token::Token) if they find a match.
    fn find_match(
        &mut self,
        oc: Option<char>,
        value: &[char],
        ctx: &mut Box<HashMap<String, i32>>,
    ) -> MatcherResult;
    /// If the matcher is still accepting [char]s or not, it hasn't yet found a match or failed
    fn is_running(&self) -> bool;
    /// Used for resolving same length matches, higher numbers have higher precedence
    fn precedence(&self) -> u8;
}

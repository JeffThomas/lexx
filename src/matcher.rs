///
/// Trait for token matchers used by [`Lexx`](crate::Lexx).
///
/// # Overview
///
/// A `Matcher` is responsible for recognizing a particular kind of token in a character stream. Each matcher
/// maintains its own state and is reset before each new tokenization attempt. Lexx will call `find_match` repeatedly,
/// feeding one character at a time to each matcher, until one returns a successful match or all fail.
///
/// Matchers can be used for words, numbers, symbols, keywords, whitespace, or any custom pattern. They can share
/// context using the provided `ctx` parameter, which is a mutable boxed `HashMap`.
///
/// # Example: Implementing and Using a Matcher
///
/// ```rust
/// use std::collections::HashMap;
/// use lexx::matcher::{Matcher, MatcherResult};
/// use lexx::token::{Token, TOKEN_TYPE_WORD};
/// 
/// #[derive(Debug)]
/// struct SimpleWordMatcher { index: usize, running: bool }
/// 
/// impl Matcher for SimpleWordMatcher {
///     fn reset(&mut self, _ctx: &mut Box<HashMap<String, i32>>) {
///         self.index = 0;
///         self.running = true;
///     }
///     fn find_match(&mut self, oc: Option<char>, value: &[char], _ctx: &mut Box<HashMap<String, i32>>)
///         -> MatcherResult {
///         match oc {
///             Some(c) if c.is_alphabetic() => {
///                 self.index += 1;
///                 MatcherResult::Running()
///             }
///             _ if self.index > 0 => {
///                 let s: String = value.iter().collect();
///                 MatcherResult::Matched(Token {
///                     token_type: TOKEN_TYPE_WORD,
///                     value: s,
///                     line: 1,
///                     column: 1,
///                     len: self.index,
///                     precedence: 0,
///                 })
///             }
///             _ => MatcherResult::Failed(),
///         }
///     }
///     fn is_running(&self) -> bool { self.running }
///     fn precedence(&self) -> u8 { 0 }
/// }
/// 
/// let mut ctx: Box<HashMap<String, i32>> = Box::new(HashMap::new());
/// let mut matcher = SimpleWordMatcher { index: 0, running: true };
/// assert!(matches!(matcher.find_match(Some('w'), &['w'], &mut ctx), MatcherResult::Running()));
/// assert!(matches!(matcher.find_match(Some('o'), &['w','o'], &mut ctx), MatcherResult::Running()));
/// assert!(matches!(matcher.find_match(Some('r'), &['w','o','r'], &mut ctx), MatcherResult::Running()));
/// assert!(matches!(matcher.find_match(Some('d'), &['w','o','r','d'], &mut ctx), MatcherResult::Running()));
/// // None signals the end of input
/// assert!(matches!(matcher.find_match(None, &['w','o','r','d'], &mut ctx), MatcherResult::Matched(_)));
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

/// All matcher types must also implement [`Debug`](Debug),
/// which allows for easy inspection and debugging of matcher state.
/// This is especially useful when writing tests or diagnosing matcher behavior
/// during tokenization.
pub trait Matcher: Debug {
    /// Puts the matcher in a starting state to beging accepting [char]s
    fn reset(&mut self, ctx: &mut Box<HashMap<String, i32>>);
    /// The function that does all the work, it is called repeatedly
    /// with new [Option<char>] values until it returns a [MatcherResult::Matched] or
    /// [MatcherResult::Failed].
    /// * `oc` - The current [Option<char>] to match against. An [None] input indicates
    /// the end of input, giving the matcher a chance to match what it's already seen or not. For
    /// example `1234[EOF]` is a valid integer for the [IntegerMatcher](crate::matcher_integer::IntegerMatcher)
    /// * `value` - Is an array of [char] characters that have already been sent to the [Matcher],
    /// so that the [Matcher]s don't have to keep their own history. [Matcher]s can use this to make
    /// the [String] value for their [Token](Token) if they find a match.
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

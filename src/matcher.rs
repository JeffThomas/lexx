//! The matcher module provides a set of token matchers for the Lexx lexer.
//!
//! Each matcher implements the [Matcher] trait and is responsible for recognizing
//! specific types of tokens in a character stream. The available matchers are:
//!
//! - [ExactMatcher](matcher::exact::ExactMatcher): Matches exact strings from a provided list.
//! - [FloatMatcher](matcher::float::FloatMatcher): Matches floating-point numbers (e.g., 3.14, 0.001).
//! - [IntegerMatcher](matcher::integer::IntegerMatcher): Matches integer numbers (e.g., 42, -7).
//! - [KeywordMatcher](matcher::keyword::KeywordMatcher): Matches specific keywords, ensuring they are not substrings.
//! - [SymbolMatcher](matcher::symbol::SymbolMatcher): Matches non-alphanumeric, non-whitespace symbols (e.g., @, #, $).
//! - [WhitespaceMatcher](matcher::whitespace::WhitespaceMatcher): Matches whitespace characters (spaces, tabs, newlines).
//! - [WordMatcher](matcher::word::WordMatcher): Matches sequences of alphabetic characters (words).
//!
//! Each matcher can be used independently or in combination to build a custom lexer.

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

/// All matcher types must also implement [`Debug`],
/// which allows for easy inspection and debugging of matcher state.
/// This is especially useful when writing tests or diagnosing matcher behavior
/// during tokenization.
pub trait Matcher: Debug {
    /// Puts the matcher in a starting state to beging accepting [char]s
    fn reset(&mut self, ctx: &mut Box<HashMap<String, i32>>);
    /// The function that does all the work, it is called repeatedly
    ///   with new [`Option<char>`] values until it returns a [MatcherResult::Matched] or
    /// [MatcherResult::Failed].
    /// * `oc` - The current [`Option<char>`] to match against. An [None] input indicates
    ///   the end of input, giving the matcher a chance to match what it's already seen or not. For
    ///   example `1234[EOF]` is a valid integer for the [IntegerMatcher](integer::IntegerMatcher)
    /// * `value` - Is an array of [char] characters that have already been sent to the [Matcher],
    ///   so that the [Matcher]s don't have to keep their own history. [Matcher]s can use this to make
    ///   the [String] value for their [Token] if they find a match.
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

/// The `exact` module provides the `ExactMatcher`, which matches strings exactly as specified.
/// It allows users to define a list of strings to match against, ensuring that only exact matches
/// are recognized, regardless of their position in the input stream.
pub mod exact;
/// The float matcher matches floating point numbers. To qualify as floating point, the numbers must
/// start and end with a numeric digit and have a period within them. For example `1.0`. Thus,
/// `.1` and `1.` do not qualify as floating point numbers.
pub mod float;
/// The integer matcher matches integer numbers. To qualify as integer the numbers must
/// start and end with a numeric digit.
pub mod integer;
/// The Keyword matcher is very similar to the [ExactMatcher](exact::ExactMatcher), in that you give it a list of matches
/// to make, and it looks EXACTLY for those matches. The difference between this matcher and the
/// [ExactMatcher](exact::ExactMatcher) is that for `THIS` matcher an exact match must end with a non alpha-numeric
/// character. For example, if you give this matcher "match" as a keyword it will NOT match
/// "matches", "matchers" or "match1", "1matcher", "2match" etc.
/// It will match "match ", " match." "---match---" and so on.
pub mod keyword;
/// The SymbolMatcher matches any series of characters that do NOT match `is_whitespace()` or
/// `c.is_alphanumeric()`. That is, this matcher
/// will match any character that is not a number, letter, or whitespace.
pub mod symbol;
/// The WhitespaceMatcher matches any series of characters that are `is_whitespace()`.
pub mod whitespace;
/// The `WordMatcher` is a matcher that matches word tokens in the input stream.
pub mod word;

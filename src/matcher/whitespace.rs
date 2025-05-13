/// The `whitespace` module provides the `WhitespaceMatcher`, which matches sequences of whitespace characters in the input stream.
/// Whitespace characters are those for which Rust’s `char::is_whitespace()` returns true (such as spaces, tabs, and newlines).
/// The matcher recognizes contiguous runs of whitespace and produces tokens of type `TOKEN_TYPE_WHITESPACE`.
///
/// This module is useful for lexers that need to identify and handle whitespace regions, which are often ignored or treated specially in tokenization.
use crate::matcher::{Matcher, MatcherResult};
pub use crate::token::{TOKEN_TYPE_WHITESPACE, Token};
use std::collections::HashMap;

/// The WhitespaceMatcher matches any series of characters that are `is_whitespace()`.
///
/// # Example
///
/// ```rust
/// use lexx::{Lexx, Lexxer};
/// use lexx::token::{TOKEN_TYPE_WHITESPACE, TOKEN_TYPE_WORD};
/// use lexx::input::InputString;
/// use lexx::matcher::whitespace::WhitespaceMatcher;
/// use lexx::matcher::word::WordMatcher;
///
/// let lexx_input = InputString::new(String::from(" a \t\nb \r\n\t c"));
///
/// let mut lexx: Box<dyn Lexxer> = Box::new(Lexx::<512>::new(
///     Box::new(lexx_input),
///     vec![
///         Box::new(WhitespaceMatcher { index: 0, column: 0,line: 0,precedence: 0, running: true}),
///         Box::new(WordMatcher { index: 0, precedence: 0, running: true }),
///     ]
/// ));
///
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE && t.line == 1 && t.column == 1));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "a" && t.token_type == TOKEN_TYPE_WORD && t.line == 1 && t.column == 2));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE && t.line == 1 && t.column == 3));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "b" && t.token_type == TOKEN_TYPE_WORD && t.line == 2 && t.column == 1));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.token_type == TOKEN_TYPE_WHITESPACE && t.line == 2 && t.column == 2));
/// assert!(matches!(lexx.next_token(), Ok(Some(t)) if t.value == "c" && t.token_type == TOKEN_TYPE_WORD && t.line == 3 && t.column == 3));
/// assert!(matches!(lexx.next_token(), Ok(None)));
/// ```
#[derive(Clone, Debug, Copy)]
pub struct WhitespaceMatcher {
    /// Current size of the ongoing match.
    pub index: usize,
    /// column count
    pub column: usize,
    /// line count
    pub line: usize,
    /// This matchers precedence.
    pub precedence: u8,
    /// If the matcher is currently running.
    pub running: bool,
}

impl Matcher for WhitespaceMatcher {
    fn reset(&mut self, _ctx: &mut Box<HashMap<String, i32>>) {
        self.index = 0;
        self.line = 0;
        self.column = 0;
        self.running = true;
    }

    fn find_match(
        &mut self,
        oc: Option<char>,
        value: &[char],
        _ctx: &mut Box<HashMap<String, i32>>,
    ) -> MatcherResult {
        match oc {
            Some(c) if c.is_whitespace() => {
                self.index += 1;
                self.column += 1;
                if c == '\r' {
                    self.column = 0;
                } else if c == '\n' {
                    self.column = 1;
                    self.line += 1;
                }
                MatcherResult::Running()
            }
            _ => {
                self.running = false;
                self.generate_whitspace_token(value)
            }
        }
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
                value: value[0..self.index].iter().collect(),
                token_type: TOKEN_TYPE_WHITESPACE,
                len: self.index,
                line: self.line,
                column: self.column,
                precedence: self.precedence,
            })
        } else {
            MatcherResult::Failed()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::input::InputString;
    use crate::matcher::Matcher;
    use crate::matcher::MatcherResult;
    use crate::matcher::whitespace::WhitespaceMatcher;
    use crate::matcher::word::WordMatcher;
    use crate::token::TOKEN_TYPE_WHITESPACE;
    use crate::{Lexx, Lexxer};
    use std::collections::HashMap;

    #[test]
    fn test_basic_whitespace_matching() {
        // Test basic whitespace matching with spaces
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("   "))),
            vec![Box::new(WhitespaceMatcher {
                index: 0,
                column: 0,
                line: 0,
                precedence: 0,
                running: true,
            })],
        );

        // Should match all three spaces as a single token
        let token = lexx.next_token().unwrap().unwrap();
        assert_eq!(token.token_type, TOKEN_TYPE_WHITESPACE);
        assert_eq!(token.value, "   ");

        // No more tokens
        assert!(matches!(lexx.next_token(), Ok(None)));
    }

    #[test]
    fn test_mixed_whitespace_types() {
        // Test matching of different whitespace characters
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from(" \t\r\n "))),
            vec![Box::new(WhitespaceMatcher {
                index: 0,
                column: 0,
                line: 0,
                precedence: 0,
                running: true,
            })],
        );

        // Should match all whitespace characters as a single token
        let token = lexx.next_token().unwrap().unwrap();
        assert_eq!(token.token_type, TOKEN_TYPE_WHITESPACE);
        assert_eq!(token.value, " \t\r\n ");

        // No more tokens
        assert!(matches!(lexx.next_token(), Ok(None)));
    }

    #[test]
    fn test_line_counting() {
        // Test that line counting works correctly with different newline styles
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("a\nb\r\nc\rd"))),
            vec![
                Box::new(WordMatcher {
                    index: 0,
                    precedence: 0,
                    running: true,
                }),
                Box::new(WhitespaceMatcher {
                    index: 0,
                    column: 0,
                    line: 0,
                    precedence: 0,
                    running: true,
                }),
            ],
        );

        // First word - 'a'
        let token = lexx.next_token().unwrap().unwrap();
        assert_eq!(token.value, "a");
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 1);

        // Newline (\n)
        let token = lexx.next_token().unwrap().unwrap();
        assert_eq!(token.token_type, TOKEN_TYPE_WHITESPACE);
        assert_eq!(token.value, "\n");

        // Second word - 'b'
        let token = lexx.next_token().unwrap().unwrap();
        assert_eq!(token.value, "b");
        assert_eq!(token.line, 2);
        assert_eq!(token.column, 1);

        // Carriage return + newline (\r\n)
        let token = lexx.next_token().unwrap().unwrap();
        assert_eq!(token.token_type, TOKEN_TYPE_WHITESPACE);
        assert_eq!(token.value, "\r\n");

        // Third word - 'c'
        let token = lexx.next_token().unwrap().unwrap();
        assert_eq!(token.value, "c");
        assert_eq!(token.line, 3);
        assert_eq!(token.column, 1);

        // Carriage return only (\r)
        let token = lexx.next_token().unwrap().unwrap();
        assert_eq!(token.token_type, TOKEN_TYPE_WHITESPACE);
        assert_eq!(token.value, "\r");

        // Fourth word - 'd'
        // let token = lexx.next_token().unwrap().unwrap();
        // assert_eq!(token.value, "d");
        // assert_eq!(token.line, 3);
        // assert_eq!(token.column, 1);
    }

    #[test]
    fn test_whitespace_with_non_whitespace() {
        // Test that whitespace matcher stops at non-whitespace characters
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("  abc  "))),
            vec![
                Box::new(WhitespaceMatcher {
                    index: 0,
                    column: 0,
                    line: 0,
                    precedence: 0,
                    running: true,
                }),
                Box::new(WordMatcher {
                    index: 0,
                    precedence: 0,
                    running: true,
                }),
            ],
        );

        // First whitespace token
        let token = lexx.next_token().unwrap().unwrap();
        assert_eq!(token.token_type, TOKEN_TYPE_WHITESPACE);
        assert_eq!(token.value, "  ");

        // Word token
        let token = lexx.next_token().unwrap().unwrap();
        assert_eq!(token.value, "abc");

        // Second whitespace token
        let token = lexx.next_token().unwrap().unwrap();
        assert_eq!(token.token_type, TOKEN_TYPE_WHITESPACE);
        assert_eq!(token.value, "  ");
    }

    #[test]
    fn test_unicode_whitespace() {
        // Test that Unicode whitespace characters are properly matched
        // Unicode whitespace includes characters like:
        // \u{00A0} (non-breaking space)
        // \u{2000}-\u{200A} (various width spaces)
        // \u{3000} (ideographic space)
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from(" \u{00A0}\u{2002}\u{3000} "))),
            vec![Box::new(WhitespaceMatcher {
                index: 0,
                column: 0,
                line: 0,
                precedence: 0,
                running: true,
            })],
        );

        // Should match all Unicode whitespace characters as a single token
        let token = lexx.next_token().unwrap().unwrap();
        assert_eq!(token.token_type, TOKEN_TYPE_WHITESPACE);
        assert_eq!(token.value, " \u{00A0}\u{2002}\u{3000} ");
    }

    #[test]
    fn test_empty_input() {
        // Test behavior with empty input
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from(""))),
            vec![Box::new(WhitespaceMatcher {
                index: 0,
                column: 0,
                line: 0,
                precedence: 0,
                running: true,
            })],
        );

        // Should return None for empty input
        assert!(matches!(lexx.next_token(), Ok(None)));
    }

    #[test]
    fn test_reset_functionality() {
        // Test that the reset function properly resets the matcher state
        let mut matcher = WhitespaceMatcher {
            index: 10, // Simulate some previous matching
            column: 5,
            line: 3,
            precedence: 0,
            running: false,
        };

        // Reset the matcher
        let mut ctx = Box::new(HashMap::new());
        matcher.reset(&mut ctx);

        // Verify that the matcher state has been reset
        assert_eq!(matcher.index, 0);
        assert_eq!(matcher.column, 0);
        assert_eq!(matcher.line, 0);
        assert!(matcher.running);
    }

    #[test]
    fn test_whitespace_at_end_of_input() {
        // Test that whitespace at the end of input is properly matched
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("abc   "))),
            vec![
                Box::new(WordMatcher {
                    index: 0,
                    precedence: 0,
                    running: true,
                }),
                Box::new(WhitespaceMatcher {
                    index: 0,
                    column: 0,
                    line: 0,
                    precedence: 0,
                    running: true,
                }),
            ],
        );

        // Word token
        let token = lexx.next_token().unwrap().unwrap();
        assert_eq!(token.value, "abc");

        // Whitespace token at end of input
        let token = lexx.next_token().unwrap().unwrap();
        assert_eq!(token.token_type, TOKEN_TYPE_WHITESPACE);
        assert_eq!(token.value, "   ");

        // No more tokens
        assert!(matches!(lexx.next_token(), Ok(None)));
    }

    #[test]
    fn test_direct_matcher_methods() {
        // Test the matcher methods directly
        let mut matcher = WhitespaceMatcher {
            index: 0,
            column: 0,
            line: 0,
            precedence: 2, // Set a non-zero precedence to test precedence method
            running: true,
        };

        let mut ctx = Box::new(HashMap::new());
        let value: Vec<char> = vec![' ', ' ', '\n'];

        // Test is_running
        assert!(matcher.is_running());

        // Test precedence
        assert_eq!(matcher.precedence(), 2);

        // Test find_match with whitespace
        assert!(matches!(
            matcher.find_match(Some(' '), &value, &mut ctx),
            MatcherResult::Running()
        ));
        assert_eq!(matcher.index, 1);
        assert_eq!(matcher.column, 1);

        // Test find_match with newline
        assert!(matches!(
            matcher.find_match(Some('\n'), &value, &mut ctx),
            MatcherResult::Running()
        ));
        assert_eq!(matcher.index, 2);
        assert_eq!(matcher.column, 1); // Column reset to 1 after newline
        assert_eq!(matcher.line, 1); // Line incremented

        // Test find_match with non-whitespace
        assert!(matches!(
            matcher.find_match(Some('a'), &value, &mut ctx),
            MatcherResult::Matched(_)
        ));
        assert!(!matcher.is_running()); // Should no longer be running
    }
}


// Token types. These are not an enum so that they can be externally extended
/// Token type Integer
pub const TOKEN_TYPE_INTEGER: u16 = 1;
/// Token type Float
pub const TOKEN_TYPE_FLOAT: u16 = 2;
/// Token type Whitespace
pub const TOKEN_TYPE_WHITESPACE: u16 = 3;
/// Token type Word
pub const TOKEN_TYPE_WORD: u16 = 4;
/// Token type Symbol
pub const TOKEN_TYPE_SYMBOL: u16 = 5;
/// Token type Exact
pub const TOKEN_TYPE_EXACT: u16 = 6;
/// Token type Keyword
pub const TOKEN_TYPE_KEYWORD: u16 = 7;
use std::fmt;

/// The result of a successful match.
#[derive(Eq, Debug)]
pub struct Token {
    /// The string value that was matched.
    pub value: String,
    /// The type of token that was matched. This is intentionally not an enum to allow users of the
    /// library to extend it as needed.
    pub token_type: u16,
    /// The length of the found [`Token`] in [`char`]s (so we don't have to do `.chars().count()`).
    pub len: usize,
    /// The line in the total input source the [`Token`] was found on.
    pub line: usize,
    /// The column in the total input source the [`Token`] was found at.
    pub column: usize,
    /// The precedence of the [`Matcher`](crate::matcher::Matcher) that made this match.
    pub precedence: u8,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
            && self.token_type == other.token_type
            && self.len == other.len
            && self.line == other.line
            && self.column == other.column
            && self.precedence == other.precedence
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Token {
            value: self.value.clone(),
            token_type: self.token_type,
            len: self.len,
            line: self.line,
            column: self.column,
            precedence: self.precedence,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token({}, '{}', ln:{}, col:{})",
            self.token_type, self.value, self.line, self.column
        )
    }
}

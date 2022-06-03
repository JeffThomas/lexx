pub const TOKEN_TYPE_INTEGER: u16 = 1;
pub const TOKEN_TYPE_FLOAT: u16 = 2;
pub const TOKEN_TYPE_WHITESPACE: u16 = 3;
pub const TOKEN_TYPE_WORD: u16 = 4;
pub const TOKEN_TYPE_SYMBOL: u16 = 5;
pub const TOKEN_TYPE_EXACT: u16 = 6;
pub const TOKEN_TYPE_KEYWORD: u16 = 7;

#[derive(Eq, Debug)]
pub struct Token {
    pub value: String,
    pub token_type: u16,
    pub len: usize,
    pub line: usize,
    pub column: usize,
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

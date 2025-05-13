use lexx::input::InputString;
use lexx::matcher::{Matcher, MatcherResult};
use lexx::token::{TOKEN_TYPE_SYMBOL, TOKEN_TYPE_WHITESPACE, TOKEN_TYPE_WORD, Token};
use lexx::{Lexx, Lexxer};
use std::collections::HashMap;
use std::fmt::Debug;

// Define a custom token type for hex color codes
const TOKEN_TYPE_HEX_COLOR: u16 = 200;

/// A custom matcher that identifies hexadecimal color codes in the format #RRGGBB or #RGB
#[derive(Debug)]
struct HexColorMatcher {
    index: usize,
    precedence: u8,
    running: bool,
}

impl HexColorMatcher {
    /// Creates a new HexColorMatcher with the given precedence
    pub fn new(precedence: u8) -> Self {
        HexColorMatcher {
            index: 0,
            precedence,
            running: true,
        }
    }

    /// Helper function to check if a character is a valid hex digit
    fn is_hex_digit(c: char) -> bool {
        c.is_ascii_hexdigit()
    }

    /// Generate a token when a hex color has been matched
    fn generate_hex_color_token(&self, value: &[char]) -> MatcherResult {
        // We need at least 4 characters for a valid hex color (#RGB)
        if self.index < 4 {
            return MatcherResult::Failed();
        }

        // Check if the first character is '#' and all subsequent characters are hex digits
        if value[0] != '#' {
            return MatcherResult::Failed();
        }

        for c in value.iter().take(self.index).skip(1) {
            if !Self::is_hex_digit(*c) {
                return MatcherResult::Failed();
            }
        }

        // Valid hex colors are either #RGB (4 chars) or #RRGGBB (7 chars)
        if self.index != 4 && self.index != 7 {
            return MatcherResult::Failed();
        }

        // Create the token
        let token_value: String = value[0..self.index].iter().collect();

        MatcherResult::Matched(Token {
            value: token_value,
            token_type: TOKEN_TYPE_HEX_COLOR,
            len: self.index,
            line: 0,   // This will be set by the Lexx tokenizer
            column: 0, // This will be set by the Lexx tokenizer
            precedence: self.precedence,
        })
    }
}

impl Matcher for HexColorMatcher {
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
            None => {
                self.running = false;
                self.generate_hex_color_token(value)
            }
            Some(c) => {
                // First character must be a hash
                if self.index == 0 {
                    if c == '#' {
                        self.index += 1;
                        self.running = true;
                        return MatcherResult::Running();
                    } else {
                        self.running = false;
                        return MatcherResult::Failed();
                    }
                }

                // After the hash, we need hex digits
                if Self::is_hex_digit(c) {
                    self.index += 1;

                    // If we've reached a valid length (4 or 7), we have a potential match
                    // but we'll keep accepting characters if they're hex digits
                    if self.index <= 7 {
                        self.running = true;
                        MatcherResult::Running()
                    } else {
                        // We've gone beyond the maximum length for a hex color
                        self.running = false;
                        MatcherResult::Failed()
                    }
                } else {
                    // We've encountered a non-hex digit
                    self.running = false;

                    // Check if we have a valid color code (#RGB or #RRGGBB)
                    if self.index == 4 || self.index == 7 {
                        self.generate_hex_color_token(value)
                    } else {
                        // Not a valid color code
                        MatcherResult::Failed()
                    }
                }
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

fn main() {
    // Sample text with hex color codes
    let text = "The background color is #FF5500 and the text color is #123. Invalid codes: #GGHHII, #12, #1234567.";
    let input = InputString::new(text.to_string());

    // Create a Lexx tokenizer with our custom HexColorMatcher
    let mut lexx = Lexx::<512>::new(
        Box::new(input),
        vec![
            Box::new(lexx::matcher::whitespace::WhitespaceMatcher {
                index: 0,
                column: 0,
                line: 0,
                precedence: 0,
                running: true,
            }),
            // Our custom matcher with high precedence to ensure it gets matched before symbols
            Box::new(HexColorMatcher::new(3)),
            Box::new(lexx::matcher::word::WordMatcher {
                index: 0,
                precedence: 0,
                running: true,
            }),
            Box::new(lexx::matcher::integer::IntegerMatcher {
                index: 0,
                precedence: 0,
                running: true,
            }),
            Box::new(lexx::matcher::symbol::SymbolMatcher {
                index: 0,
                precedence: 0,
                running: true,
            }),
        ],
    );

    println!("Tokenizing: \"{}\"", text);
    println!("{}", "-".repeat(70));

    loop {
        match lexx.next_token() {
            Ok(Some(token)) => {
                let type_name = match token.token_type {
                    TOKEN_TYPE_HEX_COLOR => "HEX_COLOR",
                    TOKEN_TYPE_WORD => "WORD",
                    TOKEN_TYPE_WHITESPACE => "WHITESPACE",
                    TOKEN_TYPE_SYMBOL => "SYMBOL",
                    lexx::token::TOKEN_TYPE_INTEGER => "INTEGER",
                    _ => "OTHER",
                };

                println!(
                    "{:<15} {:<30} (line: {}, column: {})",
                    type_name, token.value, token.line, token.column
                );
            }
            Ok(None) => {
                println!("\nEnd of input reached.");
                break;
            }
            Err(e) => {
                println!("Error during tokenization: {}", e);
                break;
            }
        }
    }
}

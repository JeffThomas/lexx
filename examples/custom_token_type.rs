use lexxor::input::InputString;
use lexxor::matcher::exact::ExactMatcher;
use lexxor::matcher::symbol::SymbolMatcher;
use lexxor::matcher::whitespace::WhitespaceMatcher;
use lexxor::matcher::word::WordMatcher;
use lexxor::token::{
    TOKEN_TYPE_EXACT, TOKEN_TYPE_FLOAT, TOKEN_TYPE_INTEGER, TOKEN_TYPE_SYMBOL,
    TOKEN_TYPE_WHITESPACE, TOKEN_TYPE_WORD,
};
use lexxor::{Lexxer, Lexxor};

// Define a special token type for our example
const TOKEN_TYPE_EMAIL_DOMAIN: u16 = 100;

fn main() {
    // Sample text with email-like content
    let text = "Contact us at support@example.com or sales@company.co.uk for more information.";
    let input = InputString::new(text.to_string());

    // Create a Lexxor tokenizer using existing matchers in a customized configuration
    // We'll use ExactMatcher to match common email domains with high precedence
    let mut lexxor = Lexxor::<512>::new(
        Box::new(input),
        vec![
            Box::new(WhitespaceMatcher {
                index: 0,
                column: 0,
                line: 0,
                precedence: 0,
                running: true,
            }),
            // Use ExactMatcher with high precedence to identify specific domains
            Box::new(ExactMatcher::build_exact_matcher(
                vec!["example.com", "company.co.uk"],
                TOKEN_TYPE_EMAIL_DOMAIN,
                3, // High precedence to ensure it gets matched
            )),
            Box::new(WordMatcher {
                index: 0,
                precedence: 0,
                running: true,
            }),
            Box::new(SymbolMatcher {
                index: 0,
                precedence: 0,
                running: true,
            }),
        ],
    );

    println!("Tokenizing: \"{}\"", text);
    println!("{}", "-".repeat(70));

    loop {
        match lexxor.next_token() {
            Ok(Some(token)) => {
                let type_name = if token.token_type == TOKEN_TYPE_EMAIL_DOMAIN {
                    "EMAIL_DOMAIN"
                } else {
                    match token.token_type {
                        TOKEN_TYPE_WORD => "WORD",
                        TOKEN_TYPE_WHITESPACE => "WHITESPACE",
                        TOKEN_TYPE_SYMBOL => "SYMBOL",
                        TOKEN_TYPE_INTEGER => "INTEGER",
                        TOKEN_TYPE_FLOAT => "FLOAT",
                        TOKEN_TYPE_EXACT => "EXACT",
                        _ => "OTHER",
                    }
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

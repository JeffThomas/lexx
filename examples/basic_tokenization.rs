use lexxor::input::InputString;
use lexxor::matcher::float::FloatMatcher;
use lexxor::matcher::integer::IntegerMatcher;
use lexxor::matcher::symbol::SymbolMatcher;
use lexxor::matcher::whitespace::WhitespaceMatcher;
use lexxor::matcher::word::WordMatcher;
use lexxor::token::{
    TOKEN_TYPE_FLOAT, TOKEN_TYPE_INTEGER, TOKEN_TYPE_SYMBOL, TOKEN_TYPE_WHITESPACE, TOKEN_TYPE_WORD,
};
use lexxor::{Lexxor, Lexxer};

fn main() {
    // Create a simple input string
    let input_text = "Hello world! This is 42 and 3.14159.";
    let input = InputString::new(input_text.to_string());

    // Create a Lexxor tokenizer with standard matchers
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
            Box::new(WordMatcher {
                index: 0,
                precedence: 0,
                running: true,
            }),
            Box::new(IntegerMatcher {
                index: 0,
                precedence: 0,
                running: true,
            }),
            Box::new(FloatMatcher {
                index: 0,
                precedence: 0,
                dot: false,
                float: false,
                running: true,
            }),
            Box::new(SymbolMatcher {
                index: 0,
                precedence: 0,
                running: true,
            }),
        ],
    );

    // Process and display all tokens
    println!("Tokenizing: \"{}\"", input_text);
    println!(
        "{:<15} {:<15} {:<10} {:<10}",
        "TOKEN TYPE", "VALUE", "LINE", "COLUMN"
    );
    println!("{}", "-".repeat(50));

    loop {
        match lexxor.next_token() {
            Ok(Some(token)) => {
                let type_name = match token.token_type {
                    TOKEN_TYPE_WORD => "WORD",
                    TOKEN_TYPE_WHITESPACE => "WHITESPACE",
                    TOKEN_TYPE_SYMBOL => "SYMBOL",
                    TOKEN_TYPE_INTEGER => "INTEGER",
                    TOKEN_TYPE_FLOAT => "FLOAT",
                    _ => "OTHER",
                };

                println!(
                    "{:<15} {:<15} {:<10} {:<10}",
                    type_name,
                    token.value.replace("\n", "\\n"),
                    token.line,
                    token.column
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

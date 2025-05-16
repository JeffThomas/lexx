use lexxor::input::InputString;
use lexxor::matcher::exact::ExactMatcher;
use lexxor::matcher::float::FloatMatcher;
use lexxor::matcher::integer::IntegerMatcher;
use lexxor::matcher::symbol::SymbolMatcher;
use lexxor::matcher::whitespace::WhitespaceMatcher;
use lexxor::matcher::word::WordMatcher;
use lexxor::token::{
    TOKEN_TYPE_FLOAT, TOKEN_TYPE_INTEGER, TOKEN_TYPE_KEYWORD, TOKEN_TYPE_SYMBOL,
    TOKEN_TYPE_WHITESPACE, TOKEN_TYPE_WORD,
};
use lexxor::{Lexxer, Lexxor};

fn main() {
    // Create a string that resembles programming code
    let code = r#"
fn calculate(x: f32, y: f32) -> f32 {
    let result = x * 3.14 + y;
    if result > 10.0 {
        return result / 2.0;
    } else {
        return result;
    }
}
"#;

    let input = InputString::new(code.to_string());

    // Create a Lexxor tokenizer specifically configured for code
    // Higher precedence for floats and keywords
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
            // Use ExactMatcher to recognize keywords with high precedence
            Box::new(ExactMatcher::build_exact_matcher(
                vec!["fn", "let", "if", "else", "return"],
                TOKEN_TYPE_KEYWORD,
                2,
            )),
            Box::new(FloatMatcher {
                index: 0,
                precedence: 1,
                dot: false,
                float: false,
                running: true,
            }),
            Box::new(IntegerMatcher {
                index: 0,
                precedence: 0,
                running: true,
            }),
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

    // Process and display all tokens
    println!("Tokenizing Code Sample:");
    println!("{}", "-".repeat(70));
    println!("{}", code);
    println!("{}", "-".repeat(70));
    println!(
        "{:<15} {:<15} {:<10} {:<10}",
        "TOKEN TYPE", "VALUE", "LINE", "COLUMN"
    );
    println!("{}", "-".repeat(70));

    loop {
        match lexxor.next_token() {
            Ok(Some(token)) => {
                let type_name = match token.token_type {
                    TOKEN_TYPE_WORD => "WORD",
                    TOKEN_TYPE_WHITESPACE => "WHITESPACE",
                    TOKEN_TYPE_SYMBOL => "SYMBOL",
                    TOKEN_TYPE_INTEGER => "INTEGER",
                    TOKEN_TYPE_FLOAT => "FLOAT",
                    TOKEN_TYPE_KEYWORD => "KEYWORD",
                    _ => "OTHER",
                };

                // Skip printing whitespace for cleaner output
                if token.token_type != TOKEN_TYPE_WHITESPACE {
                    println!(
                        "{:<15} {:<15} {:<10} {:<10}",
                        type_name,
                        token.value.replace("\n", "\\n"),
                        token.line,
                        token.column
                    );
                }
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

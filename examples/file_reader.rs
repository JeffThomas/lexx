use lexxor::input::InputReader;
use lexxor::matcher::integer::IntegerMatcher;
use lexxor::matcher::symbol::SymbolMatcher;
use lexxor::matcher::whitespace::WhitespaceMatcher;
use lexxor::matcher::word::WordMatcher;
use lexxor::token::TOKEN_TYPE_WORD;
use lexxor::{Lexxor, Lexxer};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Change this to your file path
    // The repo includes a sample text file in test_data/Varney-the-Vampire.txt
    let file_path = Path::new("test_data/Varney-the-Vampire.txt");

    // Open the file and create a reader
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let input = InputReader::new(Box::new(reader));

    // Create a Lexxor tokenizer for processing the file
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
            Box::new(SymbolMatcher {
                index: 0,
                precedence: 0,
                running: true,
            }),
        ],
    );

    println!("Tokenizing file: {}", file_path.display());
    println!("First 20 word tokens:");
    println!("{}", "-".repeat(50));

    let mut word_count = 0;
    let mut total_tokens = 0;

    // Process tokens until we've found 20 words or EOF
    while word_count < 20 {
        match lexxor.next_token() {
            Ok(Some(token)) => {
                total_tokens += 1;

                if token.token_type == TOKEN_TYPE_WORD {
                    word_count += 1;
                    println!(
                        "Word {}: '{}' (line: {}, column: {})",
                        word_count, token.value, token.line, token.column
                    );
                }
            }
            Ok(None) => {
                println!("\nEnd of file reached.");
                break;
            }
            Err(e) => {
                println!("Error during tokenization: {}", e);
                break;
            }
        }
    }

    println!(
        "\nProcessed {} tokens to find {} words.",
        total_tokens, word_count
    );

    Ok(())
}

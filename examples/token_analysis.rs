use std::collections::HashMap;
use lexx::{Lexx, Lexxer};
use lexx::input::InputString;
use lexx::matcher::word::WordMatcher;
use lexx::matcher::whitespace::WhitespaceMatcher;
use lexx::matcher::symbol::SymbolMatcher;
use lexx::matcher::integer::IntegerMatcher;
use lexx::matcher::float::FloatMatcher;
use lexx::token::{TOKEN_TYPE_WORD, TOKEN_TYPE_WHITESPACE, TOKEN_TYPE_SYMBOL, TOKEN_TYPE_INTEGER, TOKEN_TYPE_FLOAT};

fn main() {
    // Sample text for analysis
    let text = 
        "The quick brown fox jumps over the lazy dog. It was 5.8 meters high and took 2 seconds.";
    let input = InputString::new(text.to_string());

    // Create a Lexx tokenizer with standard matchers
    let mut lexx = Lexx::<512>::new(
        Box::new(input),
        vec![
            Box::new(WhitespaceMatcher { index: 0, column: 0, line: 0, precedence: 0, running: true }),
            Box::new(WordMatcher { index: 0, precedence: 0, running: true }),
            Box::new(IntegerMatcher { index: 0, precedence: 0, running: true }),
            Box::new(FloatMatcher { index: 0, precedence: 0, dot: false, float: false, running: true }),
            Box::new(SymbolMatcher { index: 0, precedence: 0, running: true }),
        ]
    );

    // Collect all tokens into a vector
    let mut tokens = Vec::new();

    loop {
        match lexx.next_token() {
            Ok(Some(token)) => tokens.push(token),
            Ok(None) => break,
            Err(e) => {
                eprintln!("Error during tokenization: {}", e);
                return;
            }
        }
    }

    // Analyze the collected tokens
    println!("Text: \"{}\"", text);
    println!("Total tokens: {}", tokens.len());

    // Count each token type
    let mut type_counts: HashMap<u16, usize> = HashMap::new();
    for token in &tokens {
        *type_counts.entry(token.token_type).or_insert(0) += 1;
    }

    println!("\nToken type distribution:");
    println!("{:<15} {:<10}", "TYPE", "COUNT");
    println!("{}", "-".repeat(30));

    for (token_type, count) in type_counts.iter() {
        let type_name = match *token_type {
            TOKEN_TYPE_WORD => "WORD",
            TOKEN_TYPE_WHITESPACE => "WHITESPACE",
            TOKEN_TYPE_SYMBOL => "SYMBOL",
            TOKEN_TYPE_INTEGER => "INTEGER",
            TOKEN_TYPE_FLOAT => "FLOAT",
            _ => "OTHER",
        };
        println!("{:<15} {:<10}", type_name, count);
    }

    // Print all words
    println!("\nWords found (in order):");
    let words: Vec<String> = tokens
        .iter()
        .filter(|t| t.token_type == TOKEN_TYPE_WORD)
        .map(|t| t.value.clone())
        .collect();

    println!("{}", words.join(", "));

    // Calculate word frequency
    let mut word_counts: HashMap<String, usize> = HashMap::new();
    for token in tokens.iter().filter(|t| t.token_type == TOKEN_TYPE_WORD) {
        let word = token.value.to_lowercase();
        *word_counts.entry(word).or_insert(0) += 1;
    }

    println!("\nWord frequency (alphabetical):");
    println!("{:<15} {:<10}", "WORD", "COUNT");
    println!("{}", "-".repeat(30));

    let mut word_counts_vec: Vec<(String, usize)> = word_counts.into_iter().collect();
    word_counts_vec.sort_by(|a, b| a.0.cmp(&b.0));

    for (word, count) in word_counts_vec {
        println!("{:<15} {:<10}", word, count);
    }
}

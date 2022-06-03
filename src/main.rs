use lexx::input::inputs::file_input::InputFile;
use lexx::matcher::matchers::float_matcher::FloatMatcher;
use lexx::matcher::matchers::integer_matcher::IntegerMatcher;
use lexx::matcher::matchers::symbol_matcher::SymbolMatcher;
use lexx::matcher::matchers::whitespace_matcher::WhitespaceMatcher;
use lexx::matcher::matchers::word_matcher::WordMatcher;
use lexx::LexxResult::{EndOfInput, Failed, Matched};
use lexx::{util, Lexx};
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let mut integers = 0;
    let mut floats = 0;
    let mut whitespace = 0;
    let mut unique_words = HashMap::new();
    let mut words = 0;
    let mut symbols = 0;
    let mut total = 0;

    let start = Instant::now();

    let input_file = InputFile::new(String::from("Varney-the-Vampire.txt"));
    //let mut input_file = InputFile::new(String::from("small_file.txt"));

    let mut lexx = Lexx::<512>::new(
        Box::new(input_file),
        vec![
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
            Box::new(WhitespaceMatcher {
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

    loop {
        match lexx.next() {
            EndOfInput() => {
                break;
            }
            Failed() => {
                assert!(false, "Should not have failed parsing file");
            }
            Matched(token) => {
                total += 1;
                match token.token_type {
                    util::token::TOKEN_TYPE_INTEGER => {
                        integers += 1;
                    }
                    util::token::TOKEN_TYPE_FLOAT => {
                        floats += 1;
                    }
                    util::token::TOKEN_TYPE_WHITESPACE => {
                        whitespace += 1;
                    }
                    util::token::TOKEN_TYPE_SYMBOL => {
                        symbols += 1;
                    }
                    util::token::TOKEN_TYPE_WORD => {
                        words += 1;
                        let count = unique_words.entry(token.value).or_insert(0);
                        *count += 1;
                    }
                    _ => {
                        assert!(false, "Don't know what this is!")
                    }
                }
            }
        }
    }

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    println!("Total tokens: {}", total);
    println!("Integers: {}", integers);
    println!("Floats: {}", floats);
    println!("Whitespace: {}", whitespace);
    println!("Symbols: {}", symbols);
    println!("Words: {}", words);
    println!("Unique words: {}", unique_words.len());
    println!("Time elapsed is: {:?}", duration);
}

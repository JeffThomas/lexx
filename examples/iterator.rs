use lexx::Lexx;
use lexx::input::InputString;
use lexx::matcher::word::WordMatcher;
use lexx::matcher::whitespace::WhitespaceMatcher;
use lexx::matcher::symbol::SymbolMatcher;
use lexx::matcher::integer::IntegerMatcher;
use lexx::matcher::float::FloatMatcher;

fn main() {
    // Create a simple input string
    let input_text = "Hello world! This is 42 and 3.14159.";
    let input = InputString::new(input_text.to_string());

    // Create a Lexx tokenizer with standard matchers
    let lexx = Lexx::<512>::new(
        Box::new(input),
        vec![
            Box::new(WhitespaceMatcher { index: 0, column: 0, line: 0, precedence: 0, running: true }),
            Box::new(WordMatcher { index: 0, precedence: 0, running: true }),
            Box::new(IntegerMatcher { index: 0, precedence: 0, running: true }),
            Box::new(FloatMatcher { index: 0, precedence: 0, dot: false, float: false, running: true }),
            Box::new(SymbolMatcher { index: 0, precedence: 0, running: true }),
        ]
    );

    // Process tokens using the Iterator interface
    for token in lexx {
        println!("{}", token);
    }
}
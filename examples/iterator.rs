use lexxor::Lexxor;
use lexxor::input::InputString;
use lexxor::matcher::float::FloatMatcher;
use lexxor::matcher::integer::IntegerMatcher;
use lexxor::matcher::symbol::SymbolMatcher;
use lexxor::matcher::whitespace::WhitespaceMatcher;
use lexxor::matcher::word::WordMatcher;

fn main() {
    // Create a simple input string
    let input_text = "Hello world! This is 42 and 3.14159.";
    let input = InputString::new(input_text.to_string());

    // Create a Lexxor tokenizer with standard matchers
    let lexxor = Lexxor::<512>::new(
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

    // Process tokens using the Iterator interface
    for token in lexxor {
        println!("{}", token);
    }
}

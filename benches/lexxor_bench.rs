use criterion::{Criterion, criterion_group, criterion_main};
use lexxor::input::InputString;
use lexxor::matcher::exact::ExactMatcher;
use lexxor::matcher::float::FloatMatcher;
use lexxor::matcher::integer::IntegerMatcher;
use lexxor::matcher::keyword::KeywordMatcher;
use lexxor::matcher::symbol::SymbolMatcher;
use lexxor::matcher::whitespace::WhitespaceMatcher;
use lexxor::matcher::word::WordMatcher;
use lexxor::{Lexxor, Lexxer};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

fn bench_lexxor_tokenization(c: &mut Criterion) {
    let input = std::fs::read_to_string("./test_data/utf-8-sampler.txt").unwrap();
    c.bench_function("lexxor_tokenization_utf8_sampler", |b| {
        b.iter(|| {
            let lexxor_input = InputString::new(input.clone());
            let mut lexxor: Box<dyn Lexxer> = Box::new(Lexxor::<4096>::new(
                Box::new(lexxor_input),
                vec![
                    Box::new(KeywordMatcher::build_matcher_keyword(
                        vec![
                            "let", "fn", "if", "else", "match", "for", "in", "while", "return",
                        ],
                        20,
                        2,
                    )),
                    Box::new(ExactMatcher::build_exact_matcher(
                        vec!["==", "!=", "<=", ">=", "&&", "||", "=>", "->", "::"],
                        10,
                        1,
                    )),
                    Box::new(FloatMatcher {
                        index: 0,
                        precedence: 0,
                        dot: false,
                        float: false,
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
                    Box::new(WordMatcher {
                        index: 0,
                        precedence: 0,
                        running: true,
                    }),
                    Box::new(WhitespaceMatcher {
                        index: 0,
                        column: 0,
                        line: 0,
                        precedence: 0,
                        running: true,
                    }),
                ],
            ));
            while let Ok(Some(_t)) = lexxor.next_token() {
                // consume token
            }
        })
    });
}

fn bench_lexxor_small_file(c: &mut Criterion) {
    let input = std::fs::read_to_string("./test_data/small_file.txt").unwrap();
    c.bench_function("lexxor_tokenization_small_file", |b| {
        b.iter(|| {
            let lexxor_input = InputString::new(input.clone());
            let mut lexxor: Box<dyn Lexxer> = Box::new(Lexxor::<256>::new(
                Box::new(lexxor_input),
                vec![
                    Box::new(KeywordMatcher::build_matcher_keyword(
                        vec!["let", "fn"],
                        20,
                        2,
                    )),
                    Box::new(ExactMatcher::build_exact_matcher(vec!["==", "!="], 10, 1)),
                    Box::new(FloatMatcher {
                        index: 0,
                        precedence: 0,
                        dot: false,
                        float: false,
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
                    Box::new(WordMatcher {
                        index: 0,
                        precedence: 0,
                        running: true,
                    }),
                    Box::new(WhitespaceMatcher {
                        index: 0,
                        column: 0,
                        line: 0,
                        precedence: 0,
                        running: true,
                    }),
                ],
            ));
            while let Ok(Some(_t)) = lexxor.next_token() {
                // consume token
            }
        })
    });
}

fn bench_lexxor_stress_test(c: &mut Criterion) {
    let line = "let x = 42; fn test() { for i in 0..1000 { if i % 2 == 0 { x += i; } } } ";
    let input = line.repeat(100_000); // ~6.5MB
    c.bench_function("lexxor_stress_test_large_repeat", |b| {
        b.iter(|| {
            let lexxor_input = InputString::new(input.clone());
            let mut lexxor: Box<dyn Lexxer> = Box::new(Lexxor::<4096>::new(
                Box::new(lexxor_input),
                vec![
                    Box::new(KeywordMatcher::build_matcher_keyword(
                        vec![
                            "let", "fn", "if", "else", "match", "for", "in", "while", "return",
                        ],
                        20,
                        2,
                    )),
                    Box::new(ExactMatcher::build_exact_matcher(
                        vec!["==", "!=", "<=", ">=", "&&", "||", "=>", "->", "::"],
                        10,
                        1,
                    )),
                    Box::new(FloatMatcher {
                        index: 0,
                        precedence: 0,
                        dot: false,
                        float: false,
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
                    Box::new(WordMatcher {
                        index: 0,
                        precedence: 0,
                        running: true,
                    }),
                    Box::new(WhitespaceMatcher {
                        index: 0,
                        column: 0,
                        line: 0,
                        precedence: 0,
                        running: true,
                    }),
                ],
            ));
            while let Ok(Some(_t)) = lexxor.next_token() {
                // consume token
            }
        })
    });
}

fn bench_lexxor_random_input(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(42);
    let charset: Vec<char> = (32u8..127u8).map(|b| b as char).collect();
    let input: String = (0..5_000_000)
        .map(|_| charset[rng.random_range(0..charset.len())])
        .collect();
    c.bench_function("lexxor_random_input_5M_ascii", |b| {
        b.iter(|| {
            let lexxor_input = InputString::new(input.clone());
            let mut lexxor: Box<dyn Lexxer> = Box::new(Lexxor::<4096>::new(
                Box::new(lexxor_input),
                vec![
                    Box::new(KeywordMatcher::build_matcher_keyword(
                        vec![
                            "let", "fn", "if", "else", "match", "for", "in", "while", "return",
                        ],
                        20,
                        2,
                    )),
                    Box::new(ExactMatcher::build_exact_matcher(
                        vec!["==", "!=", "<=", ">=", "&&", "||", "=>", "->", "::"],
                        10,
                        1,
                    )),
                    Box::new(FloatMatcher {
                        index: 0,
                        precedence: 0,
                        dot: false,
                        float: false,
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
                    Box::new(WordMatcher {
                        index: 0,
                        precedence: 0,
                        running: true,
                    }),
                    Box::new(WhitespaceMatcher {
                        index: 0,
                        column: 0,
                        line: 0,
                        precedence: 0,
                        running: true,
                    }),
                ],
            ));
            while let Ok(Some(_t)) = lexxor.next_token() {
                // consume token
            }
        })
    });
}

fn bench_lexxor_varney_file(c: &mut Criterion) {
    let input =
        std::fs::read_to_string("./test_data/Varney-the-Vampire.txt").expect("Varney file missing");
    c.bench_function("lexxor_tokenization_varney_vampire_txt", |b| {
        b.iter(|| {
            let lexxor_input = InputString::new(input.clone());
            let mut lexxor: Box<dyn Lexxer> = Box::new(Lexxor::<4096>::new(
                Box::new(lexxor_input),
                vec![
                    Box::new(KeywordMatcher::build_matcher_keyword(
                        vec![
                            "let", "fn", "if", "else", "match", "for", "in", "while", "return",
                        ],
                        20,
                        2,
                    )),
                    Box::new(ExactMatcher::build_exact_matcher(
                        vec!["==", "!=", "<=", ">=", "&&", "||", "=>", "->", "::"],
                        10,
                        1,
                    )),
                    Box::new(FloatMatcher {
                        index: 0,
                        precedence: 0,
                        dot: false,
                        float: false,
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
                    Box::new(WordMatcher {
                        index: 0,
                        precedence: 0,
                        running: true,
                    }),
                    Box::new(WhitespaceMatcher {
                        index: 0,
                        column: 0,
                        line: 0,
                        precedence: 0,
                        running: true,
                    }),
                ],
            ));
            while let Ok(Some(_t)) = lexxor.next_token() {
                // consume token
            }
        })
    });
}

fn bench_lexxor_large_file(c: &mut Criterion) {
    let input =
        std::fs::read_to_string("./test_data/Varney-the-Vampire.txt").expect("large file missing");
    c.bench_function("lexxor_tokenization_large_file_txt", |b| {
        b.iter(|| {
            let lexxor_input = InputString::new(input.clone());
            let mut lexxor: Box<dyn Lexxer> = Box::new(Lexxor::<4096>::new(
                Box::new(lexxor_input),
                vec![
                    Box::new(KeywordMatcher::build_matcher_keyword(
                        vec![
                            "let", "fn", "if", "else", "match", "for", "in", "while", "return",
                        ],
                        20,
                        2,
                    )),
                    Box::new(ExactMatcher::build_exact_matcher(
                        vec!["==", "!=", "<=", ">=", "&&", "||", "=>", "->", "::"],
                        10,
                        1,
                    )),
                    Box::new(FloatMatcher {
                        index: 0,
                        precedence: 0,
                        dot: false,
                        float: false,
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
                    Box::new(WordMatcher {
                        index: 0,
                        precedence: 0,
                        running: true,
                    }),
                    Box::new(WhitespaceMatcher {
                        index: 0,
                        column: 0,
                        line: 0,
                        precedence: 0,
                        running: true,
                    }),
                ],
            ));
            while let Ok(Some(_t)) = lexxor.next_token() {
                // consume token
            }
        })
    });
}

criterion_group!(
    benches,
    bench_lexxor_tokenization,
    bench_lexxor_small_file,
    bench_lexxor_stress_test,
    bench_lexxor_random_input,
    bench_lexxor_varney_file,
    bench_lexxor_large_file,
);
criterion_main!(benches);

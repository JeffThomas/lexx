use criterion::{Criterion, criterion_group, criterion_main};
use lexx::input::InputString;
use lexx::matcher::exact::ExactMatcher;
use lexx::matcher::float::FloatMatcher;
use lexx::matcher::integer::IntegerMatcher;
use lexx::matcher::keyword::KeywordMatcher;
use lexx::matcher::symbol::SymbolMatcher;
use lexx::matcher::whitespace::WhitespaceMatcher;
use lexx::matcher::word::WordMatcher;
use lexx::{Lexx, Lexxer};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

fn bench_lexx_tokenization(c: &mut Criterion) {
    let input = std::fs::read_to_string("./test_data/utf-8-sampler.txt").unwrap();
    c.bench_function("lexx_tokenization_utf8_sampler", |b| {
        b.iter(|| {
            let lexx_input = InputString::new(input.clone());
            let mut lexx: Box<dyn Lexxer> = Box::new(Lexx::<4096>::new(
                Box::new(lexx_input),
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
            while let Ok(Some(_t)) = lexx.next_token() {
                // consume token
            }
        })
    });
}

fn bench_lexx_small_file(c: &mut Criterion) {
    let input = std::fs::read_to_string("./test_data/small_file.txt").unwrap();
    c.bench_function("lexx_tokenization_small_file", |b| {
        b.iter(|| {
            let lexx_input = InputString::new(input.clone());
            let mut lexx: Box<dyn Lexxer> = Box::new(Lexx::<256>::new(
                Box::new(lexx_input),
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
            while let Ok(Some(_t)) = lexx.next_token() {
                // consume token
            }
        })
    });
}

fn bench_lexx_stress_test(c: &mut Criterion) {
    let line = "let x = 42; fn test() { for i in 0..1000 { if i % 2 == 0 { x += i; } } } ";
    let input = line.repeat(100_000); // ~6.5MB
    c.bench_function("lexx_stress_test_large_repeat", |b| {
        b.iter(|| {
            let lexx_input = InputString::new(input.clone());
            let mut lexx: Box<dyn Lexxer> = Box::new(Lexx::<4096>::new(
                Box::new(lexx_input),
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
            while let Ok(Some(_t)) = lexx.next_token() {
                // consume token
            }
        })
    });
}

fn bench_lexx_random_input(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(42);
    let charset: Vec<char> = (32u8..127u8).map(|b| b as char).collect();
    let input: String = (0..5_000_000)
        .map(|_| charset[rng.random_range(0..charset.len())])
        .collect();
    c.bench_function("lexx_random_input_5M_ascii", |b| {
        b.iter(|| {
            let lexx_input = InputString::new(input.clone());
            let mut lexx: Box<dyn Lexxer> = Box::new(Lexx::<4096>::new(
                Box::new(lexx_input),
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
            while let Ok(Some(_t)) = lexx.next_token() {
                // consume token
            }
        })
    });
}

fn bench_lexx_varney_file(c: &mut Criterion) {
    let input =
        std::fs::read_to_string("./test_data/Varney-the-Vampire.txt").expect("Varney file missing");
    c.bench_function("lexx_tokenization_varney_vampire_txt", |b| {
        b.iter(|| {
            let lexx_input = InputString::new(input.clone());
            let mut lexx: Box<dyn Lexxer> = Box::new(Lexx::<4096>::new(
                Box::new(lexx_input),
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
            while let Ok(Some(_t)) = lexx.next_token() {
                // consume token
            }
        })
    });
}

fn bench_lexx_large_file(c: &mut Criterion) {
    let input =
        std::fs::read_to_string("./test_data/Varney-the-Vampire.txt").expect("large file missing");
    c.bench_function("lexx_tokenization_large_file_txt", |b| {
        b.iter(|| {
            let lexx_input = InputString::new(input.clone());
            let mut lexx: Box<dyn Lexxer> = Box::new(Lexx::<4096>::new(
                Box::new(lexx_input),
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
            while let Ok(Some(_t)) = lexx.next_token() {
                // consume token
            }
        })
    });
}

criterion_group!(
    benches,
    bench_lexx_tokenization,
    bench_lexx_small_file,
    bench_lexx_stress_test,
    bench_lexx_random_input,
    bench_lexx_varney_file,
    bench_lexx_large_file,
);
criterion_main!(benches);

use criterion::{Criterion, criterion_group, criterion_main};
use lexx::input::InputReader;
use lexx::matcher::exact::ExactMatcher;
use lexx::matcher::float::FloatMatcher;
use lexx::matcher::integer::IntegerMatcher;
use lexx::matcher::keyword::KeywordMatcher;
use lexx::matcher::symbol::SymbolMatcher;
use lexx::matcher::whitespace::WhitespaceMatcher;
use lexx::matcher::word::WordMatcher;
use lexx::{Lexx, Lexxer};
use std::fs::{self, File};
use std::io::BufReader;

const DIR: &str = r"C:\Users\jefft\OneDrive\Documents\gutenburg\bucket";

fn make_default_lexx<R: std::io::Read + std::fmt::Debug + 'static>(reader: R) -> Lexx<512> {
    Lexx::new(
        Box::new(InputReader::new(reader)),
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
    )
}

fn bench_parse_all_txt_files(_c: &mut Criterion) {
    let paths = fs::read_dir(DIR).expect("Could not read directory");
    let mut total_tokens = 0u64;
    for entry in paths {
        let entry = entry.expect("Could not get entry");
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "txt" {
                let file = File::open(&path).expect("Could not open file");
                let mut lexx = make_default_lexx(BufReader::new(file));
                loop {
                    match lexx.next_token() {
                        Ok(Some(_token)) => {
                            total_tokens += 1;
                        }
                        Ok(None) => break,
                        Err(_e) => break, // Ignore errors for benchmarking
                    }
                }
            }
            println!("Total tokens parsed: {}: {}", total_tokens, path.display());
            total_tokens = 0;
        }
    }
    println!("Total tokens parsed: {}", total_tokens);
}

// fn bench_parse_all_txt_files(c: &mut Criterion) {
//     let mut paths = fs::read_dir(DIR).expect("Could not read directory");
//     let mut total_tokens = 0u64;
//     c.bench_function("lexx_gutenberg_utf8_sampler", |b| {
//         b.iter(|| {
//             let entry = paths.next().expect("Could not get next entry");
//             let path = entry.unwrap().path();
//             if let Some(ext) = path.extension() {
//                 if ext == "txt" {
//                     let file = File::open(&path).expect("Could not open file");
//                     let mut lexx = make_default_lexx(BufReader::new(file));
//                     loop {
//                         match lexx.next_token() {
//                             Ok(Some(_token)) => {
//                                 total_tokens += 1;
//                             }
//                             Ok(None) => break,
//                             Err(_e) => break, // Ignore errors for benchmarking
//                         }
//                     }
//                 }
//                 println!("Total tokens parsed: {}: {}", total_tokens, path.display());
//                 total_tokens = 0;
//             }
//         })
//     });
//     println!("Total tokens parsed: {}", total_tokens);
// }

criterion_group!(benches, bench_parse_all_txt_files);
criterion_main!(benches);

//! Lexx is a fast, extensible, greedy, single-pass string tokenizer.
//!
//! Lexx uses an [Input](crate::input::Input) that provides chars which are fed to
//! [Matcher](crate::matcher::Matcher) instances until and if the longest match is found, which is returned
//! as a [Token](crate::util::token::Token). The [Token](crate::util::token::Token) includes
//! a the type and the string matched as well as the line and column where the match was made. A custom [Input](crate::input::Input)
//! can be passed to Lexx but the library comes with implementations for [InputString](crate::input::inputs::string_input::InputString)
//! and [InputFile](crate::input::inputs::file_input::InputFile) types.
//!
//!
//! Custom [Matcher](crate::matcher::Matcher)s can also be made though Lexx comes with:
//! - [WordMatcher](crate::matcher::matchers::word_matcher::WordMatcher) matches alphabetic characters such as `gFhdds` and `word`
//! - [IntegerMatcher](crate::matcher::matchers::integer_matcher::IntegerMatcher) matches integers such as `3` or `14537`
//! - [FloatMatcher](crate::matcher::matchers::float_matcher::FloatMatcher) matches floats such as `434.312` or `0.001`
//! - [ExactMatcher](crate::matcher::matchers::exact_matcher::ExactMatcher) matches specified char combinations `39yNho^#@` or `==1==dd=03`
//! - [SymbolMatcher](crate::matcher::matchers::symbol_matcher::SymbolMatcher) matches all non alphanumeric `*&)_#@` or `.`
//! - [KeywordMatcher](crate::matcher::matchers::keyword_matcher::KeywordMatcher) matches specific passed in words such as `The` or `specific`
//! - [WhitespaceMatcher](crate::matcher::matchers::whitespace_matcher::WhitespaceMatcher) matches whitespace such as `  ` or `\t\r\n`
//!
//!
//! # Panics
//!
//! For speed Lexx does not dynamically allocate buffer space, `Lexx<CAP>` is parameterized by
//! CAP which is the maximum possible token size, if that size is exceeded a panic will be thrown.
//!
//! # Example
//!
//! ```rust
//! use lexx::Lexx;
//! use lexx::util::token::TOKEN_TYPE_WORD;
//! use lexx::util::token::TOKEN_TYPE_WHITESPACE;
//! use lexx::input::inputs::string_input::InputString;
//! use lexx::matcher::matchers::whitespace_matcher::WhitespaceMatcher;
//! use lexx::matcher::matchers::word_matcher::WordMatcher;
//! use lexx::LexxResult::Matched;
//!
//! let mut input = InputString::new(String::from("The quick\n\nbrown fox"));
//!
//! let mut lexx = Lexx::<512>::new(
//!   Box::new(input),
//!   vec![
//!     Box::new(WordMatcher{ index: 0, precedence: 0, running: true }),
//!     Box::new(WhitespaceMatcher { index: 0, precedence: 0, running: true })
//!   ]
//! );
//!
//! assert!(matches!(lexx.next(), Matched(t) if t.value == "The" && t.token_type == TOKEN_TYPE_WORD && t.line == 1 && t.column == 1));
//! assert!(matches!(lexx.next(), Matched(t) if t.token_type == TOKEN_TYPE_WHITESPACE));
//! assert!(matches!(lexx.next(), Matched(t) if t.value == "quick" && t.token_type == TOKEN_TYPE_WORD && t.line == 1 && t.column == 5));
//! assert!(matches!(lexx.next(), Matched(t) if t.token_type == TOKEN_TYPE_WHITESPACE));
//! assert!(matches!(lexx.next(), Matched(t) if t.value == "brown" && t.token_type == TOKEN_TYPE_WORD && t.line == 3 && t.column == 1));
//! assert!(matches!(lexx.next(), Matched(t) if t.token_type == TOKEN_TYPE_WHITESPACE));
//! assert!(matches!(lexx.next(), Matched(t) if t.value == "fox" && t.token_type == TOKEN_TYPE_WORD && t.line == 3 && t.column == 7));
//! ```
//     missing_docs,
//     missing_debug_implementations,
//     missing_debug_implementations,
//    missing_copy_implementations,
//    missing_copy_implementations,

#![deny(
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

pub mod input;
pub mod matcher;
pub mod util;

use arrayvec::ArrayVec;
use std::collections::HashMap;

use crate::input::Input;
use crate::matcher::Matcher;
use crate::matcher::MatcherResult::{Failed, Matched, Running};
use crate::util::rolling_char_buffer::RollingCharBuffer;
use crate::util::token::Token;

#[derive(Clone, Debug)]
pub enum LexxResult {
    EndOfInput(),
    Failed(),
    Matched(Token),
}

pub struct Lexx<const CAP: usize> {
    matchers: Vec<Box<dyn Matcher>>,
    input: Box<dyn Input>,
    cache: Box<RollingCharBuffer<CAP>>,
    value: Box<ArrayVec<char, CAP>>,
    pub lexx_result: Option<LexxResult>,
    pub final_token: Option<Token>,
    pub line: usize,
    pub column: usize,
    pub ctx: Box<HashMap<String, i32>>,
}

impl<const CAP: usize> Lexx<CAP> {
    pub fn new(input: Box<dyn Input>, matchers: Vec<Box<dyn Matcher>>) -> Self {
        let cache = Box::new(RollingCharBuffer::<CAP>::new());
        Lexx {
            matchers,
            input,
            cache,
            value: Box::new(ArrayVec::<char, CAP>::new()),
            lexx_result: None,
            final_token: None,
            line: 1,
            column: 1,
            ctx: Box::new(HashMap::new()),
        }
    }

    fn get_token(&mut self) -> LexxResult {
        let mut precedence = 0;
        self.value.clear();
        for m in self.matchers.as_mut_slice() {
            m.reset(&mut self.ctx);
        }
        loop {
            let c = if self.cache.is_empty() {
                self.input.next()
            } else {
                Some(self.cache.pop().unwrap())
            };
            let mut found_token: Option<Token> = None;
            let mut running = false;

            if c.is_some() {
                self.value.push(c.unwrap());
            }

            for m in self.matchers.as_mut_slice() {
                if m.is_running() {
                    let int_result =
                        m.find_match(c, &self.value[0..self.value.len()], &mut self.ctx);
                    match int_result {
                        Running() => {
                            running = true;
                        }
                        Matched(token) => {
                            if found_token.is_some() {
                                if precedence <= token.precedence {
                                    drop(found_token.unwrap());
                                    precedence = token.precedence;
                                    found_token = Some(token);
                                }
                            } else {
                                precedence = token.precedence;
                                found_token = Some(token)
                            }
                        }
                        Failed() => {}
                    }
                }
            }

            if found_token.is_some() {
                self.final_token = found_token
            }

            if !running {
                if self.final_token.is_some() {
                    let mut token = self.final_token.as_ref().unwrap().clone();
                    self.final_token = None;
                    if self.value.len() > token.len {
                        if let Err(e) = self.cache.extend(&self.value[token.len..self.value.len()])
                        {
                            panic!("Ran out of buffer space: {}", e)
                        };
                    }
                    token.line = self.line;
                    token.column = self.column;
                    let mut cr = false;
                    for i in 0..token.len {
                        // eat any following \n. Thanks Windows.
                        if !(cr && self.value[i] == '\n') {
                            self.column += 1;
                        }
                        if self.value[i] == '\r' || (self.value[i] == '\n' && !cr) {
                            cr = false;
                            if self.value[i] == '\r' {
                                cr = true;
                            }
                            self.column = 1;
                            self.line += 1;
                        }
                    }
                    return LexxResult::Matched(token);
                } else {
                    if c.is_none() {
                        return LexxResult::EndOfInput();
                    }
                    return LexxResult::Failed();
                }
            }
            if c.is_none() {
                return LexxResult::EndOfInput();
            }
        } // loop
    }

    pub fn next(&mut self) -> LexxResult {
        if self.lexx_result.is_some() {
            let lr = self.lexx_result.clone().unwrap();
            self.lexx_result = None;
            return lr;
        }
        return self.get_token();
    }

    pub fn look_ahead(&mut self) -> LexxResult {
        if self.lexx_result.is_some() {
            self.lexx_result.clone().unwrap()
        } else {
            self.lexx_result = Some(self.get_token());
            self.lexx_result.clone().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::input::inputs::file_input::InputFile;
    use crate::input::inputs::string_input::InputString;
    use crate::matcher::matchers::exact_matcher::ExactMatcher;
    use crate::matcher::matchers::float_matcher::FloatMatcher;
    use crate::matcher::matchers::integer_matcher::IntegerMatcher;
    use crate::matcher::matchers::symbol_matcher::SymbolMatcher;
    use crate::matcher::matchers::whitespace_matcher::WhitespaceMatcher;
    use crate::matcher::matchers::word_matcher::WordMatcher;
    use crate::util::token::TOKEN_TYPE_EXACT;
    use crate::Lexx;
    use crate::LexxResult::{EndOfInput, Failed, Matched};
    use std::collections::HashMap;
    use std::time::Instant;

    #[test]
    fn lexx_parse_large_file() {
        let mut integers = 0;
        let mut floats = 0;
        let mut whitespace = 0;
        let mut unique_words = HashMap::new();
        let mut words = 0;
        let mut symbols = 0;
        let mut total = 0;
        let mut lines = 0;

        let start = Instant::now();

        let input_file = InputFile::new(String::from("Varney-the-Vampire.txt"));

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
                    lines = token.line;
                    match token.token_type {
                        crate::util::token::TOKEN_TYPE_INTEGER => {
                            integers += 1;
                        }
                        crate::util::token::TOKEN_TYPE_FLOAT => {
                            floats += 1;
                        }
                        crate::util::token::TOKEN_TYPE_WHITESPACE => {
                            whitespace += 1;
                        }
                        crate::util::token::TOKEN_TYPE_SYMBOL => {
                            //println!("'{}'", token.value);
                            symbols += 1;
                        }
                        crate::util::token::TOKEN_TYPE_WORD => {
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
        assert_eq!(743527, total);
        assert_eq!(125, integers);
        assert_eq!(1, floats);
        assert_eq!(332190, whitespace);
        assert_eq!(72302, symbols);
        assert_eq!(338909, words);
        assert_eq!(13267, unique_words.len());
        assert_eq!(43683, lines);
        println!("Time elapsed is: {:?}", duration);
    }

    #[test]
    fn lexx_test_precedence() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("fox"))),
            vec![
                Box::new(ExactMatcher::build_exact_matcher(
                    vec!["fox"],
                    TOKEN_TYPE_EXACT,
                    1,
                )),
                Box::new(WordMatcher {
                    index: 0,
                    precedence: 0,
                    running: true,
                }),
            ],
        );

        assert!(
            matches!(lexx.next(), Matched(t) if t.value == "fox" && t.token_type == TOKEN_TYPE_EXACT)
        );

        drop(lexx);
        lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("fox"))),
            vec![
                Box::new(WordMatcher {
                    index: 0,
                    precedence: 0,
                    running: true,
                }),
                Box::new(ExactMatcher::build_exact_matcher(
                    vec!["fox"],
                    TOKEN_TYPE_EXACT,
                    1,
                )),
            ],
        );

        assert!(
            matches!(lexx.next(), Matched(t) if t.value == "fox" && t.token_type == TOKEN_TYPE_EXACT)
        );
    }

    #[test]
    fn lexx_test_look_ahead() {
        let mut lexx = Lexx::<512>::new(
            Box::new(InputString::new(String::from("The lazy dog"))),
            vec![
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
            ],
        );

        assert!(matches!(lexx.next(), Matched(t) if t.value == "The"));
        assert!(
            matches!(lexx.next(), Matched(t) if t.token_type == crate::util::token::TOKEN_TYPE_WHITESPACE)
        );
        assert!(
            matches!(lexx.look_ahead(), Matched(t) if t.value == "lazy" && t.line == 1 && t.column == 5 && t.len == 4)
        );
        assert!(
            matches!(lexx.look_ahead(), Matched(t) if t.value == "lazy" && t.line == 1 && t.column == 5 && t.len == 4)
        );
        assert!(
            matches!(lexx.next(), Matched(t) if t.value == "lazy" && t.line == 1 && t.column == 5 && t.len == 4)
        );
        assert!(
            matches!(lexx.next(), Matched(t) if t.token_type == crate::util::token::TOKEN_TYPE_WHITESPACE)
        );
        assert!(
            matches!(lexx.next(), Matched(t) if t.value == "dog" && t.line == 1 && t.column == 10 && t.len == 3)
        );
    }
}

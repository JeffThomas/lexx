# Lexx

Lexx is a fast, extensible, greedy, single-pass text tokenizer.

Sample output for the string "This is  \n1.0 thing."
```
use lexx::token::Token;
Token{ token_type: 4, value: "This".to_string(), line: 1, column: 1, len: 4, precedence: 0};
Token{ token_type: 3, value: " ".to_string(), line: 1, column: 5, len: 1, precedence: 0};
Token{ token_type: 4, value: "is".to_string(), line: 1, column: 6, len: 2, precedence: 0};
Token{ token_type: 3, value: "  \n".to_string(), line: 1, column: 8, len: 3, precedence: 0};
Token{ token_type: 2, value: "1.0".to_string(), line: 2, column: 1, len: 3, precedence: 0};
Token{ token_type: 3, value: " ".to_string(), line: 2, column: 4, len: 1, precedence: 0};
Token{ token_type: 4, value: "thing".to_string(), line: 2, column: 5, len: 5, precedence: 0};
Token{ token_type: 5, value: ".".to_string(), line: 2, column: 10, len: 1, precedence: 0};
```
# Structure

Lexx consist of 4 different componants:
* `LexxInuput` provides a stream of `char` characters
* `Matchers` which are used to identify parts of a string, such as Integers or Symbols
* `Token` which is the results of a successful match
* `Lexx` itself

# Functionality

Lexx uses a [LexxInput](crate::input::LexxInput) to provide chars that are fed to
[Matcher](crate::matcher::Matcher) instances until the longest match is found, if any. The
match will be returned as a [Token](token::Token) instance. The
[Token](token::Token) includes a type and the string matched as well as the
line and column where the match was made. A custom [LexxInput](crate::input::LexxInput)
can be passed to Lexx but the library comes with implementations for
[String](crate::input::InputString) and
[Reader](crate::input::InputReader) types.

Lexx implements [Iterator] so it can be use with `for each`.

Custom [Matcher](crate::matcher::Matcher)s can also be made though Lexx comes with:
- [WordMatcher](crate::matcher_word::WordMatcher) matches alphabetic characters such as `ABCdef` and `word`
- [IntegerMatcher](crate::matcher_integer::IntegerMatcher) matches integers such as `3` or `14537`
- [FloatMatcher](crate::matcher_float::FloatMatcher) matches floats such as `434.312` or `0.001`
- [ExactMatcher](crate::matcher_exact::ExactMatcher) given a vector of strings matches exactly those strings.
You can initialize it with a Type to return so you can use multiple ones for different things. For example one
[ExactMatcher](crate::matcher_exact::ExactMatcher) can
be used to find operators such as `==` and `+` while another could be used to find block identifiers
such as `(` and `)`.
- [SymbolMatcher](crate::matcher_symbol::SymbolMatcher) matches all non alphanumerics `*&)_#@` or `.`.
This is a good catch-all matcher.
- [KeywordMatcher](crate::matcher_keyword::KeywordMatcher) matches specific passed in words such as
`new` or `specific`, it differs from the [ExactMatcher](crate::matcher_exact::ExactMatcher) in that it
will not mach substrings, such as the `new` in `renewable` or `newfangled`.
- [WhitespaceMatcher](crate::matcher_whitespace::WhitespaceMatcher) matches whitespace such as `  ` or `\t\r\n`

[Matcher](crate::matcher::Matcher)s can be given a precedence that can make a matcher return it's
results even if another matcher has a longer match. For example, both the [WordMatcher](crate::matcher_word::WordMatcher)
and [KeywordMatcher](crate::matcher_keyword::KeywordMatcher) are used at the same time.

Note that matchers cannot find matches that start inside the valid matches of other matchers.
For matching `renewable`, the [WordMatcher](crate::matcher_word::WordMatcher)
will make the match even if the [ExactMatcher](crate::matcher_exact::ExactMatcher)
is looking for `new` with a higher precedence because the [WordMatcher](crate::matcher_word::WordMatcher)
will consume all of `renewable` without giving other matchers the chance to look inside of it.

Also while the [ExactMatcher](crate::matcher_exact::ExactMatcher)
could find the `new` inside `newfangled` the [WordMatcher](crate::matcher_word::WordMatcher)
would match `newfangled` instead since it is longer, unless the [ExactMatcher](crate::matcher_exact::ExactMatcher) is
given a higher precedence in which case it would get to return `new` and the next match would
start at `fangled`.

To successfully parse an entire stream [Lexx] must have a matcher with which to tokenize every
encountered collection of characters. If a match fails [Lexx] will return Err
[TokenNotFound](crate::LexxError::TokenNotFound) with the text that could not be matched.

# Panics

For speed Lexx does not dynamically allocate buffer space, in `Lexx<CAP>` CAP is the maximum
possible token size, if that size is exceeded a panic will be thrown.

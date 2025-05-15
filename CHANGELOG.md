# Changelog

All notable changes to the `lexx` crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- None

### Changed
- Updated github pipelines

### Fixed
- None

## [0.9.0] - 2025-05-14

### Added
- Optimized `build_exact_matcher` function for better performance
- Optimized `build_matcher_keyword` function for better performance
- Optimized `find_match` method in KeywordMatcher for better performance
- Added GitHub Actions CI/CD workflows
- Added this CHANGELOG file

### Changed
- Reduced memory allocations in ExactMatcher
- Improved character handling in KeywordMatcher
- Removed unused imports from example files

### Fixed
- None

## [0.1.0] - 2023-06-15
### Added
- Initial release of the lexx crate
- Core tokenizer functionality
- Various matcher implementations:
  - ExactMatcher
  - KeywordMatcher
  - WhitespaceMatcher
  - WordMatcher
  - SymbolMatcher
- Basic examples and documentation

[Unreleased]: https://github.com/JeffThomas/lexx/compare/v0.9.0...HEAD
[0.9.0]: https://github.com/JeffThomas/lexx/compare/v0.1.0...v0.9.0
[0.1.0]: https://github.com/JeffThomas/lexx/releases/tag/v0.1.0

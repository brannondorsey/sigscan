# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.3](https://github.com/brannondorsey/sigscan/compare/v0.2.2...v0.2.3) - 2025-05-07

### Added

- Add --empty flag ([#18](https://github.com/brannondorsey/sigscan/pull/18))

### Fixed

- Trim trailing whitespace in output ([#19](https://github.com/brannondorsey/sigscan/pull/19))

### Other

- Remove more unnecessary logic in `sigset_to_strings()` ([#22](https://github.com/brannondorsey/sigscan/pull/22))
- Remove unnecessary logic in `sigset_to_strings()` ([#21](https://github.com/brannondorsey/sigscan/pull/21))

## [0.2.2](https://github.com/brannondorsey/sigscan/compare/v0.2.1...v0.2.2) - 2025-05-04

### Other

- cargo update ([#16](https://github.com/brannondorsey/sigscan/pull/16))
- *(docs)* Update crate description ([#17](https://github.com/brannondorsey/sigscan/pull/17))
- *(docs)* Update CHANGELOG ([#14](https://github.com/brannondorsey/sigscan/pull/14))

## [0.2.1](https://github.com/brannondorsey/sigscan/compare/v0.2.0...v0.2.1) - 2025-05-02

### Fixed

- *(ci)*: Fix build-and-upload-after-release GitHub action not firing ([#11](https://github.com/brannondorsey/sigscan/pull/11))

### Other

- *(deps)* Bump nix ([#12](https://github.com/brannondorsey/sigscan/pull/12))

## [0.2.0](https://github.com/brannondorsey/sigscan/compare/v0.1.0...v0.2.0) - 2025-05-02

### Added

- Add --all/-a flag ([#8](https://github.com/brannondorsey/sigscan/pull/8))

### Fixed

- Fix panics when printing to broken pipes ([#9](https://github.com/brannondorsey/sigscan/pull/9))
- *(docs)* Fix release binary links in README ([#5](https://github.com/brannondorsey/sigscan/pull/5))

### Other

- *(docs)* Add more examples ([#10](https://github.com/brannondorsey/sigscan/pull/10))
- *(docs)* Improved install instructions in README ([#7](https://github.com/brannondorsey/sigscan/pull/7))

## [0.1.0](https://github.com/brannondorsey/sigscan/releases/tag/v0.1.0) - 2025-05-01

### Added

- *(ci)* Add release-plz and GitHub release upload actions ([#3](https://github.com/brannondorsey/sigscan/pull/3))
- Add --examples CLI argument which displays examples from the README ([#2](https://github.com/brannondorsey/sigscan/pull/2))
- Add --color and --no-color flags
- Add optional --cmdline argument
- Parse names from cmdline if they are truncated at max 15 chars
- Add --help text
- Sort signals in value order
- Add CLI arguments

### Other

- README updates
- Add installation instructions
- README updates
- Small README update
- Refactor get_name() to be more correct, readable, and faster
- Remove --cmdline argument for simplicity
- cargo update
- Small wording change
- Description change
- Add README
- *(ci)* Error on clippy warnings and check formatting in CI ([#1](https://github.com/brannondorsey/sigscan/pull/1))
- Add note to help text
- Format comments
- Add Build GH action
- Populate crates.io fields in Cargo.toml
- Add LICENSE* files
- First commit

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

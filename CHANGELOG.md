# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [4.1.0](https://github.com/oxc-project/oxc-index-vec/compare/v4.0.0...v4.1.0) - 2025-10-04

### Added

- improve Debug output for index types ([#92](https://github.com/oxc-project/oxc-index-vec/pull/92))

### Other

- update README with new features and usage examples

## [4.0.0](https://github.com/oxc-project/oxc-index-vec/compare/v3.1.0...v4.0.0) - 2025-09-30

### Added

- add const support to methods where possible ([#87](https://github.com/oxc-project/oxc-index-vec/pull/87))
- add NonMaxU32 support via define_nonmax_index_type! macro ([#86](https://github.com/oxc-project/oxc-index-vec/pull/86))

### Fixed

- reorder attributes in define_nonmax_index_type! for proc macro compatibility

### Other

- cargo fmt
- make serde and nonmax optional features
- make nonmax support always available and use crate's own nonmax
- make serde support always available and use crate's own serde
- add documentation to all public methods to fix missing_docs warnings
- remove backward compatibility alias for define_nonmax_index_type!
- rename define_nonmax_index_type! to define_nonmax_u32_index_type!
- apply tuple struct pattern to define_index_type! macro
- remove #[repr(transparent)] from define_nonmax_index_type!
- change define_nonmax_index_type! to use tuple struct pattern
- document custom attribute support in define_nonmax_index_type!
- remove bounds check from IndexVec::push ([#84](https://github.com/oxc-project/oxc-index-vec/pull/84))

## [3.1.0](https://github.com/oxc-project/oxc-index-vec/compare/v3.0.0...v3.1.0) - 2025-09-11

### Added

- implement `rayon::slice::ParallelSlice` / `rayon::slice::ParallelSliceMut` trait ([#72](https://github.com/oxc-project/oxc-index-vec/pull/72))

### Other

- *(deps)* lock file maintenance rust crates ([#61](https://github.com/oxc-project/oxc-index-vec/pull/61))
- *(deps)* lock file maintenance ([#57](https://github.com/oxc-project/oxc-index-vec/pull/57))
- *(deps)* update dependency rust to v1.89.0 ([#52](https://github.com/oxc-project/oxc-index-vec/pull/52))
# Changelog

All notable changes to this package will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project does not adhere to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) until v1.0.0.

## [2.0.1](https://github.com/oxc-project/oxc-index-vec/compare/v2.0.0...v2.0.1) - 2025-02-22

### Other

- Rust Edition 2024 ([#9](https://github.com/oxc-project/oxc-index-vec/pull/9))

## [2.0.0](https://github.com/oxc-project/oxc-index-vec/compare/v1.0.1...v2.0.0) - 2024-12-09

### Other

- use "serde" feature
- *(deps)* update dependency rust to v1.83.0 (#2)
- add README

## [1.0.1](https://github.com/oxc-project/oxc-index-vec/compare/v1.0.0...v1.0.1) - 2024-12-01

### Fixed

- clippy

### Other

- remove allowed clippy rules
- update repo
- add default features

## [0.34.0] - 2024-10-26

### Refactor

- 423d54c rust: Remove the annoying `clippy::wildcard_imports` (#6860) (Boshen)

## [0.30.0] - 2024-09-23

### Testing

- 84b7d1a index: Add unit tests to `oxc_index` (#5979) (DonIsaac)

## [0.29.0] - 2024-09-13

- 71116a1 index: [**BREAKING**] Remove ability to index `IndexVec` with `usize` (#5733) (overlookmotel)

### Features

- a362f51 index: Add `IndexVec::shrink_to` (#5713) (overlookmotel)

### Performance

- 333e2e0 index: Remove `Idx` bounds-checks from `first` + `last` methods (#5726) (overlookmotel)

## [0.28.0] - 2024-09-11

### Refactor

- 2de6ea0 index, traverse: Remove unnecessary type annotations (#5650) (overlookmotel)- 26d9235 Enable clippy::ref_as_ptr  (#5577) (夕舞八弦)

## [0.27.0] - 2024-09-06

### Features

- 4cb63fe index: Impl rayon related to trait for IndexVec (#5421) (IWANABETHATGUY)

### Documentation
- 00511fd Use `oxc_index` instead of `index_vec` in doc comments (#5423) (IWANABETHATGUY)

## [0.24.3] - 2024-08-18

### Refactor

- 786bf07 index: Shorten code and correct comment (#4905) (overlookmotel)

## [0.13.0] - 2024-05-14

### Bug Fixes

- 51de41c index: Add `example_generated` to create the docs. (#3106) (Ali Rezvani)

## [0.10.0] - 2024-03-14

### Features
- 697b6b7 Merge features `serde` and `wasm` to `serialize` (#2716) (Boshen)

## [0.5.0] - 2024-01-12

### Features

- f1b433b playground: Visualize symbol (#1886) (Dunqing)

## [0.4.0] - 2023-12-08

### Refactor

- 1a576f6 rust: Move to workspace lint table (#1444) (Boshen)


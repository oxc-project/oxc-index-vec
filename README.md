<div align="center">

[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![MIT licensed][license-badge]][license-url]
[![Build Status][ci-badge]][ci-url]
[![Sponsors][sponsors-badge]][sponsors-url]
[![Discord chat][discord-badge]][discord-url]
<!-- [![Code Coverage][code-coverage-badge]][code-coverage-url] -->
<!-- [![CodSpeed Badge][codspeed-badge]][codspeed-url] -->

</div>

# oxc-index-vec

Forked version of [index_vec](https://github.com/thomcc/index_vec).

## Features

This crate provides several optional features:

* **`rayon`** - Enables parallel iteration support via Rayon
* **`serde`** - Enables serialization/deserialization support via Serde
* **`nonmax`** - Enables `define_nonmax_u32_index_type!` macro for memory-efficient index types using `NonMaxU32`

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
oxc_index = "3.1"

# Enable optional features as needed:
# oxc_index = { version = "3.1", features = ["serde", "nonmax"] }
```

### Basic Index Type

```rust
use oxc_index::{IndexVec, define_index_type};

define_index_type! {
    pub struct MyIdx = u32;
}

let mut vec: IndexVec<MyIdx, &str> = IndexVec::new();
let idx = vec.push("hello");
assert_eq!(vec[idx], "hello");
```

### Memory-Efficient Index Type (requires `nonmax` feature)

The `define_nonmax_u32_index_type!` macro creates index types backed by `NonMaxU32`,
which uses the niche optimization to store `Option<MyIdx>` in the same space as `MyIdx`:

```rust
use oxc_index::{IndexVec, define_nonmax_u32_index_type};

define_nonmax_u32_index_type! {
    pub struct CompactIdx;
}

// Option<CompactIdx> is the same size as CompactIdx (4 bytes)
assert_eq!(
    std::mem::size_of::<CompactIdx>(),
    std::mem::size_of::<Option<CompactIdx>>()
);

let mut vec: IndexVec<CompactIdx, String> = IndexVec::new();
let idx = vec.push("world".to_string());
```

### Serialization Support (requires `serde` feature)

All index types and `IndexVec` automatically support Serde serialization when the `serde` feature is enabled:

```rust
use oxc_index::{IndexVec, define_index_type};
use serde::{Serialize, Deserialize};

define_index_type! {
    pub struct MyIdx = u32;
}

#[derive(Serialize, Deserialize)]
struct MyData {
    items: IndexVec<MyIdx, String>,
}
```

## Newly Added Features

Compared to the original `index_vec`:

* **`rayon` feature** - Parallel iteration support
* **`serde` feature** - Automatic serialization support using the crate's own serde dependency
* **`nonmax` feature** - Memory-efficient index types with `define_nonmax_u32_index_type!` macro
* **Const support** - Many methods are now `const fn` where possible
* **Proc macro compatibility** - Macros work seamlessly with custom derive attributes like `#[ast]`, `#[estree(skip)]`, etc.

<p align="center">
  <a href="https://github.com/sponsors/Boshen">
    <img src="https://raw.githubusercontent.com/Boshen/sponsors/main/sponsors.svg" alt="My sponsors" />
  </a>
</p>

[discord-badge]: https://img.shields.io/discord/1079625926024900739?logo=discord&label=Discord
[discord-url]: https://discord.gg/9uXCAwqQZW
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-url]: https://github.com/oxc-project/oxc-index-vec/blob/main/LICENSE
[ci-badge]: https://github.com/oxc-project/oxc-index-vec/actions/workflows/ci.yml/badge.svg?event=push&branch=main
[ci-url]: https://github.com/oxc-project/oxc-index-vec/actions/workflows/ci.yml?query=event%3Apush+branch%3Amain
[code-coverage-badge]: https://codecov.io/github/oxc-project/oxc-index-vec/branch/main/graph/badge.svg
[code-coverage-url]: https://codecov.io/gh/oxc-project/oxc-index-vec
[sponsors-badge]: https://img.shields.io/github/sponsors/Boshen
[sponsors-url]: https://github.com/sponsors/Boshen
[codspeed-badge]: https://img.shields.io/endpoint?url=https://codspeed.io/badge.json
[codspeed-url]: https://codspeed.io/oxc-project/oxc-index-vec
[crates-badge]: https://img.shields.io/crates/d/oxc_index?label=crates.io
[crates-url]: https://crates.io/crates/oxc_index
[docs-badge]: https://img.shields.io/docsrs/oxc_index
[docs-url]: https://docs.rs/oxc_index

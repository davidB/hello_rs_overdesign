---
#theme: beige
theme: League
---
# Hello Rust 06

Welcome

Edition 2018

---

## A bit on versioning

Semantic Versioning - https://semver.org

Calendar Versioning - https://calver.org

---

## Rust Release Cycle

All commits go to Nightly

All new features are behind feature flags that can be enabled only in Nightly
```rust
#![feature(feature_flag_name)]
```

Every 6 weeks, Nightly is promoted to Beta, Beta is promoted to Stable

When a feature is ready, the feature flag hiding it is removed

---

## RFC

Anyone can propose a new feature in an RFC

RFCs are then discussed

Once members of the core team feel an RFC have reached an agreement, either they close it or the RFC enter the final comment period

Then merged to the list of RFC awaiting implementation

---

## History

Version 0.1 (2012-01-20)

Version 1.0.0 (2015-05-15)

Current : Version 1.31.0 (2018-12-06)

---

## Edition 2015

Stable Stable Stable

---

## Edition 2018

Productivity Productivity Productivity

---

## Edition 2018

```toml
edition = "2018"
```

Very little changes actually need the 2018 edition flag

developer productivity - tooling

working groups

---

## Language Changes - NLL

Non-Lexical Lifetimes
```rust
fn main() {
    let mut x = 5;
    let y = &x;
    let z = &mut x;
}
```

---

## Language Changes - Macro

Procedural Macros

---

## Language Changes - New Keywords

`try`

`async` / `await`

Not ready for Stable yet, but keywords are now reserved

---

## Language Changes - Modules

No need for `extern crate` anymore

New `crate::` prefix for current crate

`::` prefix for external crates

```rust
use clap;
mod clap:
use ::clap;
use crate::clap;
```

---

## Tooling

Incremental Compilation

RLS & IDEs support

rustfmt

clippy

rustfix

---

## Working Groups

WebAssembly

Networking

Embedded

Command Line Tools

---

## Links

https://opensource.com/business/15/6/rust-6-week-release-cycle

https://github.com/rust-lang/rust/blob/master/RELEASES.md

https://hacks.mozilla.org/2018/12/rust-2018-is-here/

https://rust-lang-nursery.github.io/edition-guide/rust-2018/index.html

---

## Home Work (check)

improve error handling into your cli

---

## Home Work (next time)

update your CLI for edition 2018

---

## ? Next ?

Traits!

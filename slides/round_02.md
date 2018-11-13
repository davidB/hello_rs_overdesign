---
#theme: beige
theme: League
---
# Hello Rust 02

Welcome

(first cli)

---

## Home Work (check)

http://rustup.rs

https://www.rust-lang.org/

---

```sh
rustup update
rustup doc --std
```

---

```sh
cargo version
cargo new --bin hello-cli
cd hello-cli
ls -la # or tree
```

```txt
.
├── .git
│   └── ...
├── .gitignore
├── Cargo.toml
└── src
    └── main.rs
```

---

```sh
  .
  ├── Cargo.lock
  ├── Cargo.toml
  ├── build.rs
  ├── benches
  │   └── large-input.rs
  ├── examples
  │   └── simple.rs
  ├── src
  │   ├── bin
  │   │   └── another_executable.rs
  │   ├── lib.rs
  │   └── main.rs
  └── tests
      └── some-integration-tests.rs
```

---

```sh
cargo run
```

```txt
   Compiling hello-cli v0.1.0 (/Users/davidb/tmp/hello-cli)
    Finished dev [unoptimized + debuginfo] target(s) in 1.82s
     Running `target/debug/hello-cli`
Hello, world!
```

```txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/hello-cli`
Hello, world!
```

---

```sh
cargo build
ls -lh target/debug
target/debug/hello-cli
```

---

```sh
cargo build --release
ls -lh target/release
target/release/hello-cli
```

---

```sh
cat src/main.rs
cat Cargo.toml
```

```txt
[package]
name = "hello-cli"
version = "0.1.0"
authors = ["My Name <my@email>"]

[dependencies]
```

---

## https://crates.io

crates catalog

cargo doc

doc.rs

---

```toml
[dependencies]
time = "0.1.25"
```

---

```rust
extern crate time;

fn main() {
    println!("hello {}", time::now().rfc822z());
}

```

---

## Cargo install

(self-promotion)

```sh
ls -la ~/.cargo/bin
cargo install ripgrep https
cargo install git-find xsv
cargo install cargo-watch cargo-make mdbook
cargo install --list
```

```sh
brew install zeromq pkg-config
cargo install evcxr_jupyter
evcxr_jupyter --install
```

---

## Home Work

search a crates for Command Line Interface (CLI)

---

## ? Next (basic) ?

errors, test, enum, trait, iterator, string & str, owership/borrowing, log, ...

# hello.rs overdesign

Demonstration of various way to overdesign (to torture) the classic hello world in [Rust](https://www.rust-lang.org/). The variations show some features of languages or of some libs.

## Display & run the sample from Visual Studio Code

### Prepare

- On shell
  - install rust & cargo via [rustup](https://rustup.rs/)
  - install [cargo-script](https://crates.io/crates/cargo-script) in PATH: `cargo install cargo-script`
- On VS Code
  - install extension [Rust (rls)](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
  - install extension [Code Runner](https://marketplace.visualstudio.com/items?itemName=formulahendry.code-runner)
      ```json
      "code-runner.saveFileBeforeRun": true,
      "code-runner.saveAllFilesBeforeRun": true,
      "code-runner.executorMap": {
        //...
        // "rust": "cd $dir && rustc $fileName && $dir$fileNameWithoutExt",
        "rust": "cd $dir && cargo script $fileName",
        //...
      },
      ```

### Run

(shortcut for mac os x, tips: copy on a paper near computer)

1. closes every openned files
1. open every files to used for the tuto in order
1. navigate ⌘ ⌥ ← and ⌘ ⌥ →
1. toogle Zoom mode ⌘ K, Z
1. Zoom ⌘ + and ⌘ - and ⌘ 0 to adjust size for auditor
1. clear output (my user shortcut "Ctrl ⌥ C")
1. run code "Ctrl ⌥ N"
1. change between code

## Links

[ctjhoa/rust-learning: A bunch of links to blog posts, articles, videos, etc for learning Rust](https://github.com/ctjhoa/rust-learning)

### Books & Guidelines

- [The Rust Programming Language](https://www.rust-lang.org/en-US/)
- [The Rust Programming Language - 2018 Edition](https://doc.rust-lang.org/book/2018-edition/index.html)
- [Rust By Example](https://doc.rust-lang.org/stable/rust-by-example/)
- [Table of Contents - Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)
- [Rust API Guidelines](https://rust-lang-nursery.github.io/api-guidelines/about.html)
- [Rust Anthology 1](https://brson.github.io/rust-anthology/1/intro.html)
- [mre/idiomatic-rust: A peer-reviewed collection of articles/talks/repos which teach concise, idiomatic Rust.](https://github.com/mre/idiomatic-rust)

### Tools

- [The Cargo Book](https://doc.rust-lang.org/cargo/getting-started/)
- [rust-lang-nursery/rust-clippy: A bunch of lints to catch common mistakes and improve your Rust code](https://github.com/rust-lang-nursery/rust-clippy)

### Misc

- [Cargo: packages for Rust](https://crates.io/)

## Credits

- useb by UI (experimental, wip)
  - [google/diff-match-patch: Diff Match Patch is a high-performance library in multiple languages that manipulates plain text.](https://github.com/google/diff-match-patch)

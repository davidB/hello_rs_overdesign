# hello.rs overdesign

Demonstration of various way to overdesign (to torture) the classic hello world in [Rust](https://www.rust-lang.org/). The variations show some features of languages or of some libs.

## Display & run the sample from Visual Studio Code

### Prepare

- On shell
  - install rust & cargo via [rustup](https://rustup.rs/)
  - install [cargo-script](https://crates.io/crates/cargo-script) in PATH: `cargo install cargo-script`
- On VS Code, install the following extension
  - Name: Rust (rls)
    - Id: rust-lang.rust
    - Description: Rust language support - code completion, Intellisense, refactoring, reformatting, errors, snippets. A client for the Rust Language Server, built by the RLS team.
    - Version: 0.5.3
    - Publisher: rust-lang
    - VS Marketplace Link: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust
  - Name: Code Runner
    - Id: formulahendry.code-runner
    - Description: Run C, C++, Java, JS, PHP, Python, Perl, Ruby, Go, Lua, Groovy, PowerShell, CMD, BASH, F#, C#, VBScript, TypeScript, CoffeeScript, Scala, Swift, Julia, Crystal, OCaml, R, AppleScript, Elixir, VB.NET, Clojure, Haxe, Obj-C, Rust, Racket, AutoHotkey, AutoIt, Kotlin, Dart, Pascal, Haskell, Nim, D, Lisp
    - Version: 0.9.7
    - Publisher: Jun Han
    - VS Marketplace Link: https://marketplace.visualstudio.com/items?itemName=formulahendry.code-runner
    - Custom Configuration:

        ```json
        "code-runner.saveFileBeforeRun": true,
        "code-runner.saveAllFilesBeforeRun": true,
        "code-runner.executorMap": {
          //...
          // "rust": "cd $dir && rustc $fileName && $dir$fileNameWithoutExt",
          "rust": "cd $dir && cargo script $fileName",
          //...
        },
        "code-runner.customCommand": "cd $dir && cargo script --test $fileName",
        ```

  - Name: vscode-reveal
    - Id: evilz.vscode-reveal
    - Description: Show markdown as revealJs presentation
    - Version: 3.3.2
    - Publisher: evilz
    - VS Marketplace Link: https://marketplace.visualstudio.com/items?itemName=evilz.vscode-reveal


### Run

(shortcut for mac os x, tips: copy on a paper near computer)

Preparation:

1. closes every opened files
2. open every files to used for the tutorial in order, go to the first one
3. zoom `⌘ +` and `⌘ -` and `⌘ 0` to adjust size for auditor
4. 'Toggle Panel Position' to have output/terminal on Right (on landscape screen)
5. 'Toggle Zen mode' `⌘ K`, `Z`, or  'Toggle Full Screen' `Ctrl ⌥ N`

Execution:

1. 'Clear Output' `Ctrl ⌥ C`
1. 'Run Code' `Ctrl ⌥ N` on single file main
1. 'Run Custom Command' `Ctrl ⌥ K` to run test on single file
1. navigate `⌘ ⌥ ←` and `⌘ ⌥ →` between opened files

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
- [softprops/awesome-mdbook: 🗃️ a card catalog of mdbooks for your reading curiosity](https://github.com/softprops/awesome-mdbook)

### Tools

- [The Cargo Book](https://doc.rust-lang.org/cargo/getting-started/)
- [rust-lang-nursery/rust-clippy: A bunch of lints to catch common mistakes and improve your Rust code](https://github.com/rust-lang-nursery/rust-clippy)

### Misc

- [Cargo: packages for Rust](https://crates.io/)

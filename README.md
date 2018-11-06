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

## Credits

- useb by UI (experimental, wip)
  - [google/diff-match-patch: Diff Match Patch is a high-performance library in multiple languages that manipulates plain text.](https://github.com/google/diff-match-patch)

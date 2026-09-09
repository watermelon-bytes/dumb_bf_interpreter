## A naive Brainfuck implementation in 🦀

This repository contains a hobby project that I work on in my spare time, so development is intentionally irregular. There may be long periods without commits.

### Run a Brainfuck source

To run a source code file written in Brainfuck, `git clone` this repository, `cd` into the newly created directory, and launch the following command:
```bash
cargo run --release -- path/to/Brainfuck/source.brf
```
where `source.brf` is the path to a Brainfuck source file. The `.brf` file extension is not a mandatory; in fact, file extension is ignored.

Reading from standard input is not supported (yet?).

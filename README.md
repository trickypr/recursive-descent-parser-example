# Expression parser example implementation

This repository contains a simple recursive decent expression parser. Its purpose is to demonstrate the simplicity of a recursive decent parser. It contains around `150` lines of code rust code.

## Running

First, you will need to download the source code either from github using the following command, or from UC learn, if you have access to it:

```bash
git clone https://github.com/trickypr/recursive-descent-parser-example
```

Make sure you have rust installed on your system. If you need to install it, [follow the instructions on the rust website](https://www.rust-lang.org/tools/install).

You can run this program from the command line using the following command:

```bash
cargo run
# or, if you want E X T R E M E   P E R F O R M A N C E
cargo run --release
```

## Code structure

- `Cargo.toml`, `Cargo.lock`: Rust config, can be ignored
- `src/main.rs`: All of the parser code
- `src/boilerplate.rs`: Some string manipulation code
- `expr.txt`: The expression to be parsed
- `grammar.ebnf`: The grammar definition file

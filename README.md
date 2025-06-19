# Phantom

Phantom is an experimental language server for PHP written in Rust. It is composed of several crates that work together to provide parsing capabilities and an LSP server.

## Crates

- [`phantom-parser`](crates/libs/phantom-parser) – PHP parser built with [chumsky](https://github.com/zesterer/chumsky).
- [`phantom-server`](crates/phantom-server) – the language server implementation.
- [`phantom-cli`](crates/phantom-cli) – command line interface for running the parser and other utilities.
- [`phantom-logger`](crates/libs/phantom-logger) – simple logging utilities used by the server.

## Installation

Build the tools locally using `cargo`:

```bash
# Install the CLI
cargo install --path crates/phantom-cli

# Build and run the language server
cargo run -p phantom-server
```

## Usage

After installing the CLI you can parse PHP files directly:

```bash
phantom-cli parser examples/index.php
```

The language server communicates over stdin/stdout and can be launched with:

```bash
cargo run -p phantom-server
```

## Project layout

- `crates/` contains all Rust crates.
- `examples/` holds sample PHP files.
- `client/` provides a VS Code extension that connects to the server.

Phantom is licensed under the [MIT License](LICENSE).

# template_cli_std

A template for building synchronous CLI applications with Rust standard library.

## Overview

This project serves as a starting template for developing interactive command-line applications using synchronous I/O. This is the synchronous counterpart to `template_tokio`.

## Features

- **Sync REPL**: Synchronous command-line input using `rustyline`
- **Simple Architecture**: Single-threaded, blocking I/O model
- **Clean Design**: No complex concurrency primitives needed
- **Colored Output**: Terminal color support for better readability
- **Shell Word Parsing**: Command-line argument parsing capabilities

## Dependencies

- `rustyline`: Readline implementation for Rust
- `shell-words`: Shell-style string parsing

## Project Structure

```
src/
├── main.rs    # Main logic and sample implementation (text commands)
└── cli.rs     # CLI framework implementation
```

### cli.rs Components

- `Cli`: CLI initialization and readline management
- `Printer`: stdout/stderr helper with color support

### main.rs Sample Implementation

Includes a sample synchronous command processing:
- Blocking readline in main thread
- Simple text transformation commands

## How to Run Example

```sh
cargo run
```

### Example Commands

```sh
> echo hello world
hello world

> upper hello world
HELLO WORLD

> lower HELLO WORLD
hello world

> reverse hello
olleh

> q
```

## Usage

To use this template:

1. Initialize CLI with `Cli::new()`
2. Call `cli.readline()` in a loop to get `(command, args)`
3. Returns `None` on exit commands or EOF
4. Simple and straightforward - no unnecessary abstractions

## Exit Commands

The following commands will exit the application:

- `exit`
- `quit`
- `q`
- Ctrl+C
- Ctrl+D


## License

This template is free to use and modify.

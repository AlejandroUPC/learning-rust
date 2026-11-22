# Learning Rust

A personal repository for learning Rust from fundamentals through systems programming. It keeps resource progress, chapter notes, exercises, and independent projects organized in one place.

## Learning Resources

- [The Rust Programming Language](https://doc.rust-lang.org/book/) — the primary book, indexed in [`resources/the-rust-book/`](resources/the-rust-book/README.md).
- [Rustlings](https://rustlings.rust-lang.org/) — hands-on exercises to work through alongside the book; see [`resources/rustlings/`](resources/rustlings/README.md).

For a repeatable session routine, see [PROGRESS.md](PROGRESS.md).

## Repository Structure

```text
resources/      Learning resources and resource-specific progress
projects/       Larger self-contained applications
```

Each Book chapter has a directory with its own `README.md`; chapter code belongs in that directory.

Code snippets in Markdown notes may be partial or intentionally non-compilable; they are there to illustrate an idea and may need more context to compile. Runnable examples belong in a chapter's Cargo project.

## Getting Started

Install Rust using [rustup](https://rustup.rs/). The repository's `rust-toolchain.toml` selects the stable toolchain. Confirm installation with `rustc --version` and `cargo --version`. Rustlings setup and resume instructions are in [`resources/rustlings/README.md`](resources/rustlings/README.md).

Create and work with an independent Cargo project using these commands from the desired parent directory (replace `my_project` with a project name):

```sh
cargo new my_project
cd my_project
cargo run
cargo build
cargo test
cargo fmt
cargo clippy
```

## Navigation and Organization

- Track The Rust Book chapter progress in [`resources/the-rust-book/README.md`](resources/the-rust-book/README.md), and write notes in that chapter's README.
- Put chapter-related exercises and separate Cargo projects inside the relevant chapter directory. Do not add a root Cargo workspace; projects remain independent.
- Use [`projects/`](projects/README.md) for larger applications.
- Add a future learning resource as a new directory under `resources/` with a resource README, progress index, and resource-specific work.

## Overall Progress

The Book chapter checklist and Rustlings exercise state provide detailed progress tracking. Update them as you work; use this README to find each resource and work area.

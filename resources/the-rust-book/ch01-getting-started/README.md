# Chapter 1 — Getting Started

**Source:** [Official Rust Book Chapter](https://doc.rust-lang.org/stable/book/ch01-00-getting-started.html)

## Key Concepts

- `rustup` a tool to manage rust versions.
- Rust uses some weird syntax (at the moment).
- Compiles and generates and executable.
- `rustfmt` to format.
- `cargo` is the build system and packet manager for rust.

## Things I Found Interesting


### 1.1 Hello, World!

Functions are defined using the keyword `fn` followed by the name and brackets `{}` (recomended to open in same body of func sepparated by a space).
Also if it had parameters they'd be after the name wrapped in parenthesis.
**By default main function is the first code that runs on every executable**.

Now the `println!` is the part that prints the string:

```rust
println!("Hello, world!");
```

`println!` is a rust macro, thats what the `!` after the call does. Macros are a way to generate code that generates other code (templating kinda?). For now this is enough about macros.

The argument is the string we pass `"Hello, world!"`, and the `;` that ends this statement. Most Rust statements end with a semicolon; line breaks alone do not end statements.


When the code is done we compile it and we generate a binary, the content is a bit weird if you inspect it, but that's fine.

**Rust is an ahead-of-time compiled language**, you can compile it and give the binary and give it to someone without Rust, should be able to run it.

For example, this is not possible with Phython.

### 1.2 Hello, Cargo!

Mostly used to manage complex projects as its a big lift, builds code, manages dependencies and builds the libraries.

You can initialize projects running:

```sh
cargo new hello_cargo
```
You will see it creates a folder with the same name and prints some information about the newly created project, here are some of the contents:

```sh
hello_cargo
├── Cargo.toml
└── src
    └── main.rs
2 directories, 2 files
```

`Cargo.toml` contains information about the project, version, dependencies:

```sh
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

If we check the rs code you will see its a templated `Hello, World!` program. Cargo puts the main source file in `src/` by default. The project root holds `Cargo.toml` and files such as README or configuration; other code can also live in directories such as `tests/` and `examples/`.

Now to build our project we can run `cargo build`, you will see some information being printed and your project structure changed:

```sh
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
└── target
    ├── CACHEDIR.TAG
    └── debug
        ├── build
        │   └── hello_cargo
        │       └── cb81225b230cf6f7
        │           ├── fingerprint
        │           │   ├── bin-hello_cargo
        │           │   ├── bin-hello_cargo.json
        │           │   ├── dep-bin-hello_cargo
        │           │   ├── invoked.timestamp
        │           │   └── output-bin-hello_cargo
        │           └── out
        │               ├── hello_cargo
        │               ├── hello_cargo.0kaayhwosa5xo3taj94n9xkhc.09vjxg0.rcgu.o
        │               ├── hello_cargo.28dagid7ct8a39zpx3m6oj464.09vjxg0.rcgu.o
        │               ├── hello_cargo.5gkchiailt47mdc1kzv9mt9fg.09vjxg0.rcgu.o
        │               ├── hello_cargo.7ww0cvdewnhtt7kuzd1nobrqi.09vjxg0.rcgu.o
        │               ├── hello_cargo.bwefvdl0kp7dyn14v7yv8m1nf.09vjxg0.rcgu.o
        │               ├── hello_cargo.d
        │               └── hello_cargo.dpxmypnk7lagrut9xj6zcqsg4.09vjxg0.rcgu.o
        ├── examples
        ├── hello_cargo
        ├── hello_cargo.d
        └── incremental
            └── hello_cargo-3lb5yl854jazu
                ├── s-hoafp3cfpt-17yo7m5-5gl40lugbjrkvd1q5dxkwv93q
                │   ├── 0kaayhwosa5xo3taj94n9xkhc.o
                │   ├── 28dagid7ct8a39zpx3m6oj464.o
                │   ├── 5gkchiailt47mdc1kzv9mt9fg.o
                │   ├── 7ww0cvdewnhtt7kuzd1nobrqi.o
                │   ├── bwefvdl0kp7dyn14v7yv8m1nf.o
                │   ├── dep-graph.bin
                │   ├── dpxmypnk7lagrut9xj6zcqsg4.o
                │   ├── query-cache.bin
                │   └── work-products.bin
                └── s-hoafp3cfpt-17yo7m5.lock

13 directories, 29 files
```
Now this might seem intimadting but the important file is the binary generated under `target/debug`, which is the default path the binary is created, if you run it:

```sh
./target/debug/hello_cargo
Hello, world!
```

Note that its better, most of the time, to run `cargo run` which does the build + execution of the binary.

`cargo check` might be also helpful to figure out if our code could compile without actually producing the binaries (much faster).

When our project is ready to release we can run `cargo build --release`, runs with some optimizations.

## Questions / Things to Revisit

N/A

## Exercises & Code

Done in [`hello-world/`](hello-world/), including the Cargo project in [`hello-world/hello_cargo/`](hello-world/hello_cargo/).

## Progress

- [x] Read the chapter
- [x] Complete the exercises
- [x] Write my own notes
- [x] Review unclear concepts

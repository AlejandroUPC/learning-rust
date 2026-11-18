# Rustlings

**Official resource:** [Rustlings](https://rustlings.rust-lang.org/)

Rustlings is an official collection of small exercises recommended alongside The Rust Programming Language. The upstream exercise files are initialized locally; this repository tracks your exercise progress and edits without copying a second upstream checkout.

## Setup

Install Rust with [rustup](https://rustup.rs/), then install Rustlings using its official instructions:

```sh
cargo install rustlings --locked
```

For a first-time setup, run `rustlings init` from `resources/` so the initializer creates its `rustlings/` directory here, beside this README:

```sh
cd resources
rustlings init
cd rustlings
rustlings
```

The exercises and progress state live in this directory and are tracked by the parent learning repository. To resume later, start the runner here:

```sh
cd resources/rustlings
rustlings
```

## Updating

Exercise files and progress are tracked in this repository. Update the CLI with `cargo install rustlings --locked`. The CLI update does not overwrite the local exercises. To refresh exercise content, first save or commit your changes, initialize the new version in a separate temporary directory using the [official setup instructions](https://rustlings.rust-lang.org/setup/), then compare and merge upstream changes into this directory. Do not rerun initialization over your modified exercises.

## Exercise Index and Progress

Use the [official usage guide](https://rustlings.rust-lang.org/usage/) and Rustlings watch mode's interactive exercise list (`l`) as the exercise index. The initialized exercise files live under `exercises/`, with upstream topic guidance beside them. The interactive list shows completed and pending exercises.

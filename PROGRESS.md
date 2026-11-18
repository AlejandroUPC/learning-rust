# Learning Progress

Use this routine each time you study. Work through the Book and Rustlings together: read a topic, then practice related concepts with the exercises.

## Start a Study Session

From the repository root, start Rustlings:

```sh
cd resources/rustlings
rustlings
```

Rustlings opens at your next pending exercise. Use `l` to view the exercise list or choose another exercise. When you finish, exit the runner and return to the repository root.

## Study a Book Chapter

1. Open the next chapter from the [Book chapter index](resources/the-rust-book/README.md) and read the [official chapter](https://doc.rust-lang.org/book/).
2. Try the related Rustlings exercises. Edit the exercise files under `resources/rustlings/exercises/`; the runner checks your work as you go.
3. If the chapter asks you to build a program, create an independent Cargo project inside that chapter's directory. For example, from the repository root:

   ```sh
   cd resources/the-rust-book/ch02-guessing-game
   cargo new guessing_game
   cd guessing_game
   cargo run
   ```

4. Write your own takeaways, interesting details, questions, and links to code in that chapter's `README.md`.
5. Mark the chapter checklist in its README and the matching item in the Book index when ready to move on.

## Resume Later

Start `rustlings` again from `resources/rustlings/`; your exercise files and progress are kept in the repository. Continue with the next pending exercise or choose one from the list. Continue reading at the next unchecked Book chapter.

## General Commands

From a Cargo project's directory:

```sh
cargo run
cargo test
cargo fmt
cargo clippy
```

Each Cargo project is independent. Do not run these commands from the repository root, which is not a Cargo workspace.

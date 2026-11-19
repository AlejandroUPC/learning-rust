# Chapter 2 — Programming a Guessing Game

**Source:** [Official Rust Book Chapter](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html)

## Key Concepts

Starting to use some features `let`, `match`,... implementing a simple game.

## Things I Found Interesting

Using cargo to set up a new project

```sh
cargo new guessing_game
cd guessing_game
```

This generates all the boilerplate we learnt last chapter.

When we process a guess we do the following code:

```rust
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed: {guess}");
}

```

There is some stuff to unpack here line by line:


The first line is `std::io` which is a way to bring the scope of a lirbary, in this case is a standard library in rust (think of imports in Python).
Rust has this concept of `prelude` which can be understood like automated imports that it has when running a progrma, if not we need to run the line above (for whatever we want to run).

The next relevant line (`fn main() {` has been tackled) and the macros of `println!` are fine.

Declaring and storing values in variables is done in the line:

```rust
let must guess = String::new();
```

This is very interesting, `let` is a keyword to define a variable, and the keyword `mut` simply indicates if it can be mutated (altered/value changed) or not, and its optional (we should decide based on our interest).


There is also the actual value `String::new()` this is an actual function call, its the defined `String` tyepe by rust, an `UTF-8` encoded bit of text, but what does `::` mean? This line indicates that we are callign a function `new` associated to the `String` type.

To read user input we call a method (similar to `String`) for `io`:

```rust
io::stdin()
    .read_line(&mut guess)
```

The `io` is the handling of standard input/output (read/write), in the next line we call the `read_line` which literaly awaits input in `std::` for the user and we tel it to store it to a pointer in the variable `& mut`, the `&` indicates its a reference.

References is a "performant" (I guess?) to access variables without having to do copies all the time, references is a big win of using Rust.

The last line to check is `expect`; this call is chained to `read_line`, and this is because it returns a `Result` (this will be relevant for the future). `Result` is an `enumeration` (aka `enum`), that can have multiple values, each possible state is a `variant`.

The purpose of `Result` is to catch errors and handle then, its variants are `Ok` and `Err`, `Ok` means everything ran fine, and `Err` means we have to handle an error. The values of `Result` type have also methods, like `expect`, which checks if the outcome of the `Result` is `Err` will crash and display the message we pass.
If its fine though, it will take the value that `Ok` is holding.

Note that if we didn't call `expect` the compilre complains:

```sh
warning: unused `Result` that must be used
```


The last line is just printing the results with placeholding the name of the variable, we can pass by value or psition, see:


```rust
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);

```

Now the game is far from over, its a guessing game so we need to generate a number, how to? The easiest way is to generate a random number and we can do that by reusing a crate `rand` (remember a crate is nothing but a colection of source code files, like a library).
We should add it to dependencies and then re-build, under the `[dependencies]`, we can pin the versions (like Python) to ensure reproducible builds. 

We can update dependencies by running `cargo update`, our code changes now slightly:


```rust
use std::io;

use rand::prelude::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    println!("The secret number is: {secret_number}.");

    println!("Please input your guess:");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed: {guess}");
}
```

We have some new lines `use rand::prelulde::*;`, prelude just imports the most common used parts of `rand`; then we call the function `rand::rng` that gives us a number generator, local to the current hread and seeded by the operating system, then we call `random_range` method , which takes the arguments in the form of `start..=end`, which is a range.

The game is far from over, we have to compare our guess to the random number, and an easy way to do this is to import a another standard library function, `std::cmp::Ordering;` and do the following:

```rust
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
```
The match expression is made up of arms, which are kind branches the code keeps checking against, in this case just uses the `Ordering` builtin to know where our guess is based on the secret_number.

Our lsp in Nvim already gives us a hint but if we try to compile:
```sh
   Compiling guessing_game v0.1.0 (/Users/amo/Desktop/personal/learn-rust/resources/the-rust-book/ch02-guessing-game/guessing_game)
error[E0308]: mismatched types
    --> src/main.rs:21:21
     |
  21 |     match guess.cmp(&secret_number) {
     |                 --- ^^^^^^^^^^^^^^ expected `&String`, found `&{integer}`
```

Which kinda makes sense? We are using the std input as a string, but then our number is an `i32`, how are we supposed to compare letters to numbers? Maybe we should try to actually convert the guess, that by default stdin a `String` to an integer, and fail gracefully:


```rust
let guess: u32 = guess.trim().parse().expect("Please enter a number");
```


Here what we do is we overwrite the variable guess (remember it had the `mut` keyword?) so it gets shadowed and then we handle the `Result` aswell (trim for sanity). Also the code knows what to parse into because we added types to the variable `: u32` after the variable means we are using an unsigned integer for this variable, so parse tries to this.


After adding this line our game works, but we just get an attempt, ideally we get a limited number or we keep trying until we get it right, and this could be done with loops; which Rust uses with the keyword `loop`:

```rust
    // --snip--

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
```

If we actually added this snippet the code would keep running forever until we actually crashed it (passing a non-castable to `u32` value or control+c'ing); in order to handle this better we can actually add more complex code in the arms of the `match` using brakets and use the keyword `break` to leave the loop after the print:

```rust
        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

Now instead of crashing we could also manage a better way rather than crash, this can also be done by extending (similar to what we just did with arms) but with the `parse` call:

```rust
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

```

Instead of `expect` we open a block, where we handle the `Result` ourselves with a mach style:

- if `Ok` it returns the value, so we set `Ok(num`) and return using `=> num`.
- If its an error we just catch is `_` a catch-all and just continue (so we stay in loop).

Now the `continue` might not be the best approach but its just an example, but the code behaves as follows:

```rust
Guess the number!
The secret number is: 38.
Please input your guess:
foo
Please input your guess:
2
You guessed: 2
Too small!
Please input your guess:
38
You guessed: 38
You win!
```

We should def, before we "release" our game make sure the secret number is not printed.



## Questions / Things to Revisit

- Whats the diff when using `::` and a method vs chaining? E.g `io::stdin().read_line)`, why no `::` after read_line?

  **Answer:** `::` separates names in a path, while `.` calls a method on a value. In `io::stdin()`, `io` is the module and `stdin` is a function in it. That call returns a `Stdin` value; `.read_line(...)` calls a method on that value. `String::new()` uses the same path separator to name an associated function on the `String` type. “Chaining” just means calling one operation on the value returned by another; it doesn’t change which syntax to use. Rust’s method-call syntax uses `.` and can automatically borrow the receiver when needed. ([Rust paths](https://doc.rust-lang.org/reference/paths.html), [method-call expressions](https://doc.rust-lang.org/reference/expressions/method-call-expr.html))

## Exercises & Code

## Progress

- [x] Read the chapter
- [x] Complete the exercises
- [x] Write my own notes
- [x] Review unclear concepts

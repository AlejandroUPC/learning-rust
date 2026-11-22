# Chapter 3 — Common Programming Concepts

**Source:** [Official Rust Book Chapter](https://doc.rust-lang.org/stable/book/ch03-00-common-programming-concepts.html)

## Key Concepts


### Variables and Mutability

In rust by deult all variables are immutable and we need to specify the keyword `mut` to make them mutable.
Immutability, in a broader way, means that once a variable is bound to a name, you can't change its value.

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

Our LSP alraedy complaints that we cannot assign the value of a variable twice if its immutable, and if we ran `cargo run` we'd get a similar error.
Because rust is a compiled language we have this way to detect early errors, they are most of the time indicating our code is not behaving as we wanted.

Simply changing the second line of the snippet to `let mut x = 5` should fix the issues.


#### Constants

Like in Python (althgouh Python doesn't enforce it) you can declare constant in rust, also uppercase with underscores:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

Constants by default are also immutable, but they have a few differences with immutable variables:

- You can't use `mut` with constants.
- Constants can be declared in any scope, including global, so many parts of the code can use them.
- Constants can only be set to a constant expression (lol), not the result of anything that could be computed at runtime.

The above constant is defined as a multiplication, but it does not depedn on anything that happens in runtime, and we write it this way because its easy to read what it means.

#### Shadowing

When you overwrite an existing variable with a new value we say its being shadowed, it's what the compiler will see when you use the second name of the variable, for example:

```rust
fn main() {
    let x = 5;
    let x = 5 + 1;

    {
        let x = x * 2;
        println!("The value of inner scope is {x}");
    }
    println!("Outside is {x}");
}
```
In this snippet we just use `let` to shadow, and this is the output, so it maintains the value until its shadowed again, we get:

```sh
The value of inner scope is 12
Outside is 6
```

Note that shadowing is different from having a mutable variable, if we try to reassign a immutable variable we get a compile error, but if we re-use let to shadow we'll just be fine and have this variable me immutable as we shadow.

Also when shadowing we chan change the type of the variable, for example these two lines:

```rust
let spaces = "    "
let spaces = spaces.len();
```

`spaces` is first a string and then when we compute its length we get an integer, even if we defined this variable as `let mut spaces = "      "`; and later did `mut = spaces.len();` because we are not shadowing, it'd error.

### Data Types

In rust each variable has a type, we will look into two data type subsets: scalar and compound.

Because rust is a statically typed language it means needs to knwo all types at compile time, usually the compiler is able to infer it but we can also define the type annotation when the type is not clear, e.g:

```rust
let guess: u32 = "42".parse().expect("not a number");
```

If we don't the compiler will complain with `type annotations needed`.

#### Scalar Types

A scalar type represents a single value, it has four types.

##### Integer Types

A number without a fractional component, for example `u32` designates that it should be unsigned integer (signed starts with `i`) and takes up to 32 bits:

| Length | Signed | Unsigned |
|---|---|---|
| 8-bit | i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| 128-bit | i128 | u128 |
| Architecture-dependent | isize | usize |


Each variant of integers can be signed/unsigned and has a size; signed means if we need a sign (so it can be negative) or not, unsigned if we know it will only be positive.

For the same number of bits, that define as $2^{n-1}$,  let us know the values we can store, they both store the same amount of numbers but if the value can be negative, the range of values halves.

There's also the types of `isize` and `usize` that are let to be defined by the architecture of the computer, depending if its 64 or 32 bits.

You can also define integers in different literal:

| Number literal | Example |
|---|---|
| Decimal | 98_222 |
| Hex | 0xff |
| Octal | 0o77 |
| Binary | 0b1111_0000 |
| Byte (u8 only) | b'A' |

And also, like python, use the `_` underscore to split `1_000_000`.

So how do we define numbers? As integers? or as hex? Usualy we tend to then use rust defaults, that for example, for numbers its `i32`. The main use case for `isize` or `usize` is when indexing some sort of collection.

A note on integer overflows, if we had an `u8`, the `256 (0 to 255)` values  if this variable at some point changes to a too high of a value, two things could happen:

- When compiling rust includes checks for integer overflow  that cause a panic at runtime.

- WHen compiling with the `--release` flags it does not check for overflow panics, instead if an overflow happens, rust performs two complement wrapping: Values grearter than the maximum value it can hold the type does a wrap waround to the minimumu of the value the type can hold, so `256` becomes a `0` and `257` to `1`, it won't panic but this might lead to unexpected behaviour.

To handle best the possibility of overflow there are some methods provided by the standard library:

- Wrap all in compliation modes with the `wrapping_*`, such as `wrapping_add`.
- Return the None value if the overflow is with the `checked_*` methods.
- Return the value and a Boolean indicating whether there was an overflow with `overflowing_*` methods.
- Saturate the valuess minimumu or maximum values with `staurating_*` methods.


##### Floating-Point Types

Floating types can be tricky, rust offers two types of decimal points, `f32` and `f64` for its syze. The default is `f64` as most modens of CPU's handle it and for the same speed (almost) we get much more precission.

These can be defiend as:

```rust
fn main() {
    let x = 2.0; // f64 the default
    let y: f32 = 3.0; // f32 explicit
}
```

##### Numeric Operations

Rust supports basic math operations, there are some caveats: Integer division truncates toward 0 to the nearest integer, for example:


```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
```


##### Boolean Type

Boolean are either `true` or `false` and are the type `bool`:


```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

##### Character Type

Most primitive alphabetical type:


```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

It's a single quote, string literals are double. By default the `char` type is `4` bytes and represnts a unicode scalar value, it can show much more than just `ASCII`

##### Compound Types

###### Tuples

A way to group values with a varity of types into one compound type, its immutable in size (not values), once declared can't grow or shrink.
Similar to Python we can destructinct (unpack in Python?) values:

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```

We can also access its elements by a dot and its index:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

An empty tuple `()` its called unit, and its the default return value of expressions.


###### Arrays

We can also pack multiple values in a single compound type on arrays, but it must have then the same type and a fixed lenght (in Rust); they are also declared as a list in Python:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

Arrays are useful when you want your data allocated on the stack, rather than the heap (like the other types seen so far), or you want to ensure a fix length.
They are simialr to vectors but not so flexible, vector is a collection of the standard type that can grow on size and live on heap (will be discussed further).

Arrays then are better when we know the number of elements will not need to change, e.g months of the year.

We can define type and size:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Or define the length of an array and popualte with the same value:

```rust
let a = [3; 5];
```

We can access arrays elements, given that an array ins a single chunk of memory of known size it can be allocated on the stsack, and we acess by index (this time like Python) using brackets:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

We can at some point access an invalid position in the array:

```rust
use std::io; 

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Add index of array");

    let mut idx = String::new();
    io::stdin().read_line(&mut idx).expect("Failed");
    let idx: usize = idx.trim().parse().expect("cant parse");
    let element = a[idx];

    println!("Value of {idx} is {element}");
}
```

If we go out of boudns of the array size we get, although the code compiled:

```sh
Add index of array
10

thread 'main' (2492307) panicked at src/main.rs:24:19:
index out of bounds: the len is 5 but the index is 10
```

So we get this runtime error, in some languages we'd just get access another mem cell (specially low level) but rust checks this.

### Functions

Defined as snake case, other than main we can define more and call them:

```rust
fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("This is another one");
}

```

This outputs:

```sh
Hello, world!
This is another one
```

#### Parameters

We can pass data toa function using its parameters, parameters are also variables that belong to the function's signature.
The concrete values of the function are called arguments, althoguh it can be mixed often.

We add a parameter `x` to the function `another_function` and pass the argument `5`:

```rust
fn main() {
    println!("Hello, world!");
    another_function(5);
}

fn another_function(x: i32) {
    println!("Parameter x has value {x}");
}

```

So we get:

```sh
Hello, world!
Parameter x has value 5
```

We must define the type of each parrameter, so compiler can catch errors early.


#### Statements and Expressions

Function bodies are made up of a series of statments and sometimes end up in an optional expression (rust is an expression-based language), the difference between them is:

- Statements are instructions that perform some action and do not return a value.
- Expressions evaluate to a resultant value

For example a simple statement is creating a variable and assigning a value:

```rust
let y = 6;
```
Function definitions per-se are also statments, and because statements return no values you can't do this:

```rust
let x = (let y = 6);
```

We get an expected expression, found `let` statement, this works in some languages like `C` where you can do stuff like `x = y = 6`.

Expressions evaluat to a value, such as `5 + 6` which returns(evaluates) to 11, actually in the previous staement `let y = 6`, the `6` is an actual expression that evaluates to just `6`.
Calling a function or a macro is also an expression, a block of code with curkly brackets is also an expression:

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {y}");
}
```

The smal block  after `y` is an expression that evaluates to 4, and then its assigned to `y`.

Important to notice that the last `x + 1` of the block has no semicolon at the end, this is because expressions do not include ending semicolons, if you add it its an statement (assume its a Python return);

#### Functions with Return Values

Functions can return values to the code it calls them, and we need to declare its type in teh function. By default the return value of the function is the final expression in the body block, but you can always return a value early using the keyword `return`.

```rust
fn fine() -> i32 {
    5 // no return needed or semicolon
}
```


### Comments

How comments work, its `//`, multiple line, after end of line, above or whatever.

### Control Flow

Controling code behaviour based on boolean logic

#### if Expressions

Branch the code based on a condition, syntax is:

```rust
if {}
else if {}
else {}
```

Conditional must be always evaluated to boolean (no falsy values like any int > 0 is true like Python)

We can also use inline `if` when `let` statement to set a variable base don some logic, something like this:

```rust
let condition = true
let number = if condition { 5 } else { 6 };
```

Remember anything being evaluated in a block being an expression? Here is why we need to wrap (and no semicollon above), but we should see a `5` being printed out.

Similar, also we are forced to have if/else have the same types on an inline declaration of a variable, this is an issue:

```rust
let number = if condition { 5 } else { "six" };
```

#### Repetition with Loops

##### Repeaeting code with loop

`loop` tells rust to execute a block of code forever until explicitelly told to stop (`while:` in Python):

```rust
fn main() {
    loop {
        println!("again");
    }
}
```

This will run until we `control+C`, we can leave from the code using `break`. There's also the keyword (like Python) `continue`, which stops the current iteration to go to start again.

##### Returning values from loops

It could be that at some point we want to check some assumption and have the value from inside the loop be passed somewhere, we can do this by doing `break` and the expression we want to return:

```rust
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }
}
```

You could also always leave using `return`, `break` leaves the current loop but `return` the entire function.


##### Disambiguating with loop labels

Whenever we have nested loops (dont like having nested loops) the `break` or `continue` applies to the innermost loop we are, we can specify labels to loops so we can apply these keywords to the labeled loop, not the innermost, see:

```rust
fn loop_label() {
    let mut count = 0;
    'counting_loop: loop {
        println!("count  = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // current unlabeled loop
            }
            if count == 2 {
                break 'counting_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count at {count}");
}
```

Basically when we finally reach `2` we leave the outter loop and end execution on main body, othewise we keep doing loops on the 9/10 prints:

```sh
count  = 0
remaining = 10
remaining = 9
count  = 1
remaining = 10
remaining = 9
count  = 2
remaining = 10
End count at 2
```

##### Streamlining Condition loops with a while

Evaluating conditions in the loop (runs until evaluation is not True):

```rust
fn main () {
    let mut number = 3;

    while number != 0 {
        println!("We at {number}");
    }
    println!("Done");
}
```


##### Looping Through a Collection with for

We could print all numbers from 1 to 4 with such a loop:

```rust
fn while_cnt() {
    let mut index = 1;
    while index < 5 {
        println!("INdex is {index}");
        index += 1;
    }
}
```
Which is fine, but this could always be prone to bugs if index is not updated correctly, if we wanted to iterate to a known set of values we'd be asfer using a `for`:

```rust
fn main() {
    let a = [1, 2, 3, 4];
    for element in a {
        println!("We in {a}");
    }
}
```
Or even have the loop iterate directly over a range something like `for number in (1..4) `.


## Things I Found Interesting

N/A

## Questions / Things to Revisit

- Heap vs. stack allocation

  **Explanation:** The stack stores fixed-size data in function frames and cleans it up as those functions return. The heap is used for data whose size can vary at runtime; a `String` or `Vec` keeps a small descriptor (such as a pointer and length) while its contents live on the heap. A useful first approximation is stack = fixed-size, automatic storage; heap = dynamically allocated storage. ([The Rust Book: The Stack and the Heap](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap))

- I still struggle to distinguish statements from expressions.

  **Explanation:** A statement performs an action but does not produce a value, such as `let y = 6;`. An expression evaluates to a value, such as `6`, `x + 1`, or a function call. A block can also produce a value: its final expression, written without a semicolon, becomes the block’s result. Adding a semicolon discards that result.
## Exercises & Code

N/A

## Progress

- [x] Read the chapter
- [x] Complete the exercises
- [x] Write my own notes
- [x] Review unclear concepts

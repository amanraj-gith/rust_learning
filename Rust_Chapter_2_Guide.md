# Rust Chapter 2 — Programming a Guessing Game

> **Goal:** Build a small game where the computer picks a random number (1–100) and the user tries to guess it. The program tells them if the guess is too high, too low, or correct.

> **New Rust concepts you will meet:** `use`, `let`, `mut`, references (`&`), `Result`, `match`, external crates, shadowing, `loop`, `break`, `continue`.

---

## 1. What This Chapter is About

You will build a **guessing game** step by step. Each step introduces one new idea:

| Step | New Idea |
|------|----------|
| Take user input | `use`, `let mut`, `String::new()` |
| Handle errors | `Result`, `.expect()` |
| Generate random number | External crate (`rand`) |
| Compare values | `match`, `Ordering` enum |
| Convert string → number | `.trim()`, `.parse()`, shadowing |
| Play again and again | `loop`, `break`, `continue` |

Think of it as:

> "One tiny program, six Rust concepts."

---

## 2. Create the Project

Move to your projects folder, then:

```
cargo new guessing_game
cd guessing_game
```

Cargo creates:

```
guessing_game/
│
├── Cargo.toml
└── src/
   └── main.rs
```

Run the starter "Hello, world!" to check everything works:

```
cargo run
```

---

## 3. Cargo.toml — What's Inside

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"

[dependencies]
```

| Section | Purpose |
|---------|---------|
| `[package]` | Info about your project (name, version) |
| `[dependencies]` | External libraries you want to use |

---

## 4. First Version — Take a Guess from the User

Edit `src/main.rs`:

```rust
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

Run it:

```
cargo run
```

Type a number, press Enter. The program prints it back.

---

## 5. Understand the Code — Line by Line

### 5.1 `use std::io;`

Rust has a **standard library** (`std`). It has many modules.

By default only a small set (called the **prelude**) is auto-imported. Everything else must be brought in with `use`.

```rust
use std::io;   // now we can use io::stdin() etc.
```

Think of it as:

> "Bring the `io` toolbox into the room before I use its tools."

---

### 5.2 `let mut guess = String::new();`

| Piece | Meaning |
|-------|---------|
| `let` | Create a new variable |
| `mut` | Make it changeable |
| `guess` | Name of the variable |
| `String::new()` | Call the `new` function of `String` type → gives an empty String |

Important rules:

- ✅ Variables in Rust are **immutable by default**.
- ❌ You cannot change an immutable variable.
- ✅ Add `mut` to make it mutable.

Example:

```rust
let apples = 5;        // immutable — cannot change
let mut bananas = 5;   // mutable — can change later
```

**Associated function:** `String::new()` — a function attached to the `String` type. `::` means "belongs to".

---

### 5.3 `io::stdin().read_line(&mut guess)`

| Piece | Meaning |
|-------|---------|
| `io::stdin()` | Get a handle to the keyboard input |
| `.read_line(...)` | Read one line typed by the user |
| `&mut guess` | Pass `guess` **by reference**, and allow changing it |

**Why `&`?** References let multiple parts of the code look at the same value without copying it.

**Why `&mut`?** References are also immutable by default. `&mut` says "I want to be able to change this value through this reference."

---

### 5.4 `.expect("Failed to read line")`

`read_line` does not just fill up `guess` — it also returns a **Result**.

`Result` is an **enum** (a type with multiple states). Its two states are:

| Variant | Meaning |
|---------|---------|
| `Ok(value)` | Success — here's the value |
| `Err(info)` | Failure — here's what went wrong |

`.expect("...")` says:

- ✅ If Result is `Ok`, give me the inner value
- ❌ If Result is `Err`, crash the program and print my message

Think of it as:

> "This might fail. If it does, panic with this message."

If you skip `.expect()`, the compiler warns you that you didn't handle the possible error.

---

### 5.5 `println!("You guessed: {guess}");`

The `{}` is a **placeholder**. Rust fills it with the value of `guess`.

Two ways to use it:

```rust
let x = 5;
let y = 10;

println!("x = {x}");                        // variable name inside braces
println!("y + 2 = {}", y + 2);              // empty braces + value after comma
```

Think of `{}` as:

> "Little crab pincers holding a value in place."

---

## 6. Test the First Part

```
cargo run
Guess the number!
Please input your guess.
7
You guessed: 7
```

At this point: input works, output works. Time to add the actual game.

---

## 7. Generate a Secret Number — Using a Crate

Rust's standard library does **not** include random numbers. We use an external library called `rand`.

A **crate** is a package of Rust code.

| Crate Type | Meaning |
|------------|---------|
| Binary crate | An executable program (your `guessing_game`) |
| Library crate | Reusable code for other programs (like `rand`) |

---

### 7.1 Add `rand` to Cargo.toml

Under `[dependencies]`:

```toml
[dependencies]
rand = "0.8.5"
```

Now run:

```
cargo build
```

Cargo will:

```
Read Cargo.toml
     ↓
Download rand and its dependencies from crates.io
     ↓
Compile them
     ↓
Compile your project
```

---

### 7.2 Cargo.lock — Reproducible Builds

The first time you build, Cargo creates a file called `Cargo.lock`.

| File | Purpose |
|------|---------|
| `Cargo.toml` | What versions you **want** |
| `Cargo.lock` | What versions you **actually got** |

Next time you build, Cargo uses the versions from `Cargo.lock` — so your build is the same every time.

- ✅ Commit `Cargo.lock` to Git for binary projects
- ✅ Makes builds reproducible for everyone

---

### 7.3 Update a Crate

To pull in newer patch versions allowed by `Cargo.toml`:

```
cargo update
```

Cargo will find the latest version between `0.8.5` and `0.9.0` (based on your `= "0.8.5"` rule) and update `Cargo.lock`.

For a bigger version jump (e.g. `0.9.x`), edit `Cargo.toml` yourself.

---

## 8. Use `rand` in the Code

```rust
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

| Line | Meaning |
|------|---------|
| `use rand::Rng;` | Bring the `Rng` **trait** into scope (methods like `gen_range`) |
| `rand::thread_rng()` | Get a random number generator local to this thread |
| `.gen_range(1..=100)` | Pick a number between 1 and 100 (inclusive) |

**Range syntax:** `1..=100` means `1` to `100`, both included.

> **Tip:** Run `cargo doc --open` to see docs for **all** your dependencies in your browser.
> there is 2 things - Standard Library under which there are 2 things called prelude which are already imprted and others which we have to import like 'std::io' for input and External Library (Crates) under which there is trait and using that we can use methods like 'get_range'
---

## 9. Compare the Guess to the Secret Number

Add `Ordering` and a `match`:

```rust
use std::cmp::Ordering;

// ... inside main, after println!("You guessed: {guess}");

match guess.cmp(&secret_number) {
    Ordering::Less    => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal   => println!("You win!"),
}
```

### 9.1 `Ordering` — Another Enum

| Variant | Meaning |
|---------|---------|
| `Less` | left < right |
| `Greater` | left > right |
| `Equal` | left == right |

### 9.2 `match` — Pattern Matching

A `match` block has **arms**. Each arm has:

- A **pattern** on the left of `=>`
- Code on the right

Rust checks each arm top to bottom. The first matching arm runs.

Think of it as:

> "Which of these boxes fits the value I just got? Do the thing in that box."

---

## 10. Fix the Type Mismatch

The above code will **not compile** yet. Error:

```
expected `&String`, found `&{integer}`
```

Reason: `guess` is a `String`, `secret_number` is a number. You can't compare them directly.

### 10.1 Convert String → Number (Shadowing + parse)

Add this line **before** the `match`:

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

| Piece | Meaning |
|-------|---------|
| `let guess: u32` | Create a new `guess`, type is `u32` (unsigned 32-bit int) |
| `guess.trim()` | Remove whitespace / newline (`\n` or `\r\n`) at the ends |
| `.parse()` | Convert the string to a number → returns a `Result` |
| `.expect("...")` | Crash if the string is not a valid number |

### 10.2 Shadowing

We already had a variable named `guess` (a `String`). This new `let guess` **shadows** the old one.

- ✅ You can reuse a name
- ✅ Very common when converting one type into another
- Different from `mut` — shadowing creates a **new** variable

Example:

```rust
let guess = "42";         // String
let guess: u32 = 42;      // shadowed with a u32 — same name, new variable
```

---

## 11. Add a Loop for Multiple Guesses

Wrap the guessing part in `loop { ... }`:

```rust
loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal   => println!("You win!"),
    }
}
```

Problem: this loop never ends! Ctrl+C is the only way out. Fix next.

---

## 12. Quit After Winning — `break`

Change the `Equal` arm to also **break** the loop:

```rust
Ordering::Equal => {
    println!("You win!");
    break;
}
```

| Keyword | Purpose |
|---------|---------|
| `break` | Exit the loop immediately |
| `continue` | Skip to the next iteration of the loop |

---

## 13. Handle Bad Input — Don't Crash

Right now, if the user types `abc`, `.expect()` crashes the game.

Better: **skip** bad input and ask again. Replace the parse line with a `match`:

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num)  => num,
    Err(_)   => continue,
};
```

| Arm | Meaning |
|-----|---------|
| `Ok(num)` | Parse worked — use the number |
| `Err(_)` | Parse failed — skip this round with `continue` |

**The underscore `_`:** a catch-all pattern. "Match anything, I don't care what's inside."

Think of it as:

> "Ok? Take the number. Err? Try again."

---

## 14. Final Code

```rust
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num)  => num,
            Err(_)   => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
```

Notice we also **removed** the line that printed the secret number — otherwise the game gives itself away.

---

## 15. Quick Revision

### Bring a module in
```rust
use std::io;
```

### Immutable vs mutable variable
```rust
let x = 5;         // immutable
let mut y = 5;     // mutable
```

### Create a String
```rust
let mut s = String::new();
```

### Read a line
```rust
io::stdin().read_line(&mut s).expect("Failed to read line");
```

### Add a dependency
```toml
[dependencies]
rand = "0.8.5"
```

### Random number 1–100
```rust
let n = rand::thread_rng().gen_range(1..=100);
```

### Compare two values
```rust
match a.cmp(&b) {
    Ordering::Less    => ...,
    Ordering::Greater => ...,
    Ordering::Equal   => ...,
}
```

### Convert String → number, skip errors
```rust
let n: u32 = match s.trim().parse() {
    Ok(n)  => n,
    Err(_) => continue,
};
```

### Loop control
```rust
loop {
    ...
    if done { break; }
    if skip { continue; }
}
```

---

## 16. Mental Model

```
User Types a Number
        ↓
   String (guess)
        ↓
    trim() + parse()
        ↓
   u32 (guess)
        ↓
   compare with secret_number
        ↓
   match Ordering → Less / Greater / Equal
        ↓
   Equal? → break
   else  → loop again
```

And the "big picture" of new ideas from this chapter:

```
use          →  bring modules / traits into scope
let / mut    →  create variables, control mutability
&, &mut      →  share data without copying
Result       →  handle "might fail" cases
match        →  branch on the value's shape
crate        →  reuse other people's code
shadowing    →  reuse a name for a new value
loop / break →  repeat until done
```

---

*Chapter 2 done. You have written a small game that uses variables, references, error handling, external crates, pattern matching, and loops — the same tools you will use for the rest of the book.*

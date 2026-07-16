# Rust Chapter 3 — Common Programming Concepts

> **Goal:** Learn the basic bricks every Rust program is built from — variables, types, functions, comments, and control flow — so you have a solid base before ownership.

> **New Rust concepts you will meet:** `let`, `mut`, `const`, shadowing, static typing, scalar types, integer overflow, tuples, arrays, `fn`, parameters, statements vs expressions, return values, `//`, `if` / `else`, `loop`, `break`, `continue`, loop labels, `while`, `for`, ranges.

## 1. Variables Are Immutable by Default

In Rust, once you bind a value to a name with `let`, that name **cannot** be reassigned.

Example:

```rust
fn main() {
    let x = 5;
    x = 6; // ❌ compile error: cannot assign twice to immutable variable `x`
}
```

Line-by-line:

| Piece | Meaning |
| --- | --- |
| `let` | Introduces a new variable binding. |
| `x = 5` | Binds the value `5` to the name `x`. |
| `x = 6` | Tries to change what `x` points to — not allowed. |

> Think of it as:
> "A `let` is like writing a name on stone. Once carved, you can't rewrite it."

Why it's a default:

- ✅ Compiler catches unwanted changes for you.
- ✅ Other parts of the code can safely assume the value stays the same.
- ❌ You lose flexibility, but only when you need it — and Rust lets you opt out.

## 2. Making Variables Mutable with `mut`

Add `mut` before the name to allow reassignment.

Example:

```rust
let mut x = 5;
x = 6; // ✅ works
```

| Piece | Meaning |
| --- | --- |
| `mut` | "This binding will change later." |
| `x = 6` | Now legal — `x` was declared mutable. |

`mut` also **tells future readers** the value is expected to change. It is a signal, not just a switch.

## 3. Constants with `const`

Constants are values that never change, and are different from `let` bindings.

Example:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

| Rule | Detail |
| --- | --- |
| Keyword | `const` (not `let`). |
| `mut`? | ❌ Never allowed. |
| Type annotation | Required. Always. |
| Value | Must be known at compile time — no runtime values. |
| Scope | Any scope, including global. |
| Naming | `SCREAMING_SNAKE_CASE`. |
| Lifetime | Lives for the whole program in its scope. |

> Think of it as:
> "A `const` is a fact printed in the manual. Anyone anywhere in the program can rely on it, and it never changes."

## 4. Shadowing

You can declare a new variable with the **same name** as an old one using `let` again. The new one **shadows** the old.

Example:

```rust
fn main() {
    let x = 5;
    let x = x + 1;      // x is now 6

    {
        let x = x * 2;  // inner x is 12
        println!("inner: {x}");
    }

    println!("outer: {x}"); // back to 6
}
```

Shadowing vs `mut`:

| Feature | `mut` | Shadowing |
| --- | --- | --- |
| Keyword to reuse | none (just `x = ...`) | `let x = ...` |
| Can change type? | ❌ No | ✅ Yes |
| Creates new binding? | ❌ No | ✅ Yes |
| Final binding immutable? | ❌ No | ✅ Yes |

Example — changing type via shadowing:

```rust
let spaces = "   ";      // &str
let spaces = spaces.len(); // usize — legal, new binding
```

> Think of it as:
> "`mut` edits the box. Shadowing throws the old box away and puts a new one in its place under the same label."

## 5. Rust Is Statically Typed

Every value has a type, and the compiler must know all types at compile time.

Rust often infers the type. When it can't, you must annotate.

Example — annotation needed:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

| Piece | Meaning |
| --- | --- |
| `: u32` | Tells Rust to treat the parsed value as a 32-bit unsigned integer. |
| `.parse()` | Could return many number types, so the annotation removes ambiguity. |

## 6. Scalar Type — Integers

Integers are whole numbers. Rust has signed (`i`) and unsigned (`u`) variants at different bit sizes.

| Length | Signed | Unsigned |
| --- | --- | --- |
| 8-bit | `i8` | `u8` |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` | `u32` |
| 64-bit | `i64` | `u64` |
| 128-bit | `i128` | `u128` |
| arch | `isize` | `usize` |

Ranges:

| Type | Range |
| --- | --- |
| Signed `iN` | −(2^(N−1)) to 2^(N−1) − 1 |
| Unsigned `uN` | 0 to 2^N − 1 |

Default when unsure → `i32`. Use `usize` / `isize` mostly for **indexing collections**.

Integer literals:

| Form | Example |
| --- | --- |
| Decimal | `98_222` |
| Hex | `0xff` |
| Octal | `0o77` |
| Binary | `0b1111_0000` |
| Byte (`u8` only) | `b'A'` |

`_` is a visual separator: `1_000` == `1000`. You can also type-suffix: `57u8`.

## 7. Integer Overflow

What happens if a `u8` (max 255) gets assigned 256?

| Build mode | Behaviour |
| --- | --- |
| Debug (`cargo run`) | Program **panics** at runtime. |
| Release (`--release`) | **Two's complement wrap** — 256 becomes 0, 257 becomes 1. |

Relying on wrap is a bug. If you actually need controlled overflow, use these method families:

| Method family | What it does |
| --- | --- |
| `wrapping_*` | Always wraps around. |
| `checked_*` | Returns `None` on overflow. |
| `overflowing_*` | Returns `(value, bool)` — bool = did it overflow. |
| `saturating_*` | Sticks at min or max value. |

## 8. Floats, Booleans, and Characters

**Floats** — numbers with decimals.

| Type | Size | Default? |
| --- | --- | --- |
| `f32` | 32-bit | no |
| `f64` | 64-bit | ✅ yes |

Example:

```rust
let x = 2.0;        // f64
let y: f32 = 3.0;   // f32
```

**Booleans** — one byte, `true` or `false`.

Example:

```rust
let t = true;
let f: bool = false;
```

**Characters** — `char`, 4 bytes, one Unicode scalar value. Single quotes.

Example:

```rust
let c = 'z';
let z: char = 'ℤ';
let heart_eyed_cat = '😻';
```

| Detail | Notes |
| --- | --- |
| Quotes | `'x'` for `char`, `"x"` for string. |
| Range | `U+0000`–`U+D7FF` and `U+E000`–`U+10FFFF`. |
| Not the same as | A human "character" — that comes later in Ch. 8. |

## 9. Numeric Operations

Rust supports the standard math operators on all number types.

Example:

```rust
let sum = 5 + 10;
let difference = 95.5 - 4.3;
let product = 4 * 30;
let quotient = 56.7 / 32.2;
let truncated = -5 / 3;   // = -1 (integer division truncates toward zero)
let remainder = 43 % 5;
```

| Operator | Name |
| --- | --- |
| `+` | Addition |
| `-` | Subtraction |
| `*` | Multiplication |
| `/` | Division (integer division truncates) |
| `%` | Remainder |

## 10. Compound Type — Tuples

A tuple groups **multiple values of possibly different types** into one thing. Fixed length.

Example:

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

Destructuring — split it into named parts:

```rust
let (x, y, z) = tup;
println!("y = {y}"); // 6.4
```

Index access — with a dot and the position:

```rust
let five_hundred = tup.0;
let six_point_four = tup.1;
let one = tup.2;
```

The empty tuple `()` is called **unit** — it means "no value" and is what expressions return when they don't return anything else.

> Think of it as:
> "A tuple is a small paper bag with numbered slots. Each slot can hold a different kind of thing, but the bag never grows."

## 11. Compound Type — Arrays

An array is a **fixed-length list of values of the same type**, stored on the stack.

Example:

```rust
let a = [1, 2, 3, 4, 5];
let months = ["Jan", "Feb", "Mar"];       // inferred
let b: [i32; 5] = [1, 2, 3, 4, 5];        // explicit type + length
let c = [3; 5];                            // = [3, 3, 3, 3, 3]
```

| Syntax | Meaning |
| --- | --- |
| `[T; N]` | Array of `N` items of type `T`. |
| `[value; N]` | Array of `N` copies of `value`. |
| `a[i]` | Access element at index `i` (0-based). |

Array vs Vector:

| Array | Vector |
| --- | --- |
| Fixed length | Can grow / shrink |
| Stack | Heap |
| Use when count is known & fixed | Default choice when unsure |

**Invalid index → panic at runtime.**

```rust
let a = [1, 2, 3, 4, 5];
let el = a[10]; // ❌ panic: index out of bounds
```

Rust checks the index at runtime and exits safely instead of reading random memory. This is memory safety in action.

## 12. Defining Functions & Parameters

Functions are declared with `fn`. Naming convention is `snake_case`.

Example:

```rust
fn main() {
    println!("Hello, world!");
    another_function(5, 'h');
}

fn another_function(value: i32, unit_label: char) {
    println!("{value}{unit_label}");
}
```

| Piece | Meaning |
| --- | --- |
| `fn` | Declares a function. |
| `another_function` | Function name in snake_case. |
| `(value: i32, unit_label: char)` | Parameter list — **types are required**. |
| `{ ... }` | Function body. |

Notes:

- Order in the file doesn't matter — Rust finds functions in the enclosing scope.
- Function signatures **must** annotate every parameter's type. This is a language design choice, not laziness on the compiler's part.

**Parameter vs argument:**

| Term | Meaning |
| --- | --- |
| Parameter | The named slot in the function definition. |
| Argument | The concrete value passed when calling. |

## 13. Statements vs Expressions

Rust is **expression-based**. This matters.

| Kind | Description | Returns a value? |
| --- | --- | --- |
| Statement | Does something. | ❌ No |
| Expression | Evaluates to a value. | ✅ Yes |

Statements:

- `let y = 6;` → statement (binds a value, returns nothing).
- A function definition → statement.

Expressions:

- `5 + 6` → evaluates to `11`.
- A function call → expression.
- A macro call → expression.
- A `{ ... }` block → expression, if it ends in a value.

Example — a block used as an expression:

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1     // no semicolon — this is the block's value
    };
    println!("{y}"); // 4
}
```

| Line | Meaning |
| --- | --- |
| `let x = 3;` | Statement inside the block. |
| `x + 1` | Expression — the block's final value. |
| No `;` on last line | Keeps it an expression. Adding `;` would make it a statement returning `()`. |

> Think of it as:
> "A semicolon at the end of a line throws its value away. No semicolon on the last line = keep the value."

Cannot do this:

```rust
let x = (let y = 6); // ❌ let is a statement, not an expression
```

## 14. Return Values with `->`

Return type is declared after `->`. The **last expression** in the body is what gets returned. You may also `return` early.

Example:

```rust
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

| Piece | Meaning |
| --- | --- |
| `-> i32` | The function returns an `i32`. |
| `5` (no `;`) | The value returned. |
| `x + 1` (no `;`) | Same idea — expression, returned. |

Adding a semicolon breaks it:

```rust
fn plus_one(x: i32) -> i32 {
    x + 1; // ❌ mismatched types: expected i32, found ()
}
```

Because `x + 1;` becomes a statement and produces `()`, but the signature promised `i32`.

## 15. Comments

Comments start with `//` and run to the end of the line.

Example:

```rust
// a single-line comment

// A longer thought that
// needs several lines
// gets `//` on each one.

let lucky = 7; // end-of-line comment
```

Idiomatic style: put comments on the line **above** the code they describe.

Doc comments (`///`) are covered later in Ch. 14.

## 16. `if` / `else` Expressions

Branch code based on a `bool` condition.

Example:

```rust
let number = 3;

if number < 5 {
    println!("true");
} else {
    println!("false");
}
```

Rules:

| Rule | Notes |
| --- | --- |
| Condition must be `bool` | ❌ `if number` (with an integer) fails. No auto-conversion. |
| Blocks called "arms" | Same word used later with `match`. |
| `else` is optional | If missing, block is simply skipped when false. |

`else if` for multiple conditions:

```rust
if n % 4 == 0 {
    // ...
} else if n % 3 == 0 {
    // ...
} else {
    // ...
}
```

The **first** `true` branch runs; the rest are skipped. Many `else if`s → consider `match` (Ch. 6).

## 17. `if` as an Expression

Since `if` is an expression, you can put it on the right side of `let`.

Example:

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
```

| Piece | Meaning |
| --- | --- |
| `if condition { 5 }` | Arm produces `5`. |
| `else { 6 }` | Arm produces `6`. |
| Both arms | **Must have the same type.** |

Mismatch → compile error:

```rust
let n = if condition { 5 } else { "six" }; // ❌ i32 vs &str
```

## 18. `loop`, `break`, `continue`

`loop` runs forever until you break out.

Example — infinite loop:

```rust
loop {
    println!("again!");
}
```

Stop it manually with `Ctrl-C`, or with `break` in code.

| Keyword | What it does |
| --- | --- |
| `break` | Exits the (innermost) loop. |
| `continue` | Skips to the next iteration. |
| `return` | Exits the whole function (not just the loop). |

Returning a value from a `loop`:

```rust
let mut counter = 0;

let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;   // returns 20
    }
};
```

| Piece | Meaning |
| --- | --- |
| `let result = loop { ... };` | The whole loop is an expression, and its value is bound to `result`. |
| `break counter * 2` | Break AND return `20` as the loop's value. |
| `;` after `}` | Ends the enclosing `let` statement. |

## 19. Loop Labels

With nested loops, `break` and `continue` affect the **innermost** loop by default. Labels let you target an outer one.

Example:

```rust
'counting_up: loop {
    loop {
        break;             // exits inner loop only
        break 'counting_up; // exits the outer loop
    }
}
```

| Piece | Meaning |
| --- | --- |
| `'name:` | A loop label. Must start with a single quote. |
| `break 'name;` | Break out of the loop with that label. |
| `continue 'name;` | Skip to the next iteration of that labeled loop. |

## 20. `while` Loops

Runs the body while the condition stays `true`. Cleaner than `loop` + `if` + `break`.

Example — countdown:

```rust
let mut number = 3;

while number != 0 {
    println!("{number}!");
    number -= 1;
}
println!("LIFTOFF!!!");
```

Flow:

```
check condition
     ↓
 run body
     ↓
check again
     ↓
 stop when false
```

## 21. `for` Loops and Ranges

`for` iterates over each item of a collection. Safer and often faster than a hand-written `while` with an index.

Example — over an array:

```rust
let a = [10, 20, 30, 40, 50];

for element in a {
    println!("{element}");
}
```

Example — over a range, in reverse:

```rust
for number in (1..4).rev() {
    println!("{number}!");
}
println!("LIFTOFF!!!");
```

| Piece | Meaning |
| --- | --- |
| `1..4` | Range `1, 2, 3` (end is exclusive). |
| `.rev()` | Reverses it: `3, 2, 1`. |
| `for x in ...` | Bind each item to `x` and run the body. |

Why prefer `for`:

- ✅ No off-by-one bugs — bounds are handled for you.
- ✅ No runtime bounds check on each step → often faster.
- ✅ You don't need to update loop conditions when the collection changes size.

> Think of it as:
> "`for` is `while` with the counter, the bounds check, and the mistakes removed."

## Quick Revision

### Variables

```rust
let x = 5;                // immutable
let mut y = 5;            // mutable
const MAX: u32 = 100_000; // constant, always typed
```

### Shadowing

```rust
let x = 5;
let x = x + 1;      // new binding
let x = "now text"; // type may change
```

### Scalar types

```rust
let a: i32 = -1;
let b: u8 = 255;
let c: f64 = 3.14;
let d: bool = true;
let e: char = '😻';
```

### Compound types

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;
let first = tup.0;

let arr: [i32; 3] = [1, 2, 3];
let same = [0; 5];        // [0,0,0,0,0]
let el = arr[0];
```

### Functions

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b   // no semicolon = returned
}
```

### Comments

```rust
// single line
```

### if / else

```rust
if x < 5 { ... } else if x < 10 { ... } else { ... }
let n = if cond { 5 } else { 6 };
```

### Loops

```rust
loop { break; }
let v = loop { break 42; };

'outer: loop {
    loop { break 'outer; }
}

while n != 0 { n -= 1; }

for x in [10, 20, 30] { println!("{x}"); }
for i in (1..4).rev() { println!("{i}"); }
```

### Integer overflow helpers

```rust
n.wrapping_add(1);
n.checked_add(1);      // Option
n.overflowing_add(1);  // (value, bool)
n.saturating_add(1);
```

## Mental Model

**How the pieces fit together at runtime:**

```
main() starts
     ↓
declare variables (let / let mut / const)
     ↓
call functions (with typed parameters)
     ↓
inside each fn:
   statements do work
   expressions produce values
     ↓
control flow (if / loop / while / for) picks which code runs
     ↓
final expression of fn body returns a value
     ↓
main() ends
```

**Concept cheat sheet:**

```
let              →  bind a value to a name (immutable by default)
mut              →  allow that binding to change
const            →  always-immutable, compile-time value with a type
shadowing        →  new `let` reuses a name, may change type
static typing    →  compiler needs every type known at compile time
scalar types     →  one value: integer, float, bool, char
compound types   →  many values in one: tuple (mixed) or array (same)
integer overflow →  panic in debug, wrap in release, or use *_add methods
fn               →  a named block of code; params typed, body of statements + optional expression
statement        →  does work, no value
expression       →  produces a value; no trailing `;`
->               →  declares the return type of a function
//               →  a comment; ignored by the compiler
if / else        →  branch on a bool; is itself an expression
loop             →  run forever until `break` (can return a value)
break / continue →  exit / skip; labels let you target an outer loop
while            →  run while a condition is true
for              →  run once per item in a collection or range
range (a..b)     →  numbers from a up to but not including b
```

*Chapter 3 done. You have the vocabulary — variables, types, functions, and control flow — to read and write real Rust code, and you're ready for the concept that makes Rust unique: ownership.*

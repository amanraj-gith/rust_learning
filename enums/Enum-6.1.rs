// =========================================================================
// RUST BOOK - SECTION 6.1: DEFINING AN ENUM (Explain Like I'm 5)
// =========================================================================

// THE BIG IDEA: STRUCTS vs ENUMS
// 🍔 A STRUCT is like a "Combo Meal". You get a Burger AND Fries AND a Drink.
// 🥤 An ENUM is like choosing your "Drink Flavor". It can be Coke OR Sprite OR Water.
// An Enum tells Rust: "This value can only be exactly ONE of these specific choices."

// -------------------------------------------------------------------------
// 1. THE SIMPLEST ENUM (Just making choices)
// -------------------------------------------------------------------------
// In the real world, computer IP addresses (like phone numbers for computers) 
// come in exactly two versions: V4 or V6. It can't be both at the same time.

enum IpAddrKind {
    V4, // Choice 1
    V6, // Choice 2
}

fn simple_enum_example() {
    // To pick a choice, we use two colons `::` (like looking inside a menu)
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}

// -------------------------------------------------------------------------
// 2. ENUMS THAT HOLD DATA (Putting presents inside boxes)
// -------------------------------------------------------------------------
// In other languages, you might have to make a Struct to hold the Enum AND the data.
// In Rust, Enums are basically MAGIC BOXES. You can put data directly inside them!
// Even better: Each choice can hold DIFFERENT types of data!

enum IpAddr {
    // V4 holds 4 little numbers (like 127, 0, 0, 1)
    V4(u8, u8, u8, u8),
    
    // V6 holds one big String of text (like "::1")
    V6(String),
}

fn data_enum_example() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}

// -------------------------------------------------------------------------
// 3. ENUMS CAN HOLD ANYTHING!
// -------------------------------------------------------------------------
// Let's say we are making a chat app. We can send different types of messages.
// Look at how flexible an Enum can be:

enum Message {
    Quit,                       // Holds nothing (Like a Unit Struct)
    Move { x: i32, y: i32 },    // Holds named data (Like a normal Struct)
    Write(String),              // Holds a single string (Like a Tuple Struct)
    ChangeColor(i32, i32, i32), // Holds three numbers (Like a Tuple Struct)
}

// Just like Structs, we can give Enums "methods" (functions) using `impl`!
impl Message {
    fn call(&self) {
        // We will learn how to use this inside chapter 6.2!
    }
}

fn method_enum_example() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

// -------------------------------------------------------------------------
// 4. THE MOST IMPORTANT ENUM IN RUST: `Option`
// -------------------------------------------------------------------------
// WHY IT EXISTS:
// In most languages, if a variable is empty, it is called "Null".
// But "Null" causes BILLIONS of dollars in software crashes because programs 
// accidentally try to use empty data as if it were real data.
// 
// Rust DOES NOT HAVE NULL. 
// Instead, it uses a built-in Enum called `Option`.

// Think of `Option` like a Mystery Gift Box:
// 🎁 Some(toy) -> The box has something inside it!
// 📭 None      -> The box is totally empty.

/* 
// (This is how it is written inside Rust's secret standard library):
enum Option<T> { 
    Some(T),  // 'T' just means "Any Type of data" (String, i32, etc.)
    None,     // Nothing is here!
}
*/

fn option_example() {
    // Because Option is so useful, you don't even have to type `Option::`.
    // You can just use `Some` or `None` directly!

    let some_number = Some(5);          // A box containing a number
    let some_string = Some("Aman");     // A box containing text
    
    // If we make an empty box, we MUST tell Rust what was SUPPOSED to be in it.
    let empty_number: Option<i32> = None; 

    // THE MAGIC SAFETY RULE:
    // Rust will NOT let you add a regular number and an `Option` number!
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    
    // let sum = x + y; // ❌ ERROR! 
    
    // Why Error? Because `y` is still inside the Gift Box! 
    // Before you can add `x` and `y`, Rust FORCES you to open the box safely
    // and check if it's empty first. This completely eliminates "Null crashes"!
}
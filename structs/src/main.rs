struct User {     
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
} // The struct User definition creates a new type called User. This type acts as a template, 
  // and we create variables (instances) of this type to hold specific data.

// fn main() {
//   let user1 = User {
//     active: true,
//     username: String::from("Aman"),
//     email: String::from("account@email.com"),
//     sign_in_count: 64,
//   };
//
//   println!("{}", user1.username); // we will everytime have to use this way like 1st {} and then after ',' the value which we want to put
// }

// fn main() {
//    // Rust doesn’t allow us to mark only certain fields as mutable. we will have to make the whole instance (user2) as mutable, if we want to update a struct.
//    let mut user2 = User {
//     active: true,
//     username: String::from("Aman"),
//     email: String::from("another@email.com"),
//     sign_in_count: 1,
//    };
//
//    user2.email = String::from("changed@email.com");
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
//
//     // or we can use Field Init Shorthand, where we dont have to repeat the parameter which we got 
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }

fn main() {
    // Creating Instances with Struct Update Syntax: Make a new user, change the email, and copy everything else from 'old'.
    let user1 = User {
        active: true,
        username: String::from("Aman"),
        email: String::from("account@email.com"),
        sign_in_count: 64,
    };

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another2@email.com"),
    //     sign_in_count: user1.sign_in_count,
    // }; // this is a normal approach

    let user2 = User {
        email: String::from("another2@email.com"),
        ..user1
    }; // this is the struct update way

    // CRITICAL OWNERSHIP NOTE FOR FUTURE REVIEW:
    // When using struct update syntax (..user1), Rust moves complex data (like String) instead of copying it.
    // Because 'username' (a String) was moved from user1 into user2, user1 as a WHOLE is now invalid.
    // Trying to use `println!("{}", user1.username);` right here will cause a compiler error!
    // (You could only reuse user1 if you had provided a new username string for user2 as well).
}

// =========================================================================
// CREATING DIFFERENT TYPES WITH TUPLE STRUCTS
// =========================================================================

// Tuple structs look like standard tuples but have a specific name (type).
// Use them when you want to name the whole group, but naming individual fields feels redundant.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_struct_example() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // 1. TYPE SAFETY BENEFITS:
    // Even though both `Color` and `Point` are made of three i32 values, Rust treats them as COMPLETELY DIFFERENT TYPES.
    // A function expecting a `Color` will reject a `Point`. This prevents mixing up 3D coordinates with colors!

    // 2. ACCESSING DATA (Option A: Dot notation using indices)
    let x_axis = origin.0;
    let y_axis = origin.1;

    // 3. ACCESSING DATA (Option B: Destructuring)
    // Unlike regular tuples, you must explicitly state the Struct name when destructuring.
    let Point(x, y, z) = origin;
}

// =========================================================================
// DEFINING UNIT-LIKE STRUCTS
// =========================================================================

// Unit-like structs have no fields and store absolutely no data.
// They are written with just a semicolon at the end.
struct AlwaysEqual;

fn unit_struct_example() {
    // You instantiate it simply by using its name
    let subject = AlwaysEqual;

    // WHY USE THEM? 
    // They are used as placeholders when you need to implement a Trait (a behavior) 
    // on a type, but the type itself doesn't need to hold any internal data.
}

/ =========================================================================
// OWNERSHIP OF STRUCT DATA
// =========================================================================

// We purposefully use owned data types like `String` instead of `&str` references inside structs.
// This ensures that the struct completely OWNS its data, meaning the data stays valid as long as the struct exists.
// Storing references (like `&str`) requires using "Lifetimes" (<'a>), which tells the compiler how long the referenced data lives.
// (Lifetimes are covered later in Chapter 10, so use `String` for now!).


// --------------------------------------------------------------------------------------------------------------------------------

// =========================================================================
// RUST BOOK - SECTION 5.2: AN EXAMPLE PROGRAM USING STRUCTS
// =========================================================================

// OBJECTIVE: Group related data together to make a program that calculates the 
// area of a rectangle more readable, manageable, and memory-safe.

// -------------------------------------------------------------------------
// APPROACH 1: Using Plain Variables (No grouping)
// -------------------------------------------------------------------------
// Issue: The parameters 'width' and 'height' are completely separate in the code.
// There's no clear sign that they belong to the same rectangle.
/*
fn main() {
    let width1 = 30;
    let height1 = 50;
    println!("Area: {}", area(width1, height1));
}
fn area(width: u32, height: u32) -> u32 { width * height }
*/

// -------------------------------------------------------------------------
// APPROACH 2: Refactoring with Tuples
// -------------------------------------------------------------------------
// Benefit: Better structure! We now pass a single tuple argument `rect1`.
// Issue: Less clear. Tuples don't name elements. We have to use indices (.0 and .1).
// If we mixed up width and height somewhere else, it would be easy to cause bugs.
/*
fn main() {
    let rect1 = (30, 50);
    println!("Area: {}", area(rect1));
}
fn area(dimensions: (u32, u32)) -> u32 { dimensions.0 * dimensions.1 }
*/

// -------------------------------------------------------------------------
// APPROACH 3: Refactoring with Structs (The Best Practice)
// -------------------------------------------------------------------------
#[derive(Debug)] // <-- 1. This outer attribute explicitly opts-in to Debug formatting
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // 2. BORROWING THE STRUCT:
    // We pass `&rect1` (an immutable borrow) rather than taking ownership.
    // This allows `main` to retain ownership so we can keep using `rect1` later.
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // ---------------------------------------------------------------------
    // DEBUGGING & PRINTING STRUCTS
    // ---------------------------------------------------------------------
    
    // Rust structs DO NOT implement the standard `Display` trait `{}` by default 
    // because there are too many ways a developer might want to format custom types.
    
    // Instead, we use `Debug` formatting placeholders:
    println!("rect1 is {:?}", rect1);   // Single line format: Rectangle { width: 30, height: 50 }
    println!("rect1 is {:#?}", rect1);  // Pretty-print (multi-line) format

    // ---------------------------------------------------------------------
    // THE dbg! MACRO
    // ---------------------------------------------------------------------
    let scale = 2;
    let rect2 = Rectangle {
        // dbg! prints file/line number + expression value to standard error (stderr)
        // It RETURNS ownership of the expression's value, so we can wrap math directly in it:
        width: dbg!(30 * scale), 
        height: 50,
    };

    // dbg! takes ownership, so we pass a reference `&rect2` if we want to keep using rect2!
    dbg!(&rect2); 
}

// Accessing fields of a borrowed struct instance (`rectangle.width`) DOES NOT move the values.
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
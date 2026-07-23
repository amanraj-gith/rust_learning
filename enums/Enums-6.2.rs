// Rust Chapter 6.2: The match Control Flow Construct

// 1.  What is match?
//     Think of match like a shape-sorter toy or coin-counting machine. It compares a value against a series of patterns and executes code for the first pattern that fits.

// 2.  How match Works
//     You pass a value into match. Rust checks each pattern arm from top to bottom. As soon as a pattern matches, the code attached to that arm runs. If an arm needs multiple lines of code, wrap them in curly braces.

// 3.  Matching with Data Enums (Surprise Boxes)
//     In Rust, enum variants can store data inside them.
//     Example: Coin::Quarter(UsState::Alaska)
//     Here, UsState::Alaska is a custom enum variant stored inside Quarter.
//     When you write Coin::Quarter(state), Rust automatically opens the enum package, unwraps the inner state value, and binds it to the variable state so you can use it in your code.

// 4.  Matching with Option
//     Option is an enum that represents a value that might exist or be missing:

//   - Some(value) contains a value inside.
//   - None means empty / no value.
//     Using match on Option lets you safely unwrap Some(val) or handle None cleanly without causing crashes.

// 5.  Matches Are Exhaustive
//     Rust strictly requires you to handle every single possible variant. If you miss even one case (like forgetting None), the Rust compiler will refuse to compile your code.

// 6.  Catch-All Patterns and Wildcards
//     To handle leftover cases:

//   - Use a variable name like other if you want to capture and use the remaining values.
//   - Use _ (underscore wildcard) to match all remaining cases and ignore their values.
//   - Use _ => () to catch all remaining cases and do nothing.

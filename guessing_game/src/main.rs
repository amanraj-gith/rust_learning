use std::io; // standard library for input/output, they are not prelude like they don't come builtIn in rust like clone and vec.
use std::cmp::Ordering;  // this library is for matching (like Case in other languages)
use rand::Rng;  // Rust don't have it's won random value generator function, so we use rand crate for this


fn main(){
    println!("Welcome to Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);


  loop{
    println!("Please input your guess number");

    let mut guess = String::new();   // All the variables in rust are immutable by default, so to make them mutable (chnageable) we use mut keyword

    io::stdin()
             .read_line(&mut guess)          // all references also need mutablitly
             .expect("Failed to read line!");   // try and error like thing here, In case this line got any error so we use expect to handle that,
                                               //  in rust read_line it always return one result value which is an enum and can give any two values either Ok or Err. 

    let guess : u32 = guess.trim()    // this is not a new var, but we are usign shadowing for the 'guess' variable.
                           .parse()   // this is to parse guess variable from string to unsigned integer.
                           .expect("Failed to take input!"); 
   
    println!("You guessed, {}", guess);
    println!("You secret_number is, {}", secret_number);

   match guess.cmp(&secret_number){   // This is same as Case statement just like c++ or c lang.
    Ordering::Less => println!("Too small"),
    Ordering::Equal => {
        println!("You win");
        break;
    }
    Ordering::Greater => println!("Too big"),
   }
  }
}
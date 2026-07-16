use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main(){
    println!("Guess the Number");

    let secret_number = rand::thread_rng().gen_range(1..=100);


  loop{
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line!");

    let guess : u32 = guess.trim().parse().expect("Failed to take input!");
   
    println!("You guessed, {}", guess);
    println!("You secret_number is, {}", secret_number);

   match guess.cmp(&secret_number){
    Ordering::Less => println!("Too small"),
    Ordering::Equal => {
        println!("You win");
        break;
    }
    Ordering::Greater => println!("Too big"),
   }
  }
}
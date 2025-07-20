use std::io; // reading input

use rand::Rng; // imported crate for generating pseudo-random number

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("🙈 The secret number is: {secret_number}");

    println!("Input your guess, player 😘");

    /* 
        variables are immutable by default 
        *mut* keyword initializes mutable variable
    */

    // ::new => new is an associated function of `String` type 
    let mut guess = String::new(); // new instance of a String
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line 😭 im sorryyy");
    
    println!("You guessed: {guess}"); 

}

use std::io; // reading input

fn main() {
    println!("Guess the number!");

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

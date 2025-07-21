use std::cmp::Ordering;
use std::io; // reading input

use rand::Rng; // imported crate for generating pseudo-random number

fn main() {
    println!("\n🌟🌟🌟-----------------------------🌟🌟🌟");
    println!("          🎯  GUESSING GAME ");
    println!("🌟🌟🌟-----------------------------🌟🌟🌟\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("(psst... I picked a secret number between 1 and 100 🤫)");

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

    /*
        Wait... didn't we just create a variable named guess!?
        
        Shadowing allows us to reuse variable names, 
        the feature is often used in type-castings.

        Trim removes whitespaces and newline / carriage return.
    */ 
    let guess: u32 = guess.trim().parse().expect(
        "Plz type a number..."
    );
    
    println!("\n🥁 You guessed: {guess} 🥁\n"); 

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small 🤏 try again!"),
        Ordering::Greater => println!("Too big 🙈 guess lower!"),
        Ordering::Equal => println!("🎉 You win, boss babe ✨👑"),
    }
}

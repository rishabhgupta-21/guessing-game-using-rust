// Bring the 'io' module into scope from the standard library (std)
use std::io;

fn main() {
    println!("Welcome to the Guessing Game!!!");
    println!("Make your guess below!");

    // Creating a variable to store the user's guess
    let mut guess = String::new();

    // Receiving the user's input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
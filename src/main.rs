// Bring the 'io' module into scope from the standard library (std)
use std::io;
// The Rng trait defines methods that random number generators implement
use rand::Rng;

fn main() {
    println!("Welcome to the Guessing Game!!!");
    println!("Make your guess below!");

    // Generating a Random Number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    // Creating a variable to store the user's guess
    let mut guess = String::new();

    // Receiving the user's input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Displaying the user's guess
    println!("You guessed: {guess}");
}
use std::io;

use rand::{thread_rng, Rng};

// Generating a Random Number
pub fn f3_2() {
    println!("Guess the number!");

    let secret_number: i32 = thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}

use std::{cmp::Ordering, io};

use rand::{thread_rng, Rng};

// Comparing the Guess to the Secret Number
pub fn f4_0() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    let secret_number: u32 = thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

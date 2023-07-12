use std::io;

// Processing a Guess
pub fn f2_0() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}

// Printing Values with println! Placeholders
pub fn f2_4() {
    let x: i32 = 5;
    let y: i32 = 10;

    println!("x = {x} and y + 5 = {}", y + 5)
}

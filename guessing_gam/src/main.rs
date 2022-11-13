use std::io;

fn main() {
    println!("Play the Guessing Game!");
    println!("Guess a number..");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Error reading line..");

    println!("Is the number you guessed: {}", guess)
}
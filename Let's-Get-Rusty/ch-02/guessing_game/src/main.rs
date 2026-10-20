use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input you guess ");
    // Rust is immutable by default
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1, 101); //101 is not included
    println!("The Secret number is: {}", secret_number);
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}

use rand::Rng;
use std::cmp::Ordering; //cmp mean comparing and ordering is an enum which results in 3 things greater,less,equal
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
    match guess.cmp(&secret_number) {
        // Error here as it is comparing string with integer (guess is a string taken input so we change that to integer)
        // cmp returns ordering enum
        // match expression matches with all the possible return values
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

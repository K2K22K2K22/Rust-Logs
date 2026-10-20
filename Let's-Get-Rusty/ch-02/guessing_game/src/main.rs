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
    /*
     * trim Removes the white space at the beginnin or end of a line
     * parse will convert the string type but it does not know what to convert the string type into
     * so we annotate our variable and use the unsigned 32bit integer
     * since parse could fail so it also returns an error
     */
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    match guess.cmp(&secret_number) {
        // cmp returns ordering enum
        // match expression matches with all the possible return values
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

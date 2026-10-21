fn main() {
    //Variables are immutable by default in rust
    let x = 5;
    println!("The value of x is: {}",x)
    // Since Immutable it would give error on next line
    x = 6;
    println!("The value of x is: {}",x);
}

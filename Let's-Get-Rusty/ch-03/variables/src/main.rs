fn main() {
    //Variables are immutable by default in rust
    //Use mut to make it mutable
    let mut x = 5;
    println!("The value of x is: {}",x);
    x = 6;
    println!("The value of x is: {}",x);
}

fn main() {
    //Variables are immutable by default in rust
    //Use mut to make it mutable
    //no need for annotation it auto annotated to unsigned 32bit integer
    let mut x = 5;
    println!("The value of x is: {}",x);
    x = 6;
    println!("The value of x is: {}",x);

    //When declarin constant we commonly use all uppercase
    //constants are not mutable even when we use mut 
    //they also be Type annotated (like here we did unsigned 32bit integer )
    //we use Const to make a variale constant
    //we cannot make any computated value constant (like when a funtion is returning a value we cannot
    //set that to a constant )
    //in rust we can use _ to make no. more readable
    const HEAD_COUNT: u32 = 100_000;
    println!("The constant value is: {}", HEAD_COUNT);
    
    //Shadowing allow us to create a new variable using an existing name.... bascally overwrites it

    //basically instead of making y mutable we can:
    let y = 10;
    println!("The value of y is: {}",y);
    let y = "AK";
    println!("The value of y is: {}",y);
    //The First Y value is Shadow-ed by the second y value 
    //We preserved Mutablility (both are immutable)
    //We can even change the type of the same variable 
}

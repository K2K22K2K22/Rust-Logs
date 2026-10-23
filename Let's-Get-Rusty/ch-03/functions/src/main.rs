fn main() {
    my_function();
    let ad = addtion(2, 15);
    println!("The Sum is:{}", ad);
    let sub = diff(2, 15);
    println!("The Subtraction is:{}", sub);
    let mul = prd(2, 15);
    println!("The Multiplication is:{}", mul);
}

fn my_function() {
    //function name should be all lower case
    println!("Another Function");
}

fn addtion(x: i32, y: i32) -> i32 {
    //we need to specify the type of variable
    //we use -> to set a return type
    //since it is returning an integer we need to add that i32

    println!("The Value of X is {}", x);
    println!("The Value of Y is {}", y);
    let sum = x + y;
    return sum;
}

fn diff(x: i32, y: i32) -> i32 {
    let minus = x - y;
    minus // we don't need to use the return key word we can put it at the end
}

fn prd(x: i32, y: i32) -> i32 {
    x * y //or we can more simplify it to this (no need to even store it just put it at the end and it would get returned)
}

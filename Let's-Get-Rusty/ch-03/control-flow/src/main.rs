fn main() {
    let number = 5;
    if number < 10{ //should always be boolean
        println!("first condition was true");
    }else if number < 22 {
        println!("second condition was true ");
    } else{
        println!("condition was false");
    }

    //using if-else inside a let statment 
    let condition = true ;
    let a = if condition {5} else {6};

    //loops
    //first one 
    loop {
        println!("again!");
        // untill used break it goes forever
        break;
    }
}

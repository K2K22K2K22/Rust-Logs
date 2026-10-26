fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10{
            break counter; // This would return counter 
        }
    };
    println!("The result is {}", result);
    
    // 2nd type of loop
    let mut number = 4;
    while number != 0{
        println!("{}!", number);
        
        number -= 1;
    }
    println!("LIFTOFF!!!");
    
    // 3rd type of loop
    let a = [10, 20, 30, 40, 50];
    
    for element in a.iter(){ // iter() is used to iterate from the array
        println!("The value is: {}", element);
    }
    for num in 1..4 { // From 1 to 4 (4 is exclusive so it only runs till 3)
    println!("{}!", num);
    }
    
    // Line comment 
    /* 
      block comment
     */
}

fn main() {
    // Integers
    
    let a = 99_002; // Decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Byte

    let p: u8 = 255; //Max limit
    // Debug build will do Rust panic
    // Release build will wrap it so 256=0, 257=1, ...

    // Floating-point numbers
    let f = 2.0; // Default 64
    let g: f32 = 3.0;
    
    // addition
    let sum = 5 + 10;
    // subtraction
    let diff = 95.5 - 4.3;
    // multiplication
    let prd = 4 * 30;
    // division
    let qoutient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;

    // Booleans
    let t = true;
    let s: bool = false;

    // Character
    let j = 'Z';
    let z = '>'; //any symbol or emoji

    // Compound Types 
    // Tuple
    let tup = ("Rust-Logs", 100_002); // we can access values by destructing or dot notation 
    let (name,number) = tup; //destructing
    let count = tup.1; // Dot notation

    // Arrays
    let error_codes = [200, 404, 500]; //Fixed length
    let not_found = error_codes[1]; //standard syntax
    // can also be declared by using this method
    let byte = [0; 8]; //Creates an array with 0 values and all set to 0
}

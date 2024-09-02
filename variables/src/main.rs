fn main() {
    println!("Hello, world!");

    let mut age =  30;

    const  MAX_AGE: i32 = 100; // are always immutable and data type should be always mentioned 
    // we can also define a constant as a global variable
    // convention for const is to use only uppercase letters

    age = 25; // variables are immutable in Rust by default, but we can change them to mutable by using mut keyword

    println!("Max Age is {MAX_AGE}");

    println!("Age is {age}");
}

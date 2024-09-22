mod branch;  

use crate::branch::department::Department;

fn main() {
    println!("Hello, world!");

    let metaa = Department {};

    println!("I'm in branch: {:?}", metaa);
}

// package must have atleast 1 crate
// crate types: 1) library crate (lib.rs) 2) binary crate (main.rs)
// package must have 0 or 1 library crate
// package can have any number of binary crates



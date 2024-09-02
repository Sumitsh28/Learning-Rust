use rand::Rng;
use std::cmp::Ordering;
use std::io; // this is a import statement

fn main() {
    println!("Guess the Number!");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop {
    println!("Please Guess the Number:");

    let mut guess = String::new(); // we are able to use String without import because it is a prelude

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to get number!"); //we are handling error by using expect

    let guess: u32 = guess.trim().parse().expect("Failed to parse"); //shadowing (Redeclaration of guess) we are changing type using parse
    //shadowing m type bhi change hoskti hai 

    // let guess: u32 = match guess.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => continue,
    // }; handling invalid input

    println!("Use guessed: {guess}"); // or println!("Use guessed: {}, guess");

    match guess.cmp(&secret_num) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }
}
}

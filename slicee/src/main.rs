fn main() {
    println!("Hello, world!");

    let mut s = String::from("hello world");


    // let word = first_word(&s);

    //slice
    let hello = &s[0..5];
    let world = &s[6..11];

   

    println!("{hello},{world}");
}

// word is changed after storing the value it will tell wrong  word

// we use slice , just like in python

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

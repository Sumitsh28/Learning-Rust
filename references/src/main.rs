// we can use & to pass arguments as a reference instead of taking full ownership

// references are immutable by default
// but we can give permission to mutate by using &mut


fn main() {
    println!("Hello, world!");

    let s1 : String = String::from("Sumit");

    let mut s3 : String = String::from("Sumit");

    let s2 : String = referr2(&mut s3);

    println!("{s3}");
    println!("{s2}");
}

fn  referr(s : &String) -> String {

    // s.push_str("shandillya"); // can't do this because its default immutable
    s.to_string()
}

fn  referr2(s : &mut String) -> String {

    s.push_str("shandilya"); // can't do this because its default immutable
    s.to_string()
}

// you cant have 2 mutable references  to the same value at the same time

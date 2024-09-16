// only one owner can be there


// fn main() {

    // shallow copy

    // let name : String = String::from("Sumit");
    // let name2 = name;

    // println!("Name {name2}"); // will work
    // println!("Name {name}"); // will not work

    // very expensive operation, causes double free error


    // deep copy

    // let name : String = String::from("Sumit");
    // let name2 = name.clone();
    
    // println!("Name {name2}"); // will work
    // println!("Name {name}"); // will work

    // very expensive operation, doesn't causes double free error

   
   
// }

fn main() {

    // let name : String = String::from("Sumit");

    

    println!("Hello, world!");
    // take_ownership(name); // will run

//     take_ownership(name); // will not run
// Why?? Beacuse ownership of  name is passed to take_ownership function and  it is dropped after execution of that function

    let s1 = String::from("Hello");
    let s2  = take_and_give_ownership(s1); // s1 takes ownership of name

    // println!("{s1}");
    println!("{s2}");


   
}

fn take_and_give_ownership(s:String) -> String {

    let s = String::from("Ownership taking for s1 ");
    s
}

fn gives_ownership() -> String {
    let s = String::from("Giving ownership to s1");
    s
}
fn take_ownership(s:String){
    println!("{}",s);
}


// these things are very tedious 
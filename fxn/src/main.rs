fn main() {
    // println!("Hello, world!");

    // frist_fxn(21,"Sumit");


    // Fxn expression

    // let x = {
    //     let y = 10;
    //     y + 5
    // };

    let x = add(5,5);

    println!("Value: {x}");
}

fn frist_fxn(x:i32,s:&str){

    println!("My age is {x}, my name is {s}");
}

//return fxn

fn  add(x:i32,y:i32) -> i32{

    x+y // this means it will return this value

    // x+y; this means it will evaluate to something
}

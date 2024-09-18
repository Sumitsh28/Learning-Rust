// #[derive(Debug)]
// enum Nation {
//     State,
//     City,
// }

// fn nationality(name: &str, kind: Nation) {
//     println!("Name of place is {name}, it is a {:?}", kind);
// }

// fn main() {
//     println!("Hello, world!");
//     nationality("USA", Nation::State);
// }

// #[derive(Debug)]
// enum Nation {
//     State(String),
//     City(String),
//     Zip(u8,u8,u8,u8),
// }



// fn main() {

//     let new_place1 : Nation = Nation::State(String::from("Delhi"));
//     let new_place2 : Nation = Nation::City(String::from("New Delhi"));
//     let new_place3 : Nation = Nation::Zip(1,1,0,0);

//     println!("Hello, world!");
//     println!("State is : {:?}, City is : {:?}, Postal Code is : {:?}",  new_place1, new_place2, new_place3);
// }

// enums can be used as structs too

// fn main() {
//     enum Message {
//         Quit,
//         Move { x: i32, y: i32 },
//         Write(String),
//         ChangeColor(i32, i32, i32),
//     }

//     impl Message {
//         fn call(&self) {
//            println!("{:?}", self.Write);
//         }
//     }

//     let m = Message::Write(String::from("hello"));
//     m.call();
// }

// cant access fields of an enum variant directly
// we will use match types for it


// cant add option and i8
// As such, Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is Option<T>
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; this wont run 
    let sum = x + y.unwrap();  // this will run
}
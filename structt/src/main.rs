// struct Info {
//     firstName: String,
//     secondName:  String, // we are using String instead of &str beacuse we want the struct to have ownership of this
//     age: u64,
// }

// tuple as struct

// struct Color(u64,u64,u64);

//unit like struct

// struct  Unit;

// fn buildInfo(firstName:String) -> Info {

//     Info{
//         firstName,
//         secondName:  String::from("Shandillya"),
//         age: 21,
//     }
// }

struct Rec {

    height: u64,
    width: u64,
}

impl Rec {

    fn area(&self) -> u64{
        self.height * self.width
    }

    fn can_hold(&self,  other: &Rec) -> bool {
        self.height > other.height && self.width > other.width
    }

    // Associated Fxns just like String::from()
    fn square(num: u64) -> Self{
        Self{
            width: num,
            height: num,
        }
    }
}

fn main() {

    // let mut info1 : Info = Info {
    //     firstName: String::from("Sumit"),
    //     secondName: String::from("Shandillya"),
    //     age: 21,
    // };

    // info1.age = 22;

    // let info2: Info = buildInfo(String::from("Alka"));

    // let info3 : Info = Info{

    //     firstName: String::from("Shrey"),
    //     ..info1
    // };

    // let black = Color(0,0,0);

    // let unitStruct = Unit;

    // println!("My name is {} {}, my age is {}",info3.firstName,info3.secondName, info3.age);

    let rec1 = Rec{
        height: 10,
        width:10,
    };

    let rec2 = Rec{
        height: 5,
        width:5,
    };

    // Associated functions
    let sq = Rec::square(6);
    

    println!("Can rec1 hold rec2:  {}", rec2.can_hold(&rec1));
}

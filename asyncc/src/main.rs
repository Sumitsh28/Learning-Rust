use futures::{join,select,pin_mut};
use futures::future::FutureExt;

fn main() {
    println!("Hello, world!");

    let num1 = get_num().fuse();
    let num2 = get_num2().fuse();
    let num3 = get_num3().fuse();

    pin_mut!(num1,num2,num3);


    // println!("{:?}",num);//will not run

    let ans = smol::block_on(
        async {
            loop{

                select!{
                    x = num1 => println!("num1 comp {}",x),
                    x = num2 => println!("num2 comp {}",x),
                    x = num3 => println!("num3 comp {}",x),

                    complete => {
                        println!("All comp");
                        break;
                    }
                }
            }
        }
    );

    println!("{:?}",ans); // will run and return 10
}

async fn get_num() -> u8 {
    
    return 10;
}

async fn get_num2() -> u8 {
    std::thread::sleep(std::time::Duration::from_millis(50));
    return 11;
}

async fn get_num3() -> u8 {
    std::thread::sleep(std::time::Duration::from_millis(50));
    return 12;
}

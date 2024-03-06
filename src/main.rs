use std::io;
use std::cmp::Ordering;
use rand::prelude::*;

fn main() {
    println!("猜数字游戏");

    let secret_number = thread_rng().gen_range(1..101);
    println!("神秘的数字： {}", secret_number);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("无法读取行");
    
        let guess:u32 = match  guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue 
        };
    
        println!("你猜测的数是： {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("相等, YOU WIN!");
                break;
            }
        }
    }
}

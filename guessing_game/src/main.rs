use rand::Rng;
use std::cmp::Ordering;
use std::io;

// Rng 是一个 trait

// Rust 有静态强类型系统

fn main() {
    println!("Guess the number!");
    // 1..101 类范围表达式 1 <= x < 101
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // mut 可变
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Rust 允许用一个新值来隐藏（shadow）guess之前的值
        // let guess: u32 = guess.trim().parse().expect("Please type a number");
        // match 表达式
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // 猜对 退出游戏
                break;
            }
        }
    }
}

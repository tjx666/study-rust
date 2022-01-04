use rand::Rng; // 使用 cargo doc --open 可以打开 crates 的 文档
use std::cmp::Ordering;
use std::io;

fn main() {
    // 是宏不是函数
    println!("Guessing the number!");
    // 不像 js 声明一个变量很多种方式，rust 声明变量就用 let
    // rust 变量默认是 immutable 的，使用 let mut 可以声明可变变量
    let secret: u32 = rand::thread_rng().gen_range(1..11);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess) // 引用默认也是 immutable 的，默认函数传参也是值传递
            .expect("Failed to read line");
        // 不像 java/js 使用 try/catch 处理异常，貌似 rust 都是通过返回码的方式
        // expect 返回的是 Result 类型的枚举值

        // 模式匹配，给我的初步影响就是高级版的 switch
        // 但是比 switch 强大的多
        // 1. match 表达式是表达式，意味着可以返回值
        // 2. 神奇的是表达式可以 使用 break/continue，甚至语句块
        let guess: u32 = match guess.trim().parse() {
            Result::Ok(v) => v,
            Result::Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                // match 表达式可以用语句处理分支
                println!("You win!");
                break;
            }
            Ordering::Less => println!("Too small!"),
        }
    }
}

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess number!");

    let secret_number = rand::thread_rng().gen_range(1..100);

    println!("secret number is {secret_number}");

    loop {
        println!("input number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // &mut 代表引用，read_line方法将在guess后面追加字符串
            .expect("Fail to read line"); // expect处理上面返回的error的enum类型，未经处理将会产生警告
                                          // 消除警告的正确做法是实际去编写错误处理代码，不过由于我们就是希望程序在出现问题时立即崩溃，所以直接使用 expect

        // 允许复用声明，前面的guess被隐藏，u32 -> 无符号32位
        let guess: u32 = match guess.trim().parse() {
            // 转换数据类型，具体类型由接收的变量决定
            Ok(num) => num,
            Err(_) => {
                println!("Not a number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        // println!("msg:{}", guess + "some")

        // 分支（arms）,使用match来匹配所有的enum模式(pattern)
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
        }
    }
}

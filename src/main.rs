use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Please input your guess.");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        // :: 叫关联函数，相当于其他语言里的静态方法
        let mut guess = String::new();

        // 这句把控制台读取到的字符串添加到guess的尾部
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("请输入数字!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => break,
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
        }
    }

    println!("You win !!!");
}

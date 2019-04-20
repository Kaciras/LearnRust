use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::error::Error;

fn console_read_string() -> Result<i32, Box<dyn Error>> {
    // :: 叫关联函数，相当于其他语言里的静态方法
    let mut guess = String::new();

    // 这句把控制台读取到的字符串添加到guess的尾部
    io::stdin().read_line(&mut guess)?;

    // 不能直接返回 parse 的 Result？
    return Ok(guess.trim().parse::<i32>()?); // <- i32 可以从返回值定义推导
}

fn main() {
    println!("Please input your guess.");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let guess = match console_read_string() {
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

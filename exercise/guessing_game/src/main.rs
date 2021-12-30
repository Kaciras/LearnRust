use std::cmp::Ordering;
use std::error::Error;
use std::io;

use rand::Rng;

/// 从控制台读取一行，并转换为整数。
/// 文档注释的编写：
/// https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments
fn read_int() -> Result<i32, Box<dyn Error>> {

    // :: 叫关联函数，相当于其他语言里的静态方法
    // 从源码看里面用了 Vec，应该是可变的，相当于 JAVA 里的 StringBuilder？
    let mut guess = String::new();

    // 这句把控制台读取到的字符串添加到guess的尾部
    io::stdin().read_line(&mut guess)?;

    // 不能直接返回 parse 的 Result？
    return Ok(guess.trim().parse::<i32>()?); // <- i32 可以从返回值定义推导
}

fn main() {
    println!("Please input your guess:");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        let guess = match read_int() {
            Err(_) => {
                println!("请输入数字!");
                continue;
            }
            Ok(value) => value,
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => break,
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
        }
    }

    println!("卧槽你好牛逼，这都能猜到，You win !!!");
}

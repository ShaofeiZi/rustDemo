use std::io;

fn main() {
    println!("猜数");
    println!("猜测一个数");
    // 定义一个可变空字符串
    let mut guess = String::new();
    // 接收猜测的值
    io::stdin().read_line(&mut guess).expect("猜的啥？");

    println!("你猜的是 {}",guess);
}


use std::io;    // 为了获得用户的输入并将其打印出来，我们需要把标准库（也就是所谓的std）中的io模块引入当前的作用域中
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // 以let开头的语句创建了一个新的变量 （variable）
    // mut 表示可变
    let mut guess = String::new();

    // 引用与变量一样，默认情况下也是不可变的。因此，你需要使用&mut guess而不是&guess来声明一个可变引用。
    // 参数前面的&意味着当前的参数是一个引用
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    // 字符串中的那对花括号{}则是一个占位符，它用于将后面的参数值插入自己预留的特定位置
    println!("You guessed: {}", guess);
}

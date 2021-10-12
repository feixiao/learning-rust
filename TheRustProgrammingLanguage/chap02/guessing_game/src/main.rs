use std::cmp::Ordering;
use std::io;

// 为了获得用户的输入并将其打印出来，我们需要把标准库（也就是所谓的std）中的io模块引入当前的作用域中
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // 生成随机数
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);



    loop {
        println!("Please input your guess.");

        // 以let开头的语句创建了一个新的变量 （variable）
        // mut 表示可变
        let mut guess = String::new();

        // 引用与变量一样，默认情况下也是不可变的。因此，你需要使用&mut guess而不是&guess来声明一个可变引用。
        // 参数前面的&意味着当前的参数是一个引用
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Rust允许使用同名的新变量guess来隐藏 （shadow）旧变量的值。
        // 新创建的guess变量被绑定到了表达式guess.trim().parse()产生的结果上。
        // let guess: u32 = guess.trim().parse()
        //     .expect("Please type a number!");

        // 输入错误版本
        // 我们使用了match表达式来替换之前的expect方法，这是我们处理错误行为的一种惯用手段。
        // parse会返回一个Result类型，而Result则包含了Ok与Err两个变体。
        let guess: u32= match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // match表达式由数个分支 （arm）组成，每个分支都包含一个用于匹配的模式 （pattern），以及匹配成功后要执行的相应的代码。
        // Rust会尝试用我们传入match表达式的值去依次匹配每个分支的模式，一旦匹配成功，它就会执行当前分支中的代码。
        match guess.cmp(&secret_number) {
            Ordering::Less => { println!("Too small!") }
            Ordering::Greater => { println!("Too big!") }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    // 字符串中的那对花括号{}则是一个占位符，它用于将后面的参数值插入自己预留的特定位置
    // println!("You guessed: {}", guess);
}

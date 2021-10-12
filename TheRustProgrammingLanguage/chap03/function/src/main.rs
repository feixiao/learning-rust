fn main() {
    another_function(5, 6);

    let x = 5;
    let y = {
        let x = 3;
        // 注意结尾处的表达式x + 1没有添加分号，这与我们之前见过的大部分代码不同。
        x + 1
    };
    println!("The value of y is: {}", y);


    println!("The value of five() is: {}", five());

    println!("The value of plus_one() is: {}", plus_one(121));
}


// 在函数签名中，你必须 显式地声明每个参数的类型。
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}


// five函数没有参数，仅仅定义了返回值的类型
fn five() -> i32 {
    5   // 没有添加分号
}

fn plus_one(x: i32) -> i32 {
    x + 1   // 没有添加分号
}
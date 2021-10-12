fn main() {
    let number = 3;

    // 条件表达式必须产生一个bool类型的值，否则就会触发编译错误。
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }


    let condition = true;
    let number = if condition {
        5       // 没有添加分号
    } else {
        6       // 没有添加分号
    };
    println!("The value of number is: {}", number);


    // loop 和  break
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // 使用break关键字返回counter * 2
        }
    };
    println!("The result is {}", result);



    // while
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");

    // for
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // rev来翻转Range生成的序列
    for number in (1..4).rev() {
        println!("{}!", number); // 3! 2! 1!
    }
}

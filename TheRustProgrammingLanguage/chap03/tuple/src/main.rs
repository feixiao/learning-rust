fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    // 通过索引并使用点号（.）来访问元组中的值
    println!("The value of y is: {}", tup.1);


    // 定义数组
    let a = [1, 2, 3, 4, 5];

    // 输出数组
    println!("The value of a is: {:?}", a);


    // 分号之后的5则表明当前的数组包含5个元素
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a is: {:?}", a);


    // 以a命名的数组将会拥有5个元素
    let a = [3; 5];
    println!("The value of a is: {:?}", a);

}
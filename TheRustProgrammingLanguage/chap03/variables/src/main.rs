fn main() {

    // let x = 5; // cannot assign twice to immutable variable

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 隐藏机制不同于将一个变量声明为mut，因为如果不是在使用let关键字的情况下重新为这个变量赋值，则会导致编译错误。
    // 隐藏机制与mut的另一个区别在于：由于重复使用let关键字会创建出新的变量，所以我们可以在复用变量名称的同时改变它的类型
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}

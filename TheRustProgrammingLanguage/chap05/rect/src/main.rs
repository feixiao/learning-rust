// #[derive(Debug)]注解
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


// 我们需要将area函数移动到一个由impl（implementation）关键字起始的代码块中
// 并把签名中的第一个参数（也是唯一的那个参数）和函数中使用该参数的地方改写为self。
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }


    // 除了方法，impl块还允许我们定义不用接收self作为参数的函数。
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}


fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is {:?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

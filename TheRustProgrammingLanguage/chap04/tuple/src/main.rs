fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);


    let s1 = String::from("hello");
    let len = calculate_length_2(&s1);
    println!("The length of '{}' is {}.", s1, len);


    // 需要将变量s声明为mut，即可变的
    let mut s = String::from("hello");

    // 其次，我们使用&mut s 来 给 函 数 传 入 一 个 可 变 引 用
    change(&mut s);

    println!("The length of '{}' is {}.", s, calculate_length_2(&s));
}


// 使用tuple完成多个返回值
fn calculate_length(s:String) -> (String, usize) {
    let length = s.len();
    (s, length)
}


// 这些&代表的就是引用 语义，它们允许你在不获取所有权的前提下使用值
fn calculate_length_2(s : &String) -> usize {
    s.len()
}

// 与变量类似，引用是默认不可变的，Rust不允许我们去修改引用指向的值
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
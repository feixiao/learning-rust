fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);


    let s1 = String::from("hello");
    let len = calculate_length_2(&s1);
    println!("The length of '{}' is {}.", s1, len);
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

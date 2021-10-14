


// 我们需要给返回类型标注一个泛型生命周期参数，
// 因为Rust并不能确定返回的引用会指向x还是指向y。
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {y
//     }
// }

// 生命周期的标注并不会改变任何引用的生命周期长度。
// 在这个签名中我们所表达的意思是：参数与返回值中的所有引用都必须拥有相同的生命周期。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn main() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

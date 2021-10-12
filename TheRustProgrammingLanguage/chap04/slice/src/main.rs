fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    // s.clear(); // 错误  由于clear需要截断当前的String实例，所以调用clear需要传入一个可变引用。

    println!("the first word is : {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate()  {
        if item == b' ' {
            return &s[0..i];    //  遇到第一个空格就返回
        }
    }

    &s[..]
}
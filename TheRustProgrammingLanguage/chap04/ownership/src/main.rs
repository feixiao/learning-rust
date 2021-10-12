fn main() {
    let s = String::from("hello");  // 变量S进入作用域

    take_ownership(s);  // s的值被移动到函数所以从这边开始不再有效


    // println!("{}", s);      //  value borrowed here after move


    let x = 5;          // 变量x进入作用域


    // 变量x同样被传递到函数，但是由于i32的Copy的，所以后面可以继续使用x
    makes_copy(x);


    let s1 = gives_ownership();  // 返回值移到到了s1

    let s2 = String::from("hello");

    // s2被移动到函数内部，返回值又被移到变量s3
    let s3 = takes_and_gives_back(s2);
}


fn take_ownership(some_thing: String) {
    println!("{}", some_thing);
}   // some_thing的内容会被释放，drop函数自动调用

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

// gives_ownership 会将它的返回值移到调用它的函数内
fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string  // 作为返回值移到了

}


fn takes_and_gives_back(a_string : String) -> String {
    a_string    // 作为返回值移到调用函数
}
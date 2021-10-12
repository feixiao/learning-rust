fn main() {
    let s = String::from("hello");  // 变量S进入作用域

    take_ownership(s);  // s的值被移动到函数所以从这边开始不再有效


    // println!("{}", s);      //  value borrowed here after move


    let x = 5;          // 变量x进入作用域


    // 变量x同样被传递到函数，但是由于i32的Copy的，所以后面可以继续使用x
    makes_copy(x);

}


fn take_ownership(some_thing: String) {
    println!("{}", some_thing);
}   // some_thing的内容会被释放，drop函数自动调用

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
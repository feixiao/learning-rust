

use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),   // 为了拥有固定大小而使用Box<T>的List定义
    Nil,
}



fn main() {

    // 一个持有Box的值的变量b，它指向了堆上的值5
    let b = Box::new(5);
    println!("b = {}", b);


    let list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(Nil))))));
}

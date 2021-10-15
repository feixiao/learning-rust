
use std::ops::Deref;
use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),   // 为了拥有固定大小而使用Box<T>的List定义
    Nil,
}



// 定义了一个名为MyBox的结构体。结构体的定义中附带了泛型参数T，因为我们希望它能够存储任意类型的值。
struct  MyBox<T>(T);

impl<T> MyBox<T>  {
    fn new(x:T) -> MyBox<T> {
        MyBox(x)
    }
}


// 通过实现Deref trait来将类型视作引用
impl<T> Deref for MyBox<T> {
    type Target = T;        // 定义了Deref trait的一个关联类型
    fn deref(&self) -> &T {
         &self.0
    }
}


fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {

    // 一个持有Box的值的变量b，它指向了堆上的值5
    let b = Box::new(5);
    println!("b = {}", b);


    let list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(Nil))))));


    let x = 5;

    let y = &x;
    // 把Box<T>当成引用来操作
    let z = Box::new(x);


    assert_eq!(5,x);
    assert_eq!(5,*y);
    assert_eq!(5,*z);


    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);


    // 解引用转换特性使我们可以将MyBox<String>值的引用传入hello函数
    // 解引用转换（deref coercion）是Rust为函数和方法的参数提供的一种便捷特性。
    // 当某个类型T实现了Deref trait时，它能够将T的引用转换为T经过Deref操作后生成的引用。
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // 如果Rust没有解引用转换功能，就必须编写这样的代码
    // hello(&(*m)[..]);
}

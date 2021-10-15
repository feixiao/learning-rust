
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;


fn main() {
    // 创建了一个Rc<RefCell<i32>>实例，并将它暂时存入了value变量中❶以便之后可以直接访问
    let value = Rc::new(RefCell::new(5));

    // 为了确保a和value同时持有内部值5的所有权，这里的代码还克隆了value，
    // 而不仅仅只是将value的所有权传递给a，或者让a借用value。
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    // 我们通过调用borrow_mut来将value指向的值增加10。
    // 注意，这里使用了自动解引用功能来将Rc<T>解引用为RefCell<T>。
    // borrow_mut方法会返回一个RefMut<T>智能指针，我们可以使用解引用运算符来修改其内部值
    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}



// 结构体的定义
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
// build_user函数中使用了相同的参数名与字段名，并采用了字段初始化简写语法进行编写
fn build_user(email: String, username: String) -> User {
    User {
        email,   // 因为参数名与字段名相同所以省略
        username,
        active: true,
        sign_in_count: 1,
    }
}


// 定义元组结构体时依然使用struct关键字开头，并由结构体名称及元组中的类型定义组成
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


fn main() {
    // 不可变结构
    let user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // user.active = false; // cannot assign to `user.active`, as `user` is not declared as mutable


    // 一旦实例可变，那么实例中的所有字段都将是可变的。
    // Rust不允许我们单独声明某一部分字段的可变性。
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");



    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1     // 使用结构体更新语法来为一个User实例设置新的email和username字段的值，并从user1实例中获取剩余字段的值
    };


    // 这里的black和origin是不同的类型，因为它们两个分别是不同元组结构体的实例。
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

}




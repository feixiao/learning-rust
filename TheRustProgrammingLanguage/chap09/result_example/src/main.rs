
use std::io;
use std::io::Read;
use std::fs::File;
use std::fs;

// 这个函数的返回值的类型为Result<T, E>，其中的泛型参数T被替换为具体的String类型，
// 而泛型E则被替换为具体的io::Error类型。
fn read_username_from_file(file : &String) -> Result<String, io::Error> {
    let f = File::open(file);

    let mut f = match f {
        Ok(file) => file,           // File::open运行成功，我们就将生成的文件句柄存储到变量f中并继续执行下一步
        Err(e) => return Err(e),    // 在Err情况下提前将File::open产生的错误作为结果返回
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),     // 当这个函数顺利运行时，调用这个函数的代码将会获得一个包裹在Ok中的String值，也就是这个函数从文件中读取的用户名。
        Err(e) => Err(e),   // 失败将这个错误值作为结果返回
    }
}

// 传播错误的快捷方式：?运算符, ?运算符只能被用于返回Result的函数
fn read_username_from_file_2(file : &String) -> Result<String, io::Error> {
    let mut s = String::new();

    // 如果出现了错误，?就会提前结束整个函数的执行，并将任何可能的Err值返回给函数调用者。
    let mut f = File::open(file)?.read_to_string(&mut s)?;

    Ok(s)
}


// 使用fs::read_to_string读取文件
fn read_username_from_file_3(file : &String) -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}


fn main() {
    let file = String::from("run");
    match read_username_from_file(&file) {
        Ok(name) =>  println!("name : {}", name),
        Err(e) => println!("error : {}", e),
    }
}

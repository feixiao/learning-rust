
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file(file : &String) -> Result<String, io::Error> {
    let f = File::open(file);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s), // 当这个函数顺利运行时，调用这个函数的代码将会获得一个包裹在Ok中的String值，也就是这个函数从文件中读取的用户名。
        Err(e) => Err(e),
    }
}


fn main() {
    let file = String::from("run");
    match read_username_from_file(&file) {
        Ok(name) =>  println!("name : {}", name),
        Err(e) => println!("error : {}", e),
    }
}

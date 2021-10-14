
use std::env;
use std::fs;


// fn parse_config(args: &[String] ) -> (&str, &str) {
//     let query = &args[1];
//     let filename = &args[2];
//     (query, filename)
// }

fn main() {
    // 将命令行参数收集到一个动态数组中并打印出来
    let args : Vec<String> = env::args().collect();
    println!("{:?}", args);

    // 创建变量来存储查询参数和文件名参数
    let query = &args[1];
    let  filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);


    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n {}", contents);
}

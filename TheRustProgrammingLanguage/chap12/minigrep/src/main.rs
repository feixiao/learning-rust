use std::cmp::min;
use std::env;
use std::fs;
use std::process;

use minigrep::Config;

// fn parse_config(args: &[String] ) -> (&str, &str) {
//     let query = &args[1];
//     let filename = &args[2];
//     (query, filename)
// }


// struct Config {
//     query: String,
//     filename: String,
// }
//
// impl Config {
//     fn new(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }
//         let query = args[1].clone();
//         let filename = args[2].clone();
//         Ok(Config { query, filename })
//     }
// }




fn main() {
    // 将命令行参数收集到一个动态数组中并打印出来
    let args : Vec<String> = env::args().collect();
    println!("{:?}", args);


    let args: Vec<String> = env::args().collect();

    // let config = Config::new(&args).unwrap_or_else(| err| {
    //     println!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });

    // 使用unwrap_or_else可以让我们执行一些自定义的且不会产生panic! 的错误处理策略。
    // 当值为Err时，这个方法则会调用闭包 （closure）中编写的代码，也就是我们定义出来并通过参数传入unwrap_or_else的这个匿名函数❸
    let config = minigrep::Config::new(&args).unwrap_or_else(| err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

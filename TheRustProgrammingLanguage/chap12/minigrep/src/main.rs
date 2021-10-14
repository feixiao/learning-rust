use std::cmp::min;
use std::env;
use std::fs;
use std::process;

// use minigrep::Config;

// fn parse_config(args: &[String] ) -> (&str, &str) {
//     let query = &args[1];
//     let filename = &args[2];
//     (query, filename)
// }


struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}




fn main() {
    // 将命令行参数收集到一个动态数组中并打印出来
    let args : Vec<String> = env::args().collect();
    println!("{:?}", args);


    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(| err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    // if let Err(e) = minigrep::run(config) {
    //
    // }
}

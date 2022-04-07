use std::error::Error;
use std::fs;


pub struct Config {
    pub query: String,
    pub filename: String,
}


impl Config {
    // 我们在第10章的“静态生命周期”一节中曾经指出&'static str是字符串字面量的类型，这也正是我们目前使用的错误提示信息类型
    pub fn new(args: &[String]) -> Result<Config, &'static str>  {

        // 改进错误提示信息
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// 只需要知道Box<dyn Error>意味着函数会返回一个实现了Error trait的类型，
// 但我们并不需要指定具体的类型是什么
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);
    Ok(())
}
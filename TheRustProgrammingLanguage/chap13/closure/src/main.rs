use std::thread;
use std::time::Duration;


// 一个用来代替假设计算的函数，它大约会执行2秒钟
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}


fn generate_workout(intensity: u32, random_number: u32) {
    // 为了定义闭包，我们需要以一对竖线（|）开始，并在竖线之间填写闭包的参数
    // 在参数后面，我们使用了一对花括号来包裹闭包的函数体。如果这个闭包只是单行表达式，你也可以选择省略花括号。

    // 和fn定义的函数不同，闭包并不强制要求你标注参数和返回值的类型。
    // 愿意为了明确性而接受不必要的繁杂作为代价，那么你仍然可以为闭包手动添加类型标注
    // let expensive_closure = |num: u32| -> u32也可以
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

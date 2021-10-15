use std::thread;
use std::time::Duration;


fn main() {
    // thread::spawn函数来创建线程，它接收一个闭包作为参数，该闭包会包含我们想要在新线程中运行的代码
    thread::spawn(||{
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });



    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 使用join句柄等待所有线程结束
    handle.join().unwrap();


    // 使用move关键字来强制闭包获得它所需值的所有权
    let v = vec![1, 2, 3];
    // 通过将v的所有权转移给新线程，我们就可以向Rust保证主线程不会再次使用v。
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}

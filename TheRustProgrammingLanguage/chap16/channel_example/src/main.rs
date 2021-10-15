use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    // 创建一个通道，并将两端分别赋给tx和rx
    let (tx, rx) = mpsc::channel();

    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),String::from("thread"),
    ];

    thread::spawn(move || {
        // 将tx移动到新线程中并发送值
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 在主线程中接收并打印值
    for received in rx {
        println!("Got: {}", received);
    }
}

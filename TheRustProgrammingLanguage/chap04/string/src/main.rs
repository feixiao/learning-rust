fn main() {
    let s1 = String::from("hello");

    // 试图在s2创建完毕后使用s1会导致编译时错误。
    // Rust永远不会自动地创建数据的深度拷贝。
    // let s2 = s1;

    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    // println!("{}, world!", s1);
}

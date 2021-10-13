
enum SpreadsheetCell {
   Int(i32),
   Float(f64),
   Text(String),
}


fn main() {
   // 创建一个用来存储i32数据的空动态数组
   let v:Vec<i32> = Vec::new();
   println!("vector {:?}", v);


   // Rust特意提供了一个用于简化代码的vec!
   // 这个宏可以根据我们提供的值来创建一个新的动态数组。
   let v = vec![1, 2, 3];
   println!("vector {:?}", v);


   // 使用push方法将值添加到动态数组中
   let mut v = Vec::new();
   v.push(5);
   v.push(6);
   v.push(7);
   v.push(8);
   println!("vector {:?}", v);


   // 访问动态数组的方式，它们分别是使用索引和get方法
   let v = vec![1, 2, 3, 4, 5];
   let third: &i32 = &v[2];   // 使用索引方式
   println!("The third element is {}", third);
   match v.get(2) {  // get方法,会返回一个Option<&T>。
      Some(third) => println!("The third element is {}", third),
      None => println!("There is no third element."),
   }


   // 遍历动态数组中的值
   let v = vec![100, 32, 57];
   for i in &v {
      println!("{}", i);
   }


   // 在动态数组中使用定义的枚举来存储不同类型的值
   let row = vec![
      SpreadsheetCell::Int(3),
      SpreadsheetCell::Text(String::from("blue")),
      SpreadsheetCell::Float(10.12),
   ];
}

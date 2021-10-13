use std::collections::HashMap;


fn main() {
    let mut scores = HashMap::new();

    // 创建一个新的哈希映射并插入一些键值对
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("scores {:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // 类型标记HashMap<_, _>不能被省略，因为collect可以作用于许多不同的数据结构，
    // 如果不指明类型的话，Rust就无法知道我们具体想要的类型。
    let scores: HashMap<_, _> =
        teams.iter().zip(initial_scores.iter()).collect();

    println!("scores {:?}", scores);


    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // filed_name和field_value从这一刻开始失效，若尝试使用它们则会导致编译错误

    // Entry的or_insert方法被定义为返回一个Entry键所指向值的可变引用，
    // 假如这个值不存在，就将参数作为新值插入哈希映射中，并把这个新值的可变引用返回。
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    // 访问存储在哈希映射中的蓝队分数
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match  { }
    // println!("score = {}", score);
}

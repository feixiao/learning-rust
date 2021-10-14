

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("read more")       // Summary带默认实现
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}


// 为NewsArticle实现Summary
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// 使用trait作为参数, 语法糖 等价下面的函数
// pub fn notify(item: impl Summary + Display)  通过+语法来指定多个trait约束
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// pub fn notify<T: Summary>(item: T) {
//     println!("Breaking news! {}", item.summarize());
// }



fn main() {

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Ice burgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    println!("New article available! {}", article.summarize());

    notify(article);
}

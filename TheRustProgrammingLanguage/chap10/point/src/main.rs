
// Rust实现泛型的方式决定了使用泛型的代码与使用具体类型的代码相比不会有任何速度上的差异
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mix_up<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    println!("p1.x = {}, p1.y = {}", p1.x, p1.y);

    let p2 = Point { x: "Hello", y: 'c'};
    let p3 = p1.mix_up(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

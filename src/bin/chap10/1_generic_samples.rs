#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt() }
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}
// this demonstrate a situation in which some generic parameters are
// declared with and some are declared with the method de nition, pp. 199
impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    println!("integer: {:?}", integer);
    let float = Point { x: 1.0, y: 4.0 };
    println!("float: {:?}", float);

    let both_integer = Point2 { x: 5, y: 10 };
    println!("both_integer: {:?}", both_integer);
    let both_float = Point2 { x: 1.0, y: 4.0 };
    println!("both_float: {:?}", both_float);
    let integer_and_float = Point2 { x: 5, y: 4.0 };
    println!("integer_and_float: {:?}", integer_and_float);

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
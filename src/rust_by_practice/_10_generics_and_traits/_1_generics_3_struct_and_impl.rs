
// Implement struct Point to make it work.
struct Point<T> {
    x: T,
    y: T,
}

pub fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("Success!");
}

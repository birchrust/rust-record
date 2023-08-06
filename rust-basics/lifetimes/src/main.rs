use std::ops::Add;
use std::time::Duration;

fn main() {
    let floats = add(1.2, 1.2);
    let ints = add(10, 20);

    let duration = add(Duration::new(5, 0), Duration::new(10, 0));

    println!("{:?} {} {}", duration, floats, ints);
}

fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

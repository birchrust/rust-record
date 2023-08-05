use std::ops::Add;
use std::time::Duration;

fn main() {
    let floats = add(1.2, 3.4);
    let ints = add(10, 20);

    let durations = add(Duration::new(5, 0), Duration::new(10, 0));

    // {}调用的是 std::Display trait，{:?}调用
    println!("floats:{},ints:{},durations:{:?}", floats, ints, durations);
}

fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

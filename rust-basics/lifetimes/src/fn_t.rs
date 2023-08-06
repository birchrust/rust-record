//  T必须实现 std::ops::Add<Output = T> trait
fn add_t<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

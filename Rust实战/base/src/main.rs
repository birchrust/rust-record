fn main() {
    greet_word();
}

fn greet_word() {
    let chinese = "你好，世界";
    let english = "hello, world";
    let regions = [chinese, english];
    for region in regions.iter() {
        println!("{}", &region); // &表示“借用”regoin的值，用于只读的访问
    }
}

fn main() {
    another_function(5, 6.1);
}

// 通过 ; 结尾的表达式返回一个 ()
fn another_function(x: i32, y: f32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// 当用 ! 作函数返回类型的时候，表示该函数永不返回( diverge function )
fn dead_end() -> ! {
    panic!("你已经到了穷途末路，崩溃吧！");
}

mod fn_t;

fn main() {
    // 变量解构
    let (a, b) = (10, 20);

    let res = add_with_lifetimes(&a, &b);
    println!("结果:{}", res);
}

//  生命周期
fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    // 解引用，返回 i+j
    *i + *j
}

fn main() {
    // Rust 的变量在默认情况下是不可变的
    // 可变性很简单，只要在变量名前加一个 mut 即可
    let mut x = 5;
    x -= 4;
    println!("The value of x is: {}", x);

    //使用下划线开头忽略未使用的变量
    let _y = 10;

    // let 表达式不仅仅用于变量的绑定，还能进行复杂变量的解构：从一个相对复杂的变量中，匹配出该变量的一部分内容：
    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);
    b = true;
    assert_eq!(a, b); // 判断是否相等

    let (e, f, g, h);
    (f, g) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [h, .., _, e] = [3, 4, 5, 6];

    assert_eq!([1, 2, 3, 6], [f, g, h, e]);
}

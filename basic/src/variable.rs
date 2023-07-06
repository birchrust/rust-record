struct Struct {
    g: i32,
}

fn variable() {
    println!("Hello, world!");

    let x = 5;
    println!("The value of x is: {}", x);

    // 不使用的变量需要加下划线
    let _y = "hi,birch";
    // 变量结构
    let (a, mut b): (bool, bool) = (true, true);
    println!("a = {:?}, b = {:?}", a, b);
    b = true;
    assert_eq!(a, b);

    let (c, d, e, f, g);
    (c, d) = (1, 2);
    [e, .., f, _] = [1, 2, 3, 4, 5];
    Struct { g, .. } = Struct { g: 5 };
    assert_eq!([1, 2, 1, 4, 5], [c, d, e, f, g]);

    // 常量的命名约定是全部字母都使用大写，并使用下划线分隔单词
    const _MAX_POINS: u32 = 100_000;
}

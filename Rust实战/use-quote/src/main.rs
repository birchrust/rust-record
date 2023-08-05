fn main() {
    let a = 42;
    let r = &a;
    let b = a + *r;
    println!("{},{}", b, a); //实现 a 与 a 相加,并将结果赋值给b

    find_number();
}

fn find_number() {
    let needle = 0o204;
    let haystack = [1, 1, 2, 5, 15, 132, 877, 4140, 21147];

    for i in &haystack {
        if *i == needle {
            println!("{}", i);
        }
    }
}

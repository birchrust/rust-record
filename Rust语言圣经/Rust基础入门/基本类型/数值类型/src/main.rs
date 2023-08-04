use num::complex::Complex;
fn main() {
    // 整型溢出
    let a: u8 = 255;
    // 使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理
    let b = a.wrapping_add(20);
    println!("{}", b); // 19

    // 防御性编程，判断一个数值是否是 NaN
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为");
    }

    // 生成从 1 到 5 的连续数字，包含 5
    for i in 1..=5 {
        println!("{}", i);
    }

    // 有理数和复数
    let c = Complex { re: 2.1, im: -1.2 };
    let d = Complex::new(11.1, 22.2);
    let result = c + d;
    println!("{} + {}i", result.re, result.im);

    // 取整
    let e = (13.14_f32).round();
    println!("13.14取整：{}", e);
}

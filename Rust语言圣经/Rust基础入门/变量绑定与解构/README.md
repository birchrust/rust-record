#### [章节](https://course.rs/basic/variable.html)🦀	|	[练习实践](https://zh.practice.rs/variables.html):pencil2:

------

#### 绑定和可**变性**

🌟 变量只有在初始化后才能被使用

```rust
// 修复下面代码的错误并尽可能少的修改
fn main() {
    let x: i32; // 未初始化，但被使用
    let y: i32; // 未初始化，也未被使用
    println!("x is equal to {}", x); 
}
```

```rust
fn main() {
    let x =0_i32; // 在数值和类型中间添加一个下划线，让可读性更好
    let _y: i32; // 未初始化，也未被使用
    // println! 会自动推导出具体的类型，因此无需手动指定
    // 字符串使用双引号 "" 而不是单引号 ''
    println!("x is equal to {}", x);  // Rust 使用 {} 来作为格式化输出占位符
}

```

🌟🌟 可以使用 `mut` 将变量标记为可变

```rust
// 完形填空，让代码编译
fn main() {
    let __ = 1;
    __ += 2; 
    
    println!("x = {}", x); 
}
```

```rust
fn main() {
    let mut x = 1;
    x += 2; 
    
    println!("x = {}", x); 
}
```

#### 变量作用域

🌟 作用域是一个变量在程序中能够保持合法的范围

```rust
// 修复下面代码的错误并使用尽可能少的改变
fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("x 的值是 {}, y 的值是 {}", x, y);
    }
    println!("x 的值是 {}, y 的值是 {}", x, y); 
}
```


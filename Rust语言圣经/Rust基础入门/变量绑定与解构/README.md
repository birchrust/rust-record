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

```rust
fn main() {
    let x: i32 = 10;
    {
        let x = x;
        let y: i32 = 5;
        println!("x 的值是 {}, y 的值是 {}", x, y);
    }
    println!("x 的值是 {}", x);
}
```

🌟🌟

```rust
// 修复错误
fn main() {
    println!("{}, world", x); 
}

fn define_x() {
    let x = "hello";
}
```

```rust
fn main() {
    let x = define_x(); // 接收返回值
    println!("{}, world", x);
}

// 声明返回类型为string
fn define_x() -> String {
    let x = "hello".to_string(); // 使x的类型为String
    x
}
```

#### 变量遮蔽(Shadowing)

🌟🌟 若后面的变量声明的名称和之前的变量相同，则我们说：第一个变量被第二个同名变量遮蔽了( shadowing )

```rust
// 只允许修改 `assert_eq!` 来让 `println!` 工作(在终端输出 `42`)
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 5);
    }

    assert_eq!(x, 12);

    let x = 42;
    println!("{}", x); // 输出 "42".
}
```

```rust
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // 输出 "42".
}
```

🌟🌟 修改一行代码以通过编译

```rust
fn main() {
    let mut x: i32 = 1;
    x = 7;
    // 遮蔽且再次绑定
    let x = x; 
    x += 3;


    let y = 4;
    // 遮蔽
    let y = "I can also be bound to text!"; 
    println!("Success!");
}
```

```rust
fn main() {
    let mut x: i32 = 1;
    x = 7;
    // 遮蔽且再次绑定
    let mut x = x; 
    x += 3;


    let y = 4;
    // 遮蔽
    let y = "I can also be bound to text!"; 
    println!("Success!");
}
```

#### 未使用的变量

使用以下方法来修复编译器输出的 warning :

- 🌟 一种方法
- 🌟🌟 两种方法

注意: 你可以使用两种方法解决，但是它们没有一种是移除 `let x = 1` 所在的代码行

```rust
fn main() {
    let x = 1; 
}

// compiler warning: unused variable: `x`
```

```rust
// 方法一
fn main() {
    let _x = 1; 
}

// 方法二
#[allow(unused_variables)]
fn main() {
    let x = 1;
}
```

#### 变量解构

🌟🌟 我们可以将 `let` 跟一个模式一起使用来解构一个元组，最终将它解构为多个独立的变量

```rust
// 修复下面代码的错误并尽可能少的修改
fn main() {
    let (x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
    println!("Success!");
}
```

```rust
// 方法一
fn main() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
    println!("Success!");
}

// 方法二
fn main() {
    let (x, y) = (1, 2);
    let x = 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
    println!("Success!");
}
```

#### 解构式赋值

Note: 解构式赋值只能在 Rust 1.59 或者更高版本中使用

```rust
fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // 填空，让代码工作
    assert_eq!([x,y], __);
} 
```

```rust
fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // 填空，让代码工作
    assert_eq!([x,y], [3,2]);
}
```

------

- [x] 打卡：2023.8.4

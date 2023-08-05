#### 所有权

栈和堆的核心目标就是为程序在运行时提供可供使用的内存空间。

##### 栈

栈按照顺序存储值并以相反顺序取出值，这也被称作**后进先出**。栈中的所有数据都必须占用已知且固定大小的内存空间。

##### 堆

对于大小未知或者可能变化的数据，我们需要将它存储在堆上。当向堆上放入数据时，返回一个表示该位置地址的**指针**, 该过程被称为**在堆上分配内存**，你将通过栈中的**指针**，来获取数据在堆上的实际内存位置，进而访问该数据。

##### 性能区别

处理器处理分配在栈上数据会比在堆上的数据更加高效。

#### 所以权原则

1. Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
2. 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
3. 当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)

------

🌟🌟

```rust
fn main() {
    // 使用尽可能多的方法来通过编译
    let x = String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);
}
```

```rust
fn main() {
    let x = &String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);
}
```

🌟🌟

```rust
// 不要修改 main 中的代码
fn main() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// 只能修改下面的代码!
fn take_ownership(s: String) {
    println!("{}", s);
}
```

```rust
fn main() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}
```

🌟🌟

```rust
// 修复错误，不要删除任何代码行
fn main() {
    let s = String::from("hello, world");

    print_str(s);

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}
```

```rust
fn main() {
    let s = String::from("hello, world");

    print_str(s.clone()); // 拷贝
    
    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}
```

```rust
 fn main() {
     let s = String::from("hello, world");
     print_str(&s);	// 传引用值
     println!("{}", s);
 }
 fn print_str(s: &String)  {
     println!("{}",s)
 }
```

🌟🌟

```rust
// 不要使用 clone，使用 copy 的方式替代
fn main() {
    let x = (1, 2, (), "hello".to_string());
    let y = x.clone();
    println!("{:?}, {:?}", x, y);
}
```

```rust
fn main() {
    let x = (1, 2, (), "hello"); // 基础类型是引用
    let y = x; // 浅拷贝
    println!("{:?}, {:?}", x, y);
}
```

#### 可变性

🌟🌟

```rust
fn main() {
    let s = String::from("hello, ");
    
    // 只修改下面这行代码 !
    let s1 = s;

    s1.push_str("world")
}
```

```rust
fn main() {
    let s = String::from("hello, ");
    let mut s1 = s;	// 添加mut关键字才能是可变变量

    s1.push_str("world")
}
```

🌟

```rust

```

🌟🌟🌟

```rust

```

🌟

```rust

```

🌟🌟

```rust

```


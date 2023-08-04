#### 字符

🌟

```rust
// 修改2处 `assert_eq!` 让代码工作

use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),1); 

    let c2 = '中';
    assert_eq!(size_of_val(&c2),3); 

    println!("Success!")
} 

```

```rust
use std::mem::size_of_val;
fn main() {
    let c1 = 'a'; // 字符类型是占用 4 个字节
    assert_eq!(size_of_val(&c1),4); 

    let c2 = '中'; // 字符类型是占用 4 个字节
    assert_eq!(size_of_val(&c2),4); 

    println!("Success!")
}
```

🌟

```rust
fn main() {
    let c1 = "中";
    print_char(c1);
} 

fn print_char(c : char) {
    println!("{}", c);
}
```

```rust
fn main() {
    let c1:char = '中'; // 字符只能用''
    print_char(c1);
}

fn print_char(c: char) {
    println!("{}", c);
}
```

#### 布尔

🌟

```rust
// 使成功打印
fn main() {
    let _f: bool = false;

    let t = true;
    if !t {
        println!("Success!")
    }
} 
```

```rust
fn main() {
    let _f: bool = false;

    let t = false;
    if !t {
        println!("hello, world");
    }
} 
```

🌟

```rust
fn main() {
    let f = true;
    let t = true && false; // & 位与：相同位置均为1时则为1，否则为0
    assert_eq!(t, f);

    println!("Success!")
}
```

```rust
fn main() {
    let f = true;
    let t = true || false; // | 位或：相同位置只要有1时则为1，否则为0
    assert_eq!(t, f);

    println!("Success!")
}
```

#### 单元类型

🌟🌟

```rust
// 让代码工作，但不要修改 `implicitly_ret_unit` !
fn main() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(v, implicitly_ret_unit());

    println!("Success!")
}

fn implicitly_ret_unit() {
    println!("I will return a ()")
}

// 不要使用下面的函数，它只用于演示！
fn explicitly_ret_unit() -> () {
    println!("I will return a ()")
}
```

```rust
// 让代码工作，但不要修改 `implicitly_ret_unit` !
fn main() {
    let v0: () = ();

    let v = (2, 3);
    assert_eq!(v0, implicitly_ret_unit()); // 两个()比较

    println!("Success!")
}

fn implicitly_ret_unit() {
    println!("I will return a ()")
}

// 不要使用下面的函数，它只用于演示！
fn explicitly_ret_unit() -> () {
    println!("I will return a ()")
}
```

🌟🌟单元类型占用的内存大小是多少？

```rust
// 让代码工作：修改 `assert!` 中的 `4` 
use std::mem::size_of_val;
fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 4);

    println!("Success!")
}
```

```rust
use std::mem::size_of_val;
fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 4); // 单元类型不占任何空间

    println!("Success!")
}
```


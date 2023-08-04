#### 语句和表达式

- **语句会执行一些操作但是不会返回一个值**.
- **表达式会在求值后返回一个值**, **表达式不能包含分号**
- **表达式如何不返回任何值，会隐式地返回一个（）**

🌟🌟

```rust
// 使用两种方法让代码工作起来
fn main() {
   let v = {
       let mut x = 1;
       x += 2
   };

   assert_eq!(v, 3);
}
```

```rust
// 方法一
fn main() {
   let v = {
       let mut x = 1;
       x += 2	// x += 2 仅仅是x做了运算，并不是返回x运算后的值
   };

   assert_eq!(v, ());
}

// 方法二
fn main() {
   let v = {
       let mut x = 1;
       x += 2; // 需要添加分号
       x	// 返回x
   };

   assert_eq!(v, 3);
}
```

🌟

```rust
fn main() {
   let v = (let x = 3);

   assert!(v == 3);
}
```

```rust
fn main() {
    let v = {
        let x = 3;
        x
    };
 
    assert!(v == 3);
}
```

🌟

```rust
fn main() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y;
}
```

```rust
fn main() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y	// 返回值没有;
}
```


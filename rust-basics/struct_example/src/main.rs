#[derive(Debug)]
struct Reactangle {
    width: u32,
    height: u32,
} // 一个struct可以有多个实现块

impl Reactangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Reactangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Reactangle {
    // 关联函数
    fn square(size: u32) -> Reactangle {
        Reactangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect0 = Reactangle {
        width: 20,
        height: 30,
    };
    let rect1 = Reactangle {
        width: 30,
        height: 40,
    };
    let rect2 = Reactangle {
        width: 50,
        height: 60,
    };

    let res = rect2.can_hold(&rect1);

    println!("{}", res);
    println!("rect area: {:#?}", rect0.area()); // #：结构化输出
    println!("square: {:#?}", Reactangle::square(20));
}

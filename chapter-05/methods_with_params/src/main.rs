#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width < self.width && other.height < self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 28,
        height: 52,
    };

    println!(
        "The are of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "rect1 can hold rect2: {}.",
        rect1.can_hold(&rect2)
    );

    println!(
        "rect1 can hold rect3: {}.",
        rect1.can_hold(&rect3)
    );
}


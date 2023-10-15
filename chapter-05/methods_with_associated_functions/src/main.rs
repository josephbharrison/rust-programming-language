#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width < self.width && other.height < self.height
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 50);

    let rect2 = Rectangle::new(20, 40);

    let rect3 = Rectangle::new(28, 52);

    println!(
        "The are of rect1 is {} square pixels.",
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


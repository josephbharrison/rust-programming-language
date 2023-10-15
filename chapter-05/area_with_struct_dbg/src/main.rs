#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!(
        "The are of the rectangle is {} pixels.",
        area(&rect1)
    );

    dbg!(&rect1);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

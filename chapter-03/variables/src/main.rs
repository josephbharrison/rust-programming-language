fn main() {
    // let x = 5; will error, vars immutable by default
    let mut x = 5; // make x mutable;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Data Types

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {guess}");

    // Floating points

    let x = 2.0;

    let y: f32 = 3.0;

    println!("{x} {y}");


}

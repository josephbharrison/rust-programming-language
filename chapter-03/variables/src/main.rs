fn main() {
    // let x = 5; will error, vars immutable by default
    let mut x = 5; // make x mutable;
    println!("The value of x is: (x)");
    x = 6;
    println!("The value of x is: (x)");
} 

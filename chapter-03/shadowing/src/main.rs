fn main() {
    let x = 5;
    
    let x = x + 1; // this x shadows the former

    {
        // inner scope x shadows outer scope
        // it does not overwrite out scope x
        // x declared in inner scope is distinct
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

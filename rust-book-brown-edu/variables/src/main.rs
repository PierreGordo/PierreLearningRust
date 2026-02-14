fn main() {
    let mut x = 5;
    println!("The value of x is: {x}.");
    x = 6;
    println!("The value of x is: {x}.");

    let y = 5;

    //shadowing
    let y = y + 1;

    //inner scope
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}.");
    }

    println!("The value of y in the outer scope is: {y}.");
}

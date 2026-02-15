use std::cmp::Ordering;

fn main() {
    check_number(255);

    //If - let
    let condition = true;

    let number = if condition { 5 } else { 6 };
    println!("The number is {number}.");
}

fn check_number(num: i32) {
    /*
        if num < 5 {
            println!("Number is less than 5.");
        } else {
            println!("Number is not less than five.");
        }
    */

    //I did this just to experiment with the match, this was not part of the chapter section
    match num.cmp(&5) {
        Ordering::Less => println!("The number is less than 5"),
        Ordering::Equal => println!("The number is equal to 5"),
        Ordering::Greater => println!("The number is more than 5"),
    };
}

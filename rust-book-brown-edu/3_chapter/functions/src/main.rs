fn main() {
    print_labeled_measurement(6, 'm');

    express();

    let x = five();

    println!("Function five() returned the value: {x}");

    let x = plus_one(x);

    println!("The value of x after running the plus_one(x) function is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

//Expressions
fn express() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {y}.");
}

//Return function
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

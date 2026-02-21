use std::env;

fn main() {
    //Command line arguments are here so I can build it as release and try out multiple values
    let args: Vec<String> = env::args().collect();

    let digit: u32;

    match args.len() {
        2 => {
            //first argument is path, second should be the digit we want to reach
            match args[1].trim().parse::<u32>() {
                Ok(num) => digit = num,
                Err(_) => {
                    println!("You have to provide a digit.");
                    return;
                }
            }
        }
        _ => {
            println!("Invalid number of arguments were provided.");
            return;
        }
    }

    let mut a: u32;
    let mut b: u32 = 1;
    let mut c: u32 = 1;

    if digit > 3 {
        for _ in 0..(digit - 3) {
            // its digit -3 because i already have 3
            a = c;
            c = c + b;
            b = a;
        }
    }

    println!("The {digit} digit is {c}");
}

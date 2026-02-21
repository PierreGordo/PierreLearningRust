use std::env;

fn main() {
    //Getting the command line arguments
    //I will have two possibilities: --f-to-c and --c-to-f and after that a number
    let args: Vec<String> = env::args().collect();
    //Check that there is the correct number of arguments
    //Its 3 because the first is the path to the thing
    match args.len() {
        3 => {
            match args[1].as_str() {
                "--f-to-c" => {
                    //Converting from fahrenheit to celsius
                    match args[2].trim().parse::<i32>() {
                        Ok(temp) => {
                            //Unfortunately here, there is nothing I can do to make it more precise (at least right now, that is easy) because 5/9 is irrational.
                            println!(
                                "{temp} F is {} C",
                                (temp - 32) as f64 * 0.5555555555555555555555
                            );
                        }
                        Err(_) => println!(
                            "The conversion failed, you probably entered an invalid number."
                        ),
                    }
                }
                "--c-to-f" => {
                    //Converting from celsius to fahrenheit
                    match args[2].trim().parse::<i32>() {
                        Ok(temp) => {
                            println!("{temp} C is {} F", (temp as f64 * 1.8) + 32.0);
                        }
                        Err(_) => println!(
                            "The conversion failed, you probably entered an invalid number."
                        ),
                    }
                }
                _ => println!("Invalid command {}", args[1]),
            }
        }
        _ => {
            println!("An incorrect amount of arguments was provided.");
            return;
        }
    }
}

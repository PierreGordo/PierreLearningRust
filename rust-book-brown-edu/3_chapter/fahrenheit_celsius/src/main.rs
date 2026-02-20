use std::io;

fn f_to_c(fah: i32) {
    println!("{fah} fahrenheit is {} celsius", (fah - 32) / (9 / 5))
}

fn main() {
    loop {
        println!("If you want to convert from fahrenheit to celsius, type 1, if you want to convert from celsius to fahrenheit type 2");

        let mut f_or_c = String::new();

        io::stdin()
            .read_line(&mut f_or_c)
            .expect("Failed to read line.");

        let f_or_c: i32 = match f_or_c.trim().parse() {
            Ok(num) => num,
            Err(_) => prinln!("You have to put a number in..."),
        };
    }
}

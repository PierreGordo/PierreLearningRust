fn main() {
    let x = 2.0; //Default type for floating point is f64

    let y: f32 = 3.0;

    //addition
    let sum = 5 + 10;

    //substraction
    let difference = 95.5 - 4.3;

    //multiplication
    let product = 4 * 30;

    //division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; //when dividing in i32 it goes to the nearest number in this case its -1

    //remainder
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false; //not necesary -> compiler will infer the type

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (a, b, c) = tup; //destructuring
    println!("The first value from the tuple is {a}");

    let b = tup.1; //shadowing, but i am assigning it the same value, that it already has
    println!("The second value from the tuple is {b}");

    //mutable tuple, so I can modify the values
    let mut tup: (i32, i32) = (1, 2);
    tup.0 = 0;
    tup.1 += 5;

    //arrays - fixed lenght in Rust
    let arr = [1, 2, 3, 4, 5]; //type is inferred, but has to be the same for all elements

    let arr: [i32; 3] = [1, 2, 3]; //type is specified - 3 elements that are i32

    let arr = [3; 5]; //this array contains 5 elements, all of them are the number 3

    let a = arr[0]; //this uses the [] to acess the elements by index
    let b = arr[4];
    println!("The arrays first element is {a} and last element is {b}.");
}

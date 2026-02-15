fn main() {
    /*
        loop {
            println!("again!");
        }
    */

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    //Multiple loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    //Conditional while loops
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }
    println!("LIFTOFF!!!");

    //Looping through a collection
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    //Using this approach is error prone
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    //Better to use a for loop
    for element in a {
        println!("the value is: {element}");
    }
    //Countown using a for loop
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}

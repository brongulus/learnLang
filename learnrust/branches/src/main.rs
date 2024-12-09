fn main() {
    let number = 6;

    // Unlike languages such as Ruby and JavaScript, Rust will not
    // automatically try to convert non-Boolean types to a Boolean.  You must
    // be explicit and always provide ‘if’ with a Boolean as its condition.
    if number < 5 {
        println!("condition was true");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("condition was false");
    }

    // let number = if true { 5 } else { "six" }; // If-else arms should have compatible values!

    // 'loop' (break, add value to be returned from break: break <expr>), 'while' and 'for'
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }; // let result = <val>; <-- statement requires semi-colon
    println!("The result is {result}");

    counter = 0;

    // outer loop is labelled, break knows which loop to exit
    'counting_up: loop {
        println!("count = {counter}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up; // <-- break from the labelled loop
            }
            remaining -= 1;
        }

        counter += 1;
    }
    println!("End count = {counter}");

    while counter != 0 {
        println!("{counter}");
        counter -= 1;
    }

    println!("LIFTOFF!!!");

    let arr = [10, 20, 30, 40, 50];

    for element in arr {
        println!("the value is: {element}");
    }

    for number in (2..5).rev() {
        println!("{number}");
    }
    println!("LIFTON!!!");
}

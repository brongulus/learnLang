use std::io;

fn main() {
    loop {
        println!("Choose one:");
        println!("1. Temperature conversion");
        println!("2. Fibonacci number");
        println!("3. Christmas Carol");

        let input = read_int();

        match input {
            1 => {
                println!("Enter celsius temperature: ");
                let temp = read_int();
                convert_temp(temp);
            }
            2 => {
                fibonacci();
            }
            3 => {
                carol();
            }
            _ => {
                println!("Please enter valid input (1-3)");
                break;
            }
        }
    }
}

fn read_int() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("String");
    input.trim().parse().expect("Input not an integer")
}

fn convert_temp(temp: i32) {
    const MUL_FACTOR: f32 = 9.0 / 5.0;
    const CONST: u32 = 32;
    let farhenheit_temp = temp as f32 * MUL_FACTOR + CONST as f32;
    println!("The temperature in Farhenheit is {farhenheit_temp}");
}

fn fibonacci() {
    println!("Enter the fibonacci index");
    let n = read_int();
    let (mut first, mut second): (u64, u64) = (0, 1);
    for _ in 1..n {
        let temp = second;
        second += first;
        first = temp;
    }
    if n <= 0 {
        println!("{n}th fibonacci is {first}");
        return;
    }
    println!("{n}th fibonacci is {second}");
}

fn carol() {
    // Ref: https://www.openmymind.net/Rust-Strings/
    fn repeating_line(index: i32) {
        let day = match index {
            1 => "first",
            2 => "second",
            3 => "third",
            _ => "",
        };
        println!("\nOn the {day} day of Christmas my true love sent to me")
    }

    let mut recursive_line = String::from("A patridge in a pear tree.");
    for i in 1..=3 {
        repeating_line(i);
        match i {
            1 => {}
            2 => {
                recursive_line = format!("Tow turtle doves,\nAnd {}", recursive_line);
            }
            3 => {
                recursive_line = "Three French hens,\n".to_owned() + &recursive_line;
            }
            _ => {
                continue;
            }
        }
        println!("{recursive_line}");
    }
    println!();
}

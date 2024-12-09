use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    // By default, Rust has a set of items defined in the standard library that
    // it brings into the scope of every program.  This set is called the  _prelude_

    let secret_number = rand::thread_rng().gen_range(1..=10);
    println!("Guess the number (between 1 and 10)! ({secret_number})");

    loop {
        println!("Please input your guess.");
        // Variables are immutable by default
        let mut guess = String::new();

        // Bring module (io here) in scope with 'use' statement.
        let _ = io::stdin()
            // like variables, references are immutable by default so '&mut guess' and not '&guess'
            .read_line(&mut guess)
            // read_line returns a 'Result' enum: <Ok, Err>, 'expect' handles Err result
            .expect("Failed to read line");

        // this guess shadows the previous one! parse() converts string to other types
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     // If parse results in Ok, then guess becomes num
            Err(_) => continue, // Else all other values _ are discarded and we run it back
        };

        // A ‘match’ expression is made up of ‘arms’.  An arm consists of a
        // ‘pattern’ to match against, and the code that should be run if the value
        // given to ‘match’ fits that arm's pattern.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }

    print!(
        "You guessed {secret_number} and the secret number was {}\n",
        secret_number
    );
}

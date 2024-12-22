#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    // Enums allow defining a type by enumerating its possible 'variants'
    // Each variant can have different types and amounts of associated data.
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    // Name of each enum variant that's defined also becomes a function
    // that constructs an instance of the enum: IpAddr::V4() constructor
    let four = IpAddr::V4(127, 0, 0, 1);
    let six = IpAddr::V6(String::from("::1"));

    impl IpAddr {
        fn route(&self) {
            println!("Routing {self:?}");
        }
    }

    four.route();
    six.route();

    // 'Match' expression allows comparing a value against a series
    // of patterns and executing code based on which pattern matches.
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            // Match arms must cover all possibilities (exhaustive)
            // `catch-all' final arm: 'other' when we want the passed value
            // or '_' in case we don't care about the passed value
            // in case we don't want to do anything we can return the unit value '()'
            // order of match arms matters since matching happens sequentially
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => {
                println!("A Dime is worth 10 cents.");
                10 // returns 10
            }
            Coin::Quarter => 25,
        }
    }

    value_in_cents(Coin::Dime);

    // 'Option' is a stdlib enum that encodes the case where a value
    // could be something or nothing
    // enum Option<T> { // <-- included in prelude
    //     None,        // <-- included in prelude
    //     Some(T),     // <-- included in prelude
    // }

    let some_number = Some(5);
    let absent_number: Option<i32> = None; // <-- type annotation needed

    // let sum = some_number + 5; // <-- errors can't add Option<integer> with i32
    // Need to convert Option<T> to 'T' before operations supported by 'T' can be done
    // This allows to catch non-null value assumption issues.
    let add_seven = match some_number {
        None => 7,
        Some(value) => value + 7,
    };

    println!("Adding 5 to {some_number:?} gives {add_seven}");

    // Or even simply, `unwrap_or' comes to our rescue.
    println!(
        "Adding 9 to {absent_number:?} gives {}",
        9 + absent_number.unwrap_or(0)
    );

    // 'if let' combines if and let into a less verbose way
    // to handle one match pattern while ignoring the rest.
    let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {max}"),
    //     _ => (),
    // }
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    // if let can have an else clause too that goes with the '_' case
}

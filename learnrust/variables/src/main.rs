fn main() {
    let x = 5;
    println!("The value of x is {x}");
    // x = 6;
    // println!("The value of x is {x}");

    // Constants should be upper snake_case and have explicit type annotation
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("You have {THREE_HOURS_IN_SECONDS} seconds to finish this backlog!");

    // Shadowing
    {
        // You can shadow x here irrespective of its mutability, this is essentially a new x
        let x = x * x;
        println!("Value of x in inner scope is {x}");
        let spaces = "   ";
        // A new spaces of int type
        let spaces = spaces.len();
        // In cases when many types are possible, such as when
        // we converted a ‘String’ to a numeric type using ‘parse’
        // We need to use a type annotation
        let guess: u32 = "42".parse().expect("Not a number!");
        println!("Value of spaces is {guess} and length is {spaces}")
    }
    println!("Value of x is {x}");

    // Scalars are ints (8-128 bit signed/unsigned, default i32), floats (f32, 'f64'), bools and chars
    // rustc checks for int overflow when compiling in debug mode only! to handle use 'wrapping_add'
    // instead of '+' and so on. (check manual for other ways: checked_*, overflowing_*, saturating_*)
    println!(
        "This is number literal with type suffix and visual separator {}",
        1_000u16 // unsigned 16-bit number with value 1000
    );
    let x: f32 = 2.3;
    let f: bool = true; // bools are one byte in size
    let c = '🏁'; // Chars are surrounded by single quotes, 4 bytes and represent unicode scalar value
    if f {
        println!("Here we have a float x {}, let's go {c}", x);
    }

    // Compound types consist of tuples (different types of values in one) and arrays (both fixed len)
    // Tuple without any values is a 'unit' with value and type both being (): empty
    let tup: (i32, f64, bool) = (500, 6.4, false);
    let (a, a_init) = ([1, 2, 3, 4, 5], [-1; 5]); // a_init = [init value; len]
    let (_, b, _) = tup; // destructuring the tuple in parts
    println!(
        "The value of the elements of {:?} are {}, {b} and {}",
        tup, tup.0, tup.2
    );
    println!(
        "The array consists of {:?}, the initialized array has {:?} all values being {}",
        a, a_init, a_init[0]
    ); // Vectors allow for dynamic size unlike array

    // • *Statements* are instructions that perform some action and do not
    //   return a value. e.g: let y = 6;
    //   let x = (let y = 6) <-- this is erraneous as (let y = 6) is a statement that doesnt return
    // • *Expressions* evaluate to a resultant value. e.g: 5 + 6
    //   Calling a function or a macro is an expression. A scope block is as well.
    another_function(a[2], c);
    println!("{}", func_that_returns());
}

// This function definition is a statement
fn another_function(x: i32, c: char) {
    println!("Value of third element in the array is {x} and the declared char was {c}");
}

fn func_that_returns() -> i32 {
    // Expressions do not include ending semicolons.  If
    // you add a semicolon to the end of an expression, you turn it into a
    // statement, and it will then not return a value.
    5 // Return value is value of final expression in function body
}

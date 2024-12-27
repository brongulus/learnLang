#[allow(dead_code)]
fn main() {
    // Unlike array and tuple types, data collections point
    // to is stored on the heap. 'Vector', 'String', 'Hash Map'
    let mut v: Vec<i32> = Vec::new(); // <-- type annotation needed since empty vector
                                      // so compiler doesn't know what type of elements to store
    {
        let v = vec![1, 2, 3]; // <-- Vec<i32>
    } // <-- this v goes out of scope and is freed here all its contents are dropped

    v.push(5);
    v.push(6);
    v.push(7);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2); // get returns Option<&T>
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // Accessing outside of range
    // let _does_not_exist = &v[5]; // <-- panics!
    let _does_not_exist = v.get(5); // returns None without panicking

    // Immutable and mutable borrow at the same time is not allowed
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // <-- immutable borrow

    // This causes issues because adding a new element might require
    // allocating new memory and copying the elements which make the
    // reference to the first element point to deallocated memory.
    // v.push(6); // <-- mutable borrow ERROR
    println!("The first element is {first}"); // immutable borrow used here

    for i in &v {
        print!("{i} "); // iterates through all elements of vector
    }

    for i in &mut v {
        // iterate over mutable refs to elements in mut vector
        *i += 50; // dereference to get the value
    }

    println!("\nThe values are now {v:?}");

    // To store a list of items of different types, a vector of enums can be used
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    // If one doesn't know the exhaustive set of types that are
    // needed enums won't work, then trait objects must be used.
}

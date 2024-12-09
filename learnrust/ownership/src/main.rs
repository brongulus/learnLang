// TODO: Check out https://www.openmymind.net/Rust-Ownership-Move-and-Borrow-part-1/
fn main() {
    // All data stored on the stack must have known, fixed size. Data with
    // an unknown size at compile time or a size that might change must be
    // stored on the heap instead. The pointer to the heap is a known, fixed
    // size object so it's stored on the stack whereas the actual data it
    // points to is stored on the heap.

    // Keeping track of what parts of code are using what data on the heap,
    // minimizing the amount of duplicate data on the heap, and cleaning up
    // unused data on the heap so you don't run out of space are all problems
    // that ownership addresses. Ownership rules:
    // • Each value in Rust has an ‘owner’.
    // • There can only be one owner at a time.
    // • When the owner goes out of scope, the value will be dropped.
    // A scope is the range within a program for which an item is valid.

    // In the case of a string literal, we know the contents at compile time, so the
    // text is hardcoded directly into the final executable. Hence they are immutable.
    // Rust has second string type, ‘String’. This type manages data allocated on the
    // heap and is able to store an amount of text that is unknown at compile time.
    {
        let mut s = String::from("Hello"); // Memory requested from allocator at runtime (s is valid now)
        s.push_str(", world!");

        println!("{s} from inside the {{}} scope");
    } // scope is over and s is no longer valid, memory is returned to the allocator ('drop' called)

    let x = 5; // bind x to value 5
    let y = x; // <-- bind y to copy of value in x which is 5

    // A string is made of 'pointer' to the memory having its contents, 'length' and 'capacity'
    let s1 = String::from("Hello");
    let mut s2 = s1; // <-- s2 "isn't" bound to copy of value in s1
    println!("{s2}, world! from s2");

    // Rather string data is copied i.e. 'pointer', 'length' and
    // 'capacity' on the stack (move/shallow copy)
    // let s2 = s1.clone(); <-- for deep copy of the heap data
    //          stack
    //    +-------------------+
    // s1 | ptr               +-----------+     heap
    //    | len          5    |           |   +--------+-------+
    //    | capacity     5    |           |   | index  | value |
    //    +-------------------+           +---|   0    |   h   |
    //      stack                         |   |   1    |   e   |
    //    +-------------------+           |   |   2    |   l   |
    // s2 | ptr               +-----------+   |   3    |   l   |
    //    | len          5    |               |   4    |   o   |
    //    | capacity     5    |               +--------+-------+
    //    +-------------------+
    // Now when drop is called, when s2/s1 go out of scope, they
    // try to free same memory (double free). To ensure safety,
    // after 'let s2 = s1', rust makes s1 invalid.
    // println!("{s1}, world!"); // <-- ERR: borrow of moved value s1

    // When you assign a completely new value to an existing variable,
    // Rust will call ‘drop’ and free the original value's memory
    // immediately. In this case, heap memory for "hello" is freed.
    s2 = String::from("Ahoy");
    println!("{s2}, world! from modified s2");

    // Rust has a special annotation called the ‘Copy’ trait that is
    // placed on types that are stored on the stack, as integers are.
    // If a type implements 'copy' trait, variables that use it do not
    // 'move', rather they are copied making them valid after assignment
    // to other variables. Rust won't let us annotate a type with ‘Copy’
    // if the type, or any of its parts, has implemented the ‘Drop’ trait.
    // Tuples containing only types that implement copy trait implement it too!
    println!("x = {x}, y = {y}");

    // • Passing a variable to fn will move or copy, similar to assignment.
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here
    let x = 5; // x comes into scope
    makes_copy(x); // because i32 implements the Copy trait,
                   // x does NOT move into the function,
    println!("{}", x); // so it's okay to use x afterward

    return_scope();
    use_refs();
    slices();
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

// ------------------------------------------------------------------------------------
fn return_scope() {
    // • Return values can also transfer ownership
    // The ownership of a variable follows the same pattern every time:
    // assigning a value to another variable 'moves' it.  When a variable that
    // includes data on the heap goes out of scope, the value will be cleaned
    // up by ‘drop’ unless ownership of the data has been moved to another
    // variable.
    let s1 = gives_ownership(); // gives_ownership 'moves' its return value into s1
    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is 'moved' into takes_and_gives_back, which
                                       // also 'moves' its return value into s3
    println!("{s3}, {s1}");
} // Here, s3 goes out of scope and is dropped. s2 was 'moved', so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("yours"); // some_string comes into scope
    some_string // some_string is returned & moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

// ------------------------------------------------------------------------------------
fn use_refs() {
    // A ‘reference’ is like a pointer in that it's an address
    // we can follow to access the data stored at that address; that data is
    // owned by some other variable.  Unlike a pointer, a reference is
    // guaranteed to point to a valid value of a particular type for the life
    // of that reference.
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // <-- &s1 is a reference that 'refers' to s1 but does not own it

    // When functions have references as parameters instead of the actual values,
    // we won't need to return the values in order to give back ownership, because
    // we never had ownership. We call the action of creating a reference ‘borrowing’.
    //                                                   heap
    //                       stack                     +--------+-------+
    //   +--------+        +-------------------+       | index  | value |
    // s | ptr    +---> s1 | ptr               +-----> |   0    |   h   |
    //   +--------+        | len          5    |       |   1    |   e   |
    //                     | capacity     5    |       |   2    |   l   |
    //                     +-------------------+       |   3    |   l   |
    //                                                 |   4    |   o   |
    //                                                 +--------+-------+
    println!("The length of '{s1}' is {len}.");

    let mut s = String::from("hello");
    // Mutable references have one big restriction: if you have a mutable
    // reference to a value, you can have no other references to that value.
    change(&mut s);
    {
        let _r1 = &mut s;
    } // r1 goes out of scope, so a new mutable ref can be created now
    let _r2 = &mut s; // r2 is valid

    // Multiple immutable refs are allowed but a mutable
    // and an immutable ref at the same time is a no go.
    // let r3 = &s; // <-- cannot have a mutable ref while having an immutable ref to same value
    // println!("{}, {}", r2, r3);

    ref_scope();
    dangling_refs();
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String (immutable reference)
    s.len()
} // s goes out of scope, doesn't have ownership so the value it refers to is not dropped

fn change(str: &mut String) {
    str.push_str(", world from mut ref");
}

fn ref_scope() {
    // A reference's scope starts from where it is introduced and
    // continues through the last time that reference is used.
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    print!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!(", {r3} from the ref_scope function");
}

fn dangling_refs() {
    // If you have a reference to some data, the compiler will ensure that
    // the data will not go out of scope before the reference to the data does.
    let _reference_to_nothing = dangle();
}

// This function's return type contains a borrowed value,
// but there is no value for it to be borrowed from.
// dangle returns ref to a string
// fn dangle() -> &String {
//     let s = String::from("hello"); // s is new String
//     &s // return ref to String s
// } // s goes out of scope, its 'drop'ped!!
// Solution is to return the string itself and not the ref, 'move' happens
fn dangle() -> String {
    let s = String::from("hello");
    s
}

// ------------------------------------------------------------------------------------
fn slices() {
    // Slices let you reference a contiguous sequence of elements
    // in a collection instead of the entire collection. Since its
    // a reference, it does not have ownership.
    println!("{}", first_word_non_slice(&"abcd efgh".to_string())); // 4

    // String slice is a reference to part of a String
    // Note: String slice range indices must occur at valid UTF-8 character
    // boundaries.  If you attempt to create a string slice in the middle of a
    // multibyte character, your program will exit with an error. (UTF-8 vs ASCII)
    let s = String::from("hello world");
    let _hello = &s[..5]; // s[0..5]
    let _world = &s[6..]; // s[6..s.len()]
    let _slice = &s[..]; // s[0..s.len()]
    println!("{}", first_word_slice(&"abcd efgh".to_string())); // abcd
    println!("{}", first_word_slice(&"abcdefgh".to_string())); // abcdefgh

    // String literals are slices (immutable ref to data)
    let s = "Hello, world!";
    println!("Type of {s} is &str");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // type of slice is &[i32]
    assert_eq!(slice, &[2, 3]);
}

fn first_word_non_slice(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

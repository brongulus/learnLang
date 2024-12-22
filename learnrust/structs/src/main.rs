#![allow(unused)]
// The 'owned' String type is used rather than &str slice type
// so that each instance of the struct owns all its data.
// Its possible for structs to store references to data owned
// by something else, leveraging 'lifetimes' which ensure that
// the data referenced by a struct is valid as long as the struct is.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// struct User1 {
//     active: bool,
//     username: &str, // <-- Error: missing lifetime specifier
// }

fn main() {
    // Unlike tuples, structs name each piece of data and hence
    // don't have to rely on the order of the data to access values.
    // Entire instance of the struct must be mutable, not just singular fields.
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    let u2 = build_user(user1.email, user1.username); // user1.email, user1.username moved now!

    // println!("{}, {}", user1.email, user1.username); // <-- Error
    let u3 = User {
        active: false,
        ..u2
    };

    println!("{}, {}", u2.active, u2.sign_in_count);
    println!("{}, {}", u3.active, u3.sign_in_count);

    // Tuple structs have just types of fields.
    struct Color(i32, i32, i32);
    // Unlike tuples, tuple structs require naming the type of struct when destructuring.
    let black = Color(240, 110, 0);
    let Color(r, g, b) = black;

    // Structs without any field are 'unit-like structs', useful when one
    // has to implement a trait on a type but don't want to store any data.
    struct AlwaysEqual;

    let subject = AlwaysEqual;

    // Area calculation using tuples
    let rect1 = (30, 50);
    println!("Area calculated by using tuples is {}", area(rect1));

    // Using struct would allow us to label the dimensions
    let rect1 = Rectangle {
        width: 30,
        length: 50,
    };

    println!(
        "Area calculated by using struct for rect {rect1:?} is {} and using method is {}",
        area_str(&rect1), // borrow the struct, not move
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 40,
        length: 20,
    };

    println!("Can {rect1:?} hold {rect2:?}? {}", rect1.can_hold(&rect2));

    dbg!(rect1); // <-- Outputs to stderr, takes ownership of rect1 & returns it.

    // rect1 = dbg!(rect1) // <-- ownership returned back to rect1 (needs rect1 to be mut)
    // println!("{rect1:?}"); // <-- Errors out

    println!(
        "Square of length 5 has area {}",
        Rectangle::square(5).area()
    );
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 3,
    }
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

// Unlike functins, methods are defined in context of a struct/enum/trait object
// First parameter is always 'self' representing the instance of the struct method
// is being called on.
impl Rectangle {
    // Within 'impl' block, type 'Self' is an alias for type that
    // the 'impl' block is for. '&self' is short for 'self: &Self'
    fn area(&self) -> u32 {
        self.width * self.length
    }
    // Often, not always, when a method name is same as a field, that method
    // is only used to return the field value, aka 'getter' method. Useful
    // as they allow making field private but method public to allow read-only access.
    fn width(&self) -> u32 {
        self.width
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        (self.width >= rectangle.width && self.length >= rectangle.length)
            || (self.length >= rectangle.width && self.width >= rectangle.length)
    }

    // Associated functions that aren't methods are often used for constructors that
    // return a new instance of the struct. ('new') These functions are called with ::
    // :: is used for associated fns and namespaces created by modules
    fn square(size: u32) -> Self {
        Self {
            width: size,
            length: size,
        }
    }
}

fn area_str(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.length
}

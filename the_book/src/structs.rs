// defining a struct
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// TUPLE STRUCTS
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

// UNIT LIKE STRUCT
struct AlwaysEqual;

pub fn run_structs() {
    let user1 = User {
        // creating an instance
        active: false,
        username: String::from("vitalik"),
        email: String::from("vitalik@ethereum.org"),
        sign_in_count: 21,
    };
    //Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable

    println!("{:#?}", user1);
    println!("username: {:?}", user1.username); // accessing the value of a particular field in the user struct

    // creating instance with other instances using struct update syntax
    let user2 = User {
        email: String::from("gavin@polkadot.com"),
        ..user1
    }; // Note that the struct update syntax uses = like an assignment; this is because it moves the data,
       // In this example, we can no longer use user1 after creating user2 because the String in the username field of user1 was moved into user2

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("x-cordinate: {:?}", origin.0); // accessing a value in tuple struct

    let subject = AlwaysEqual;
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    } // shorthand: Because the parameter names and the struct field names are exactly the same
}

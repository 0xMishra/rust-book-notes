// rust uses ownership for memory management
// ownership: set of rules for rust to manage memory
// ownership rules:
//1, Each value in Rust has an owner.
//2. There can only be one owner at a time.
//3. When the owner goes out of scope, the value will be dropped.
pub fn run_ownership() {
    // The stack stores values in the order it gets them and removes the values in the opposite order i.e. LIFO (last in, first out).
    // fixed size data is stored on the stack and dynamic size data is stored on the heap

    // scope of a variable
    {
        let s = "hello";
        println!("s: {}", s);
    } // s is out of scope here

    // THE STRING TYPE growable string that is stored on the heap

    let mut s = String::from("hello");
    s.push_str(" world");
    println!("new s: {}", s);

    {
        let s1 = String::from("yo"); // s is valid from this point
    } // this scope is over, s is no longer valid

    // how variables and data interact
    //1. copying of data
    let x = 5;
    let y = 6;

    //2. moving of data
    let s1 = String::from("yo");
    let s2 = s1; // s1 is no longer valid it has moved into s2
                 // println!("{s1}");// this will give error

    //3. cloning of data
    let s3 = String::from("hey");
    let s4 = s3.clone(); // s3 cloned, it is still valid
    println!("s3: {}, s4: {}", s3, s4);

    //The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does.

    let s5 = String::from("hello"); // s comes into scope

    takes_ownership(s5); // s's value moves into the function...
                         // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

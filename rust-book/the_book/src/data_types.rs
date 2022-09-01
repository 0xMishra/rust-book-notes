// two types: scalar and compound
pub fn show_data_types() {
    // SCALAR TYPES:
    // integers, floating-point numbers, Booleans, and character

    let num1 = 0;
    let dec = 0.5;
    let bool = true;
    let ch = 'a';

    // range of signed integers:  -(2n - 1) to 2n - 1 - 1
    // range of unsigned integers: 0 to 2n - 1

    // floats in rust are f32 and f64

    // Numeric operations
    let sum = 5 + 10; // addition
    let product = 5.0 * 0.8; // multiplication

    // division
    let quotient = 6 / 3; // quotient
    let floored = 2 / 3; // results in 0

    let remainder = 43 % 5; // remainder

    let literal = r#""hello world""#; // template string literal
    println!("{literal}");

    // COMPOUND TYPES:
    // primitive compound types: tuples and arrays

    // tuple has fixed length

    let tup = (43, "hello", String::from("me"));
    println!("{:?}", tup);
    println!(" first value in tup: {}", tup.0); // tuple indexing

    let (a, b, c) = tup; // destructuring tuple

    let unit_tuple = ();

    // arrays have fixed length in rust
    let array = ["harry", "ron", "hermione"];
    let harry = array[0]; // array indexing
}

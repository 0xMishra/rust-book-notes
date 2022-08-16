// by default the variables are immutable in rust

pub fn variables_and_mutability() {
    let x = 5;
    println!("value of x is {}", x);
    // x = 6;  cannot assign twice to immutable variable

    let mut y = 6;
    println!("value of y is {}", y);
    y = 7; // y is mutable, can change the value of y
    println!("value of y later is {}", y);

    // constants: can't use mut with constants
    // always annotate the type manually in case of constants
    //  constants may be set only to constant expression, not the result of a value that could only be computed at runtime.

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // shadowing
    let z = 78;
    println!("z before shadow: {}", z);
    let z = String::from("Z"); // z shadowed here
    println!("z after a shadow: {}", z);
}

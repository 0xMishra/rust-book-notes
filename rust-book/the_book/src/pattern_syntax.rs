// all the syntax valid in patterns

use std::i32;

pub fn run_pattern_syntax() {
    // matching literals
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        _ => println!("something else"),
    }

    // matching named variables
    let a = Some(5);
    let b = 10;
    match a {
        Some(50) => println!("got 50"),
        Some(b) => println!("matched, y: {}", b), // b is new and completely different variable not the variable declared before
        _ => println!("default case, x ={:?}", a), // this is the same a variable declared before,
    }
    println!("at the end: x ={:?}, y= {b}", a);

    // multiple patterns
    let c = 5;
    match c {
        1 | 2 => println!("one or two"), // pattern OR operator
        _ => println!("something else"),
    }

    // matching ranges of value
    let d = 5;
    match d {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // destructuring to break apart values
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // destructuring enums
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }

    // destructuring nested structs and enums
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    let msg2 = Message2::ChangeColor(Color::Hsv(0, 160, 255));
    match msg2 {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }

    // destructuring structs and tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // ignoring values in pattern
    foo(3, 4);
    // ignoring parts of a value with a nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    // We can also use underscores in multiple places within one pattern to ignore particular value
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    // ignoring unused variables with _
    let _f = 5; // Here we get a warning about not using the variable y, but we don’t get a warning about not using _x
    let g = 10;
    println!("g:{}", g);

    // ignoring remaining parts of a value using ..
    struct Point2 {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point2 { x: 0, y: 0, z: 0 };

    match origin {
        Point2 { x, .. } => println!("x is {}", x),
    }

    // extra conditionals with match guard

    // A match guard is an additional if condition, specified after the pattern in a match arm, that must also match for that arm to be chosen.
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    } // the compiler doesn't try to check for exhaustiveness when match guard expressions are involved.

    // @ bindings
    enum Message3 {
        Hello { id: i32 },
    }

    let msg = Message3::Hello { id: 5 };

    match msg {
        Message3::Hello {
            id: id_variable @ 3..=7, // The at operator @ lets us create a variable that holds a value at the same time as we’re testing that value for a pattern match.
        } => println!("Found an id in range: {}", id_variable),
        Message3::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message3::Hello { id } => println!("Found some other id: {}", id),
    }
}

fn foo(_: i32, y: i32) {
    // ignoring an entire value with _
    println!("This code only uses the y parameter: {}", y);
}

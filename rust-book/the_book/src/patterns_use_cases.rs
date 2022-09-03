pub fn run_pattern_use_cases() {
    // 1. match arms
    let x = Some(6);
    match x {
        Some(value) => Some(value + 1),
        None => None,
    };

    // 2. conditional if let
    // it’s also possible to mix and match if let, else if, and else if let expressions
    let favourite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favourite_color {
        println!("using your favourite color {} as the background ", color);
    } else if is_tuesday {
        println!("tuesday is the green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // 3. while let conditional loops
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // 4. for loops
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // 5. let statements
    let y = 5; // let PATTERN = EXPRESSION;
    let (x, y, z) = (1, 2, 3);

    // 6. function parameters
    let point = (3, 5);
    print_coordinates(&point);
}

// 6. function parameters
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

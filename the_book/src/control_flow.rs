// if else and loops
pub fn run_control_flow() {
    // IF ELSE
    let number = 3;
    if number > 5 {
        println!("the number is greater than 5");
    } else if number == 5 {
        println!("the number is equal to 5");
    } else {
        println!("the number is smaller than 5");
    }

    // using if in a let  statement
    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("the value of num is {}", num);

    // LOOPS
    // infinite loop
    // loop {
    //     println!("hello");
    // }

    // returning value from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2;
        }
    };
    println!("the result is {}", result);

    // loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while loops
    let mut num2 = 7;

    while num2 != 0 {
        println!("{num2}");
        num2 -= 1;
    }

    // looping through the elements of a collection
    let a = [10, 20, 30, 40, 50];
    for num in a {
        println!("{num}");
    }
}

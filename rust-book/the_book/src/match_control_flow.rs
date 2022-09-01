// match is powerful control flow construct in rust

#[derive(Debug)]
enum UsState {
    Albama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
pub fn run_match_flow() {
    let five = Some(5);
    let six = plus_one(five);
    let none = None;
    let still_none = plus_one(none);
    println!("{:?}", still_none);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // defining a match statement
        // a match arm has two parts: pattern and some code
        Coin::Nickel => 5, // this is a match arm
        Coin::Quarter(state) => {
            println!("This quarter is from {:?}", state);
            25
        }
        Coin::Penny => {
            println!("lucky penny!");
            1
        } // you can also use brackets for your expressions
        Coin::Dime => 10,
    }
}

// match statements with Option<T> enum
fn plus_one(x: Option<i32>) -> Option<i32> {
    // arms’ patterns must cover all possibilities
    match x {
        Some(value) => Some(value + 1),
        None => None,
    }
}

// catching all the possible variant using _ syntax
fn roll() {
    let dice_roll = 6;
    match dice_roll {
        3 => add_something(),
        6 => remove_something(),
        _ => change_nothing(), // all the other possible variant is taken care by this _ pattern, no need to explicitely mention them
    }
}

fn add_something() {}
fn remove_something() {}
fn change_nothing() {}

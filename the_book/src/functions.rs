// snake case as convention to name the functions in rust
pub fn run_functions() {
    println!("hello functions");
    another_function(String::from("harry"));
    multiple_parameter(String::from("john"), 35);
    let age_after_a_year = increment_age(27);
}

// adding a parameter
fn another_function(name: String) {
    println!("this is from another function, hello {}", name);
}

// function with mutliple parameters
fn multiple_parameter(name: String, age: i32) {
    println!("You are {} and your age is {}", name, age);
}

// returning a value in functions
fn increment_age(age: i32) -> i32 {
    age + 1
}

pub fn run_advanced_functions_and_closures() {
    let answer = do_twice(add_one, 5);
    println!("the answer is {}", answer);
    //  an example of where you could use either a closure defined inline or a named function
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    // or
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
}

// function pointers
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
// one example of where you would want to only accept fn and not closures is when interfacing with external code that doesn’t have closures: C functions can accept functions as arguments, but C doesn’t have closures.

fn main() {
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}
// }Here we create Status::Value instances using each u32 value in the range that map is called on by using the initializer function of Status::Value.

// can't directly return a closure so we have to use Box
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

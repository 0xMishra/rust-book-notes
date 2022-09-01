// two types of errors: recoverable and unrecoverable
//  allows you to choose the alternative of immediately aborting, which ends the program without cleaning up
// you can switch from unwinding to aborting upon a panic by adding panic = 'abort' to the appropriate [profile] sections in your Cargo.toml file.
pub fn run_unrecoverable_errors() {
    // panic!("crash and burn");  panic call

    // panic backtrace
    let v = vec![1, 2, 3, 4];
    v[99];
    // A backtrace is a list of all the functions that have been called to get to this point
    // you can run this command below to track the error
    // $  RUST_BACKTRACE=1 cargo run
}

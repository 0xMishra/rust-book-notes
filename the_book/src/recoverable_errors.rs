// Result enum is defined as having two variants, Ok and Err, as follows:
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::File;
use std::io::{self, ErrorKind, Read};
pub fn run_recoverable_errors() {
    let greeting_file_result = File::open("hello.txt"); // this operation can fail , the file might not exist, or we might not have permission to access the file.

    // handling this with match expression
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // handling different kinds of error
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("{:?}", error),
            },
            other_error => {
                panic!("problem while opening the file: {:?}", other_error);
            }
        },
    };

    // another way of handling all of this using closures

    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // unwrap and expect
    let greeting_file = File::open("hello.txt").unwrap();
    // unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us. Here is an example of unwrap in action

    //expect method lets us also choose the panic! error message
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

// PROPAGATING THE ERRORS
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }

    // shortcut for propagating errors
    //
    // {
    //  let mut username_file = File::open("hello.txt")?;
    //  let mut username = String::new();
    //  username_file.read_to_string(&mut username)?;
    //  Ok(username)
    // }
}

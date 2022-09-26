pub fn run_advanced_types() {
    // Rust provides the ability to declare a type alias to give an existing type another name. For this we use the type keyword.
    type Kilometers = i32;
    //  Values that have the type Kilometers will be treated the same as values of type i32

    let x = 5;
    let y: Kilometers = 5;
    assert_eq!(x, y);

    // A type alias makes our code more manageable by reducing the repetition
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }
}

// Type aliases are also commonly used with the Result<T, E> type for reducing repetition.
use std::fmt;

type Result<T> = std::result::Result<T, std::io::Error>;
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

// Rust has a special type named ! that’s known in type theory lingo as the empty type because it has no value
// fn bar() -> ! {}

// Dynamically Sized Types and the Sized Trait
// Let’s dig into the details of a dynamically sized type called str, which we’ve been using throughout the book. That’s right, not &str, but str on its own, is a DST. We can’t know how long the string is until runtime, meaning we can’t create a variable of type str

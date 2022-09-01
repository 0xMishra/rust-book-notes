// implementing the Deref trait allows you to customize the behavior of the dereference operator * (not to be confused with the multiplication or glob operator)
use std::ops::Deref;
pub fn run_deref_trait() {
    let x = 5;

    // using Box<T> like a reference
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // dereference operator

    // using our own Box implementation
    let a = 5;
    let b = MyBox::new(x);
    assert_eq!(5, a);
    assert_eq!(5, *b);

    // When we entered *b in Listing 15-9, behind the scenes Rust actually ran this code:
    // *(b.deref())

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // deref coercion happens here
               // under the hood:   hello(&(*m)[..]);
}

// IMPLEMENTING OUR OWN BOX TYPE
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Deref coercion converts a reference to a type that implements the Deref trait into a reference to another type. For example, deref coercion can convert &String to &str because String implements the Deref trait such that it returns &str.

fn hello(name: &str) {
    println!("Hello, {name}!");
}

// Box<T> allows you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data.

// Because a Box<T> is a pointer, Rust always knows how much space a Box<T> needs: a pointer’s size doesn’t change based on the amount of data it’s pointing to.
use crate::box_pointer::List::{Cons, Nil};

pub fn run_box_pointer() {
    // using box to store data on the heap
    let b = Box::new(5);
    println!("b: {}", b);

    // implementing cons list data structure using Box
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

// enabling recursive types with box
enum List {
    Cons(i32, Box<List>),
    Nil,
}

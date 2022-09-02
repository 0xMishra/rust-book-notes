use crate::refcell_smart_pointer::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;
//  Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data
// normally, this action is disallowed by the borrowing rules.
pub fn run_refcell_smart_pointer() {
    // With references and Box<T>, the borrowing rules’ invariants are enforced at compile time. With RefCell<T>, these invariants are enforced at runtime. With references, if you break these rules, you’ll get a compiler error.

    let x = 5;
    // let y = &mut x;  you can't borrow x mutably as x is immutable

    //  Because Rc<T> holds only immutable values, we can’t change any of the values in the list once we’ve created them. Let’s add in RefCell<T> to gain the ability to change the values in the lists. Listing 15-24 shows that by using a RefCell<T> in the Cons definition, we can modify the value stored in all the lists:

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
    println!("value: {:?}", *value);
}

// use case for interior mutability

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

// A common way to use RefCell<T> is in combination with Rc<T>. Recall that Rc<T> lets you have multiple owners of some data, but it only gives immutable access to that data. If you have an Rc<T> that holds a RefCell<T>,

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

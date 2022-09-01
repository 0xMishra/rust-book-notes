// The assert! macro, provided by the standard library, is useful when you want to ensure that some condition in a test evaluates to true
// if condition == true: test passes
// if condition == false: assert! macro calls panic!

// This code snippet is for assert! macro
#[derive(Debug, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// This function is written for the assert_eq! and assert_ne! macros
pub fn add_2(num: i32) -> i32 {
    num + 2
}

// This function is for custom failure messages
pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

// This code snipper is for should_panic! macro
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1 , got {}",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100 , got {}",
                value
            );
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] // test attribure converts a function into a test function
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // this test should panic
    #[test]
    #[should_panic]
    fn another() {
        panic!("make this test fail");
    }

    // test for can_hold method of Rectangle struct
    #[test]
    fn larger_can_hold_smaller() {
        let rect1 = Rectangle {
            width: 8,
            height: 7,
        };
        let rect2 = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(rect1.can_hold(&rect2));
    }

    // this test should fail
    #[test]
    #[ignore = "just an example"]
    fn smaller_cannot_hold_larger() {
        let rect1 = Rectangle {
            width: 8,
            height: 7,
        };
        let rect2 = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(rect2.can_hold(&rect1));
    }

    // Test for the add_2 funtion
    #[test]
    fn it_adds_two() {
        assert_eq!(add_2(2), 4);
    }

    // Test shows the use of custom failure messages. This test should fail
    #[test]
    #[ignore = "just an example"]
    fn greeting_contains_name() {
        let result = greeting("harry");
        assert!(
            result.contains("harry"),
            "Greeting did not contain name, value was '{}'",
            result
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")] // this test will pass as it is intended to panic
    fn greater_than_100() {
        Guess::new(200);
    }

    // Using Result<T,E> instead of just panicking

    #[test]
    #[ignore = "its just an example"] // Now this test will be ignored
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("input is not equals to output"))
        }
    }
}

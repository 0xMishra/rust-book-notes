// Methods are similar to functions: we declare them with the fn keyword and a name, they can have parameters and a return value,
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl (implementation) block
// All functions defined within an impl block are called associated functions
impl Rectangle {
    fn area(&self) -> u32 {
        //  The &self is actually short for self: &Self
        // this is a method
        self.width * self.height
    }

    // we can choose to give a method the same name as one of the struct’s fields for e.g.
    fn width(&self) {
        println!("width of this rectangle is {}", self.width);
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// we can have multiple impl blocks for the same struct
impl Rectangle {
    // methods with multiple parameters
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }
}

pub fn run_method_syntax() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let area1 = rect1.area();
    println!("rect1: {:#?}", rect1);
    rect1.width();
    println!("the area of rectangle is {} square units", area1);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    let sq = Rectangle::square(3); // a way to call an associated function
}

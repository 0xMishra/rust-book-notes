// understanding the use cases of structs in rust with an example

// grouping data with structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn run_struct_example() {
    let rect1 = Rectangle {
        width: 40,
        height: 20,
    };
    let area1 = area(&rect1);
    println!("rect1: {:#?}", rect1);
    println!("the area of rectangle is {} square units", area1);
    dbg!(&rect1); // takes ownership, used for debugging
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

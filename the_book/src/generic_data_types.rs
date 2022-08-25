// We use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types
// Using generic dosen't make your code run slower Rust uses Monomorphization to achieve this.
// Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

// in enum definition
enum Option<T> {
    // with single generic type
    Some(T),
    None,
}

// enum with multiple generic type
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// in struct definition
struct Point<T, U> {
    // x and y are different types here
    x: T,
    y: U,
}

// in method definition
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// using different type parameter for different instances of the same struct
// some generic parameters are declared with impl and some are declared with the method definition
impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run_generic_data_types() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// in function definition
// this function largest is generic over some type T
fn largest<T>(list: &[T]) -> &T
where
    T: std::cmp::PartialOrd,
{
    let mut _largest = &list[0];
    for elem in list {
        if elem > _largest {
            _largest = elem;
        }
    }
    _largest
}

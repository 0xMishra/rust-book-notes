//Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. Vectors can only store values of the same type.
pub fn run_vectors() {
    let mut v = Vec::new(); // to create a new empty vector

    // vectors are implemented using generics
    let v2 = vec![1, 2, 3]; // to create a vectors using macro
    v.push(1); // to update a vector
    v.push(2);
    v.push(3);

    // accessing a value in a vector
    let third = &v[2];
    println!("The third element is {}", third);
    let third = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // iterarting over values in a vector
    // iterate over immutable reference
    for i in &v {
        println!("{}", i);
    }

    // iterate over mutable reference
    for i in &mut v {
        *i += 50;
    }
}

//  A reference is like a pointer in that it’s an address we can follow to access the data. unlike pointer reference is guranteed to point to some valid data
pub fn run_references() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("length of {} is {}", s1, len);

    let mut s2 = String::from("world");
    change_string(&mut s2);
}

// We call the action of creating a reference borrowing.
// references are immutable by default
fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped.

fn change_string(s: &mut String) {
    //  if you have a mutable reference to a value, you can have no other references to that same value at the same time
    s.push_str("! ");
}

// Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used.

fn scope_of_references() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

// RULES OF REFERENCES
//1. At any given time, you can have either one mutable reference or any number of immutable references.
// 2. References must always be valid.

// String type is UTF-8 encoded and they are growable
pub fn run_strings() {
    // creating a new string instance
    let s = String::new();

    let data = "hello".to_string(); // converting string slices into String type

    // we can also use String::from() to generate a new String type from str literals
    let s2 = String::from("hello");

    // updating a string
    let mut s1 = String::from("foo");
    s1.push_str("bar"); // to append a string
    s1.push('!'); // to append a character

    // conactenation with + operator

    let s3 = String::from("harry ");
    let s4 = String::from("potter");
    let s5 = s3 + &s4; // s3 is moved here, its no longer valid

    // The + operator uses the add method, whose signature looks something like this:
    // fn add(self, s: &str) -> String {}

    //  When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..]

    let s6 = String::from("a");
    let s7 = String::from("b");
    let s8 = String::from("c");
    let s9 = s6 + " " + &s7 + " " + &s8;

    // or we can use format! macro
    // let s9 = format!("{} {} {}", s6, s7, s8);

    let word = String::from("hello");
    // let h = s1[0]; this will give an error in rust
    // Rust strings don’t support indexing

    let he = &word[..2]; // this is valid

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("s: {}", s);

    // iterate over strings
    //  For individual Unicode scalar values, use the chars method.
    for c in "Зд".chars() {
        println!("{}", c);
    }

    //  the bytes method returns each raw byte, which might be appropriate for your domain:
    for b in "Зд".bytes() {
        println!("{}", b);
    }
}

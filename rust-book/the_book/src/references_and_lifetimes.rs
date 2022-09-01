use std::fmt::Display;

// lifetimes are another kind of generics
// every reference in rust has a lifetime which is the scope for which the reference is valid

pub fn validating_references_with_lifetimes() {
    // lifetime annotation syntax:
    // &i32-> a reference
    // &'a i32 -> a reference with an explicit lifetime
    // &'a mut i32-> a mutable reference with an explicit lifetime

    let string1 = String::from("abcd");
    {
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        println!("result : {}", result);
    }

    let novel = String::from("Call me ishmael. Some years ago..");
    let first_sentence = novel.split('.').next().expect("could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // static lifetime
    let s: &'static str = "I have a static lifetime.";
}

//  it means that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the values referred to by the function arguments.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// this function has an example of dangling reference
fn dangling_reference() {
    // preventing dangling references with lifetimes
    let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    let x = 5;
    r = &x; // this line fixes the problems with dangling reference
    println!("r:{}", r);
}

// references in struct definition

struct ImportantExcerpt<'a> {
    part: &'a str,
}

// this function compiles witout lifetime annotations
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// lifetime annotation for impl blocks

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// Generic Type Parameters, Trait Bounds, and Lifetimes Together
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection
pub fn run_slice_type() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that we could meaningfully use the value 5 with. word is now totally invalid!

    // STRING SLICES
    let hello = &s[0..5];
    let world = &s[6..11];
    let slice1 = &s[..2]; // starts from index zero
    let slice2 = &s[3..]; // ends with last byte of String
    let slice3 = &s[..]; // contains entire string

    // ARRAY SLICE

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

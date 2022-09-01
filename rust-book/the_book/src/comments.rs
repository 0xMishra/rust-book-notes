pub fn run_comments() {
    // hello world: simple comment
    // first line
    // second line

    let number = 7; // this is a number
}

// below are documentation comments that starts with three slahes(///).
// these types of comments will generate HTML documentation. The HTML displays the contents of documentation comments for public API items intended for programmers interested in knowing how to use your crate as opposed to how your crate is implemented.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

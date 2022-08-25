// Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.
// A trait defines functionality a particular type has and can share with other types.

use std::fmt::{Debug, Display};

// defining a trait
pub trait Summary {
    fn summarize_author(&self) -> String; // without default implementation
    fn summarize(&self) -> String {
        format!("author: {}", self.summarize_author()) // default implementation,
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// implementing a trait
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// implemnting the dafault implementation
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

pub fn traits_and_shared_behavior() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    let tweet = Tweet {
        username: String::from("vitalik"),
        content: String::from("I like coffee"),
        reply: false,
        retweet: false,
    };
    print!("{}", tweet.summarize());
    notify(&article);
}

// traits as parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}

// trait bound syntax
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news: {}", item.summarize());
}

// multiple trait bounds with the + syntax
pub fn notify3<T: Summary + Display>(item: &T) {}
// OR
// pub fn notify3(item: &(impl Summary + Display)) {}

// trait bounds with where clause
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    6
}

// returning type that implements traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
} // However, you can only use impl Trait if you’re returning a single type.

// Using Trait Bounds to Conditionally Implement Methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

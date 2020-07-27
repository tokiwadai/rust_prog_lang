use std::fmt::Display;
use std::fmt::Debug;

//Defining a Trait, pp. 201
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String;
    // Trait with default implementation, pp. 204
    // Default implementations can call other methods in the same trait, pp. 204
    fn summarize_default(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// Implementing a Trait on a Type, pp. 202
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.headline)
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
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content) }
}

// Traits as Parameters, pp. 206
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize_default());
}

// Trait Bound Syntax, pp. 206
pub fn notify_multiple_traits<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize_default());
}

//  Trait Bounds with where Clauses, pp. 207
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug {
    unimplemented!()
}

// Returning Types that Implement Traits, pp 207
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"), content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new tweet, default summary: {}", tweet.summarize_default());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"), location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ), };
    println!("New article available! {}", article.summarize());
    println!("New article available! default summary: {}", article.summarize_default());

    notify(&tweet);
    notify(&article);

    notify(&returns_summarizable());
}
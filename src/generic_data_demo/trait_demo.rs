use std::fmt::Display;

pub trait Summary {
    fn summaryize_author(&self) -> String;
    fn summaryize(&self) -> String {
        format!("(read more from {}...)", self.summaryize_author())
    }
}

pub trait Displayd {
    fn display_author(&self) -> String;
    fn display(&self) -> String {
        format!("display")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summaryize_author(&self) -> String {
        format!("{} {} 著 ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summaryize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl Displayd for Tweet {
    fn display_author(&self) -> String {
        format!("display_author")
    }
}

pub fn notify(item: &impl Summary) {
    println!("notify top news!{}", item.summaryize());
}

// Trait Bound
pub fn generic_notify<T: Summary>(item: &T) {
    println!("generic_notify top news!{}", item.summaryize());
}

pub fn mul_generic_notify<T: Summary + Displayd>(item: &T) {
    println!("mul_generic_notify top news!{}", item.display());
}

pub fn where_mul_generic_notify<T>(item: &T)
where T: Summary + Displayd
{
    println!("where_mul_generic_notify top news!{}", item.display());
}

pub fn returns_summarizable() -> impl Summary + Displayd {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T>{
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("largest x = {}", self.x);
        } else {
            println!("largest y = {}", self.x);
        }
    }
}

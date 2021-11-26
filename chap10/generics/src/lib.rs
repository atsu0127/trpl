use std::fmt::{Display, Formatter};

pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("Author {}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {} {}", self.username, self.content, self.summarize_author())
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news!! {}", item.summarize())
}

pub fn notify_detail<T>(item: &T)
    where T: Display + Summary
{
    println!("Detailの方");
    notify(item);
}

pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

pub struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y}
    }
}

impl<T:Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(New content from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, {} ({})",
            self.headline,
            self.summarize_author(),
            self.location
        )
    }

    fn summarize_author(&self) -> String {
        format!("by {}", self.author)
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
        format!("{}: {}", self.summarize_author(), self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub struct BlogPost {
    pub name: String,
    pub content: String,
    pub author: String,
}

impl Summary for BlogPost {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub fn notify<T: Summary>(item: T) {
    println!("{}", item.summarize());
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

use std::fmt::Display;

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

fn main() {
    let tweet = Tweet {
        username: String::from("matiasklemola"),
        content: String::from("Rust rust rust rust"),
        reply: false,
        retweet: false,
    };

    let news_article = NewsArticle {
        headline: String::from("Putin and Trump meet today in Helsinki"),
        location: String::from("Helsinki"),
        author: String::from("Matias"),
        content: String::from("Content goes here"),
    };

    let blog_post = BlogPost {
        name: String::from("How to implement traits in Rust"),
        content: String::from("Interesting content"),
        author: String::from("Matias"),
    };

    notify(tweet);
    notify(news_article);
    notify(blog_post);

    let number_list = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest(&number_list));

    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest char is {}", largest(&char_list));

    let string_1 = String::from("First");
    let string_2 = String::from("Second");

    longest_with_an_announcement(
        &string_1,
        &string_2,
        String::from("This is an announcement"),
    );
}

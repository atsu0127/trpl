use std::cmp::Ordering;
use generics::{Summary, Tweet, NewsArticle, notify, notify_detail, returns_summarizable};

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let point_list = vec![Point {x: 10, y: 20}, Point {x: 15, y: 20}, Point {x: 5, y: 20}];
    let result = largest(&point_list);
    println!("The largest point is {:?}", result);

    let x: f32 = 9.0;
    let y: f32 = 12.0;
    let p = Point { x, y };
    println!("distance = {}", p.distance_from_origin());

    let q = Point {x: 10, y: 20};
    let mixed = p.mixup(q);
    println!("mixed x: {}, y: {}", mixed.x, mixed.y);

    let tweet = Tweet {
        username: String::from("uname"),
        content: String::from("content"),
        reply: false,
        retweet: false,
    };

    println!("tweet summary: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    notify(&article);
    notify_detail(&article);
    notify(&returns_summarizable());
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U
}

impl<T, U> PartialOrd for Point<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.x > other.x {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
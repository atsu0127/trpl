use std::ops::{Add};
use std::fmt;
use std::ops;
use std::fmt::{Formatter};

fn main() {
    // 関連型でトレイト定義においてプレースホルダーの型を指定する
    // ジェネリックでやったらどうなるか => impl時に具体的な型が必要になる
    let mut counter = Counter {
        count: 0u32
    };

    counter.next();

    // デフォルトのジェネリック型引数と演算子オーバーロード
    assert_eq!(
        Point { x: 1, y: 1 } + Point { x: 3, y: -1 },
        Point { x: 4, y: 0 }
    );

    assert_eq!(
        Millimeters(10) + Meters(1),
        Millimeters(1010)
    );

    // 明確化のためのフルパス記法: 同じ名前のメソッドを呼ぶ
    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);

    assert_eq!(Dog::baby_name(), "Dog".to_string());
    assert_eq!(<Dog as Animal>::baby_name(), "Animal".to_string());

    // スーパートレイトを使用して別のトレイト内で、あるトレイトの機能を必要とする
    Point {
        x: 3,
        y: 4,
    }.outline_print();

    // ニュータイプパターンを使用して外部の型に外部のトレイトを実装する
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
    println!("w[0] = {}", w.get(0).unwrap());
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

impl ops::Deref for Wrapper {
    type Target = Vec<String>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        "Dog".to_string()
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        "Animal".to_string()
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Flying as Pilot");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Flying as Wizard");
    }
}

impl Human {
    fn fly(&self) {
        println!("Flying as Human");
    }
}

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
#[derive(Debug, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

trait MyIterator<T> {
    fn next(&mut self) -> Option<T>;
}

#[derive(Debug)]
struct Counter {
    count: u32
}

impl MyIterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        println!("{:?}", self);
        Some(self.count)
    }
}
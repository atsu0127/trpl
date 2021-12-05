use std::ops::Deref;
use List::{Cons, Nil};

fn main() {
    let _ = Cons(1,
Box::new(Cons(2,
Box::new(Cons(3,
Box:: new(Nil))))));

    let x = 5;
    let y = MyBox::new(x);

    println!("not deref => {}", 5 == x);
    println!("deref => {}", 5 == *y);

    let name = MyBox::new(String::from("Rust"));
    hello(&name);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

 struct MyBox<T>(T);

 impl<T> MyBox<T> {
     fn new(x: T) -> MyBox<T> {
         MyBox(x)
     }
 }
 
 impl<T> Deref for MyBox<T> {
     type Target = T;

     fn deref(&self) -> &Self::Target {
         &self.0
     }
 }
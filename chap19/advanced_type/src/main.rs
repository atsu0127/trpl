extern crate advanced_type;

use std::io::Error;

fn main() {
    // 型エイリアスで型同義語を生成する
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("{}", x + y);

    println!("hoge: {:?}", get("hoge"));
    println!("hoge: {:?}", get("hehe"));
}

fn get(name: &str) -> advanced_type::types::Result<String> {
    if name == "hoge" {
        Ok("hoge".to_string())
    } else {
        Err(Box::new(advanced_type::types::CustomError))
    }
}
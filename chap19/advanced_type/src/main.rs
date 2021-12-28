extern crate advanced_type;

use std::io::Error;
use std::fmt;

fn main() {
    // 型エイリアスで型同義語を生成する
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("{}", x + y);

    // TODO: 以下 doesn't have a size known at compile-time が出るのでもうちょっと読み進めたり調べたりする
    // println!("hoge: {}", get("hoge"));
    // println!("hoge: {}", get("hehe"));

}

// TODO: 正しく実装しましょう
// fn get(name: &str) -> advanced_type::types::Result<String> {
//     if name == "hoge" {
//         Ok("hoge".to_string())
//     } else {
//         Err(advanced_type::types::CustomError)
//     }
// }
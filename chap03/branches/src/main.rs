fn main() {
    let cond = true;
    let number = if cond {
        5 // 式なのでセミコロン入れたらダメ
    } else {
        //6.0 // これはダメ
        6
    }; // let number = ~という文の末尾なのでセミコロンを入れる

    println!("{}", number);
}

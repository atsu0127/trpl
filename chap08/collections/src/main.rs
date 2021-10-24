use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // Vec
    let mut v = vec![1, 2, 3];
    v.push(10);

    println!("{:?}", v);

    let second = v.get(1);

    if let Some(2) = second {
        println!("2")
    } else {
        println!("not 2")
    }

    let v = vec![1, 2, 3, 4, 5];
    for i in &v {
        // ↑で不変の参照を借用しているので、以下はできない
        // ただしこのスコープ(for文)を抜けてればOK
        // let _ = &mut v[0];
        println!("{}", i);
    }

    let mut v = vec![1, 2, 3, 4, 5];

    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", v);

    // String
    // 文字列リテラルから作るときはto_string()
    let literal = "hogehoge";
    let mut s = literal.to_string();
    let s2 = "hehe".to_string();
    s.push_str(&s2);
    s.push('a');
    println!("{}", s);

    let s1 = "Hello, ".to_string();
    let s2 = "World".to_string();
    let s3 = s1 + &s2;
    println!("{}", s3);
    // s1の所有権は移動しているので、以下はできない
    // println!("{} + {} = {}", s1, s2, s3);

    let s1 = "hoge".to_string();
    let s2 = "hehe".to_string();
    let s3 = format!("{}{}", s1, s2);
    // format!マクロでは所有権が奪われないので以下ができる
    println!("{} + {} = {}", s1, s2, s3);

    let s = String::from("Здравствуйте");

    let len = s.len();
    println!("12になるはずが… {}", len);

    // Stringをindexで引けないので以下はできない(checkで引っかかる)
    // println!("1文字目は{}",  &s[0]);
    // 以下は一文字にならないので、スライスするときもダメ(実行時エラー)
    // println!("1文字目は{}",  &s[0..1]);
    // 以下は一文字以上になるのでOK
    println!("部分文字列: {}",  &s[0..4]);

    // 上記の感じで挙動が怖いので、charsとか使いましょう
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // 以下でbyteも返せます
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    // 書記素クラスタは以下の感じ
    // 以下のuse必要
    // use unicode_segmentation::UnicodeSegmentation;
    for c in "नमस्ते".graphemes(true) {
        println!("{}", c);
    }

    // HashMap
    let mut scores  = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

    // collectで作る
    // let teams = vec!["Red".to_string(), "Blue".to_string()];
    // let scores_only = vec![10, 20];
    // let scores: HashMap<_, _> = teams.iter().zip(scores_only.iter()).collect();

    // getしてみる
    let team = "Red".to_string();
    let _ = scores.get(&team);
    // println!("got Red score: {}", score);

    // 走査はこっち
    for (k, v) in &scores {
        println!("{}'s score: {}", k, v)
    }

    // 更新関連
    println!("初期値 {:#?}", scores);

    // 上書き
    scores.insert("Red".to_string(), 100);
    println!("Redを更新 {:#?}", scores);

    // なかったら追加
    scores.entry("Red".to_string()).or_insert(50);
    scores.entry("Green".to_string()).or_insert(50);
    println!("Greenを追加 {:#?}", scores);

    // 以前の値を使っていく
    let nums = vec![1, 2, 3, 4, 5, 6, 7];
    let mut count = HashMap::new();

    for num in nums {
        let c: &mut i32;
        if num % 2 == 0 {
            c = count.entry("Even".to_string()).or_insert(0);
        } else {
            c = count.entry("Odd".to_string()).or_insert(0);
        }
        *c += 1;
    }

    println!("{:#?}", count);
}

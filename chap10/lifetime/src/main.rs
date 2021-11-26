fn main() {
    // 長い文字列は長い
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // 僕をイシュマエルとお呼び。何年か前・・・
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");  // '.'が見つかりませんでした
    let i = ImportantExcerpt { part: first_sentence };
    println!("{}", i.part);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ImportanceExcerptインスタンスは、partより長生きしない
// => インスタンス参照がpartの参照より外にあったらエラーがでる(そらそう)
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // &selfだけなので注釈はいらない
    fn level(&self) -> i32 {
        3
    }

    // これは&selfと戻り値の&strのライフタイムが一緒になるので、注釈はいらない
    // => selfがなくなるときには参照できなくなる
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
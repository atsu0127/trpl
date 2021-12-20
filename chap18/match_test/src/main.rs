fn main() {
    // if letサンプル
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            // 紫を背景色に使用します
            println!("Using purple as the background color");
        } else {
            // オレンジを背景色に使用します
            println!("Using orange as the background color");
        }
    } else {
        // 青を背景色に使用します
        println!("Using blue as the background color");
    }

    // while let サンプル
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for文
    let v = vec!['a', 'b', 'c'];
    for(idx, item) in v.iter().enumerate() {
        println!("{} is at index {}", item, idx);
    }

    // 関数の引数にパターンを使う
    let point = (3, 5);
    print_coordinates(&point);

    // 論駁可能性
    // 論駁不可能な文脈で、論駁可能なものを使ってしまう
    // let Some(x) = Some(110);
    // println!("number is {}", x);

    // 論駁可能な文脈で、論駁不可能なものを使ってしまう
    // これはwarningになる
    // if let x = 5 {
    //     println!("number is {}", x);
    // }

    // パターン記法色々
    // リテラルへのマッチ
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        _ => println!("other"),
    }

    // 名前付き変数へのマッチ
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("x is 50"),
        Some(y) => println!("Matched y, y => {:?}", y),
        _ => println!("Not Matched, x => {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    // 複数のパターン
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("other"),
    }

    // ...で値の範囲に合致させる
    let x = 5;

    match x {
        1 ..= 5 => println!("one through five"),
        _ => println!("other"),
    }

    let x = 'c';

    match x {
        'a' ..= 'j' => println!("early ASCII letter"),
        'k' ..= 'z' => println!("late ASCII letter"),
        _ => println!("other"),
    }

    // 分配して値を分解する
    // 構造体の場合
    let p = Point { x: 0, y: 7 };
    // 以下は let Point{ x, y } = p; と言うふうにもかける
    let Point { x: a, y: b} = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    match p {
        Point { x, y: 0} => println!("x: {}", x),
        Point { x: 0, y}  => println!("y: {}", y),
        Point { x, y } => println!("x: {}, y: {}", x, y),
    }

    // 参照の場合
    let points = vec![
        Point { x: 0, y: 0},
        Point { x: 1, y: 5},
        Point { x: 10, y: -3},
    ];

    let sum_of_squares: i32 = points
        .iter()
        .map(|&Point { x, y }| x * x + y * y)
        .sum();

    println!("sum is {}", sum_of_squares);

    // 構造体とタプル
    // 以下みたいなこともできる
    let ((_feet, _inches), Point { x: _, y: _ }) = ((3, 10), Point { x: 3, y: -10 });

    // パターンの値の無視
    // _で値全体を無視する
    foo(1, 2);

    // ネストされた_で値の一部を無視する
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("cannot overwrite an existing value");
        },
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    // 名前を_で始めて未使用の変数を無視する
    // 以下の感じ
    let _x = 5;
    let y = 10;

    // _<変数名>は束縛はするので、以下はできない
    // _sを_にすれば動きます
    // let s = Some("Hello".to_string());
    //
    // if let Some(_s) = s {
    //     println!("found string");
    // }
    //
    // println!("{:?}", s);

    // ..で値の残りの部分を無視する
    let origin = Point3d { x: 0, y: 0, z: 0 };

    match origin {
        Point3d { x, .. } => println!("x is {}", x),
    }

    // refとref mutでパターンに参照を生成する
    let mut robot_name = Some(String::from("Hello"));

    match robot_name {
        Some(ref name) => println!("Found a name: {}", name),
        None => (),
    }

    match robot_name {
        Some(ref mut name) => *name = "World".to_string(),
        None => (),
    }

    println!("robot_name is: {:?}", robot_name);

    // マッチガードで追加の条件式
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // matchの中で外の変数を使う
    let x = Some(10);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched n = {:?}", n),
        _ => println!("Default x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    // orのパターンにマッチガード使う
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @束縛
    let msg = Message::Hello { id: 6 };

    match msg {
        Message::Hello { id: id_variable @ 3...7 } => {
            println!("between 3 and 7: {}", id_variable);
        },
        Message::Hello { id: 10...12 } => {
            println!("Found an id in another range, between 10 and 12")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
}

enum Message {
    Hello { id: i32 },
}

fn foo(_: i32, y: i32) {
    // このコードは、y引数を使うだけです: {}
    println!("This code only uses the y parameter: {}", y);
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

struct Point {
    x: i32,
    y: i32,
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
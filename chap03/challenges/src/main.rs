use std::io;
use std::io::Write;

fn main() {
//    c1();
    println!("{}", c2_1(10));
    println!("{}", c2_2(10));
}

// 1. 温度を華氏と摂氏で変換する。
fn c1() {
    let mut t= String::new();
    loop {
        print!("which type, do you want to use(c or f): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut t).expect("Failed to read line");
        t = t.trim().to_string();
        if t == "c" || t == "f" {
            break;
        } else {
            println!("Please input c or f");
            t = "".to_string();
            continue;
        }
    }

    let mut number = String::new();

    loop {
        print!("Please input number: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut number).expect("Failed to read line");

        let number = match number.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Not Number Form: {}", e);
                number = "".to_string();
                continue;
            }
        };

        if t == "c" {
            println!("f: {}", _c2f(number));
        } else if t == "f" {
            println!("c: {}", _f2c(number));
        }

        break;
    }
}

// 摂氏->華氏
fn _c2f(c: f64) -> f64 {
    c * 1.8 + 32.0
}

// 華氏->摂氏
fn _f2c(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

// 2. フィボナッチ数列のn番目を生成する。
// 末尾再帰の場合
fn c2_1(n: u32) -> u32 {
    if n == 1 {
        1
    } else if n == 2 {
        1
    } else {
        run(n - 1, 1, 1)
    }
}

fn run(n: u32, a: u32, b: u32) -> u32 {
    if n == 0 {
         return b;
    }
    run(n - 1, b, (a + b))
}

// 末尾再帰じゃない場合
fn c2_2(n: u32) -> u32 {
    if n == 1 {
        1
    } else if n == 2 {
        1
    } else {
        c2_2(n - 1) + c2_2(n - 2)
    }
}
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let num = &args[1];
    let num = num.parse().expect("パースエラー");
    println!("{}", c2_1(num))
}

// 末尾再帰の場合
fn c2_1(n: usize) -> usize {
    if n == 1 {
        1
    } else if n == 2 {
        1
    } else {
        run(n - 2, 1, 1)
    }
}

fn run(n: usize, a: usize, b: usize) -> usize {
    if n == 0 {
        return b;
    }
    run(n - 1, b, a + b)
}

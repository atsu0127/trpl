use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let num = &args[1];
    let num = num.parse().expect("パースエラー");
    println!("{}", c2_2(num))
}

fn c2_2(n: usize) -> usize {
    if n == 1 {
        1
    } else if n == 2 {
        1
    } else {
        c2_2(n - 1) + c2_2(n - 2)
    }
}
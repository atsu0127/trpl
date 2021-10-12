fn main() {
    c2_1(1000);
}

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

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1 // return x + 1やreturn x + 1;も可能 => returnは式を返す？
}
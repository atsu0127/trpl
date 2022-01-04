fn main() {
    // 関数ポインタ
    let v = vec![1, 3, 5, 7];
    let v = easy_map(v, double);
    println!("v = {:?}", v);

    let v: Vec<i32> = v.into_iter()
        .map(half)
        .collect();
    println!("v = {:?}", v);

    // クロージャを返却する
    let v: Vec<i32> = v.into_iter()
        .map(returns_closure())
        .collect();
    println!("v = {:?}", v);
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x * 10)
}

fn easy_map(arr: Vec<i32>, f: fn(i32) -> i32) -> Vec<i32> {
    let mut result = vec![];
    arr.into_iter().for_each(|i| {
        result.push(f(i));
    });
    result
}

fn double(x: i32) -> i32 {
    x * 2
}

fn half(x: i32) -> i32 {
    x / 2
}
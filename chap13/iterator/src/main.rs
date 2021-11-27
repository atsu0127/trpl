fn main() {
    // イテレータの基礎
    // let v1 = vec![1, 2, 3];
    // let v1_iter = v1.iter();
    //
    // for val in v1_iter {
    //     println!("{}", val);
    // }
    // println!("v1: {:?}", v1);
    //
    // イテレータの明示的な消費
    // into_iter…元配列をmoveする、iter_mut…可変借用する
    // let mut v1_iter = v1.iter();
    // println!("{:?}", v1_iter.next());
    // println!("{:?}", v1_iter.next());
    // println!("{:?}", v1_iter.next());
    // println!("{:?}", v1_iter.next());
    // println!("v1: {:?}", v1);
    //
    // let mut v1_iter = v1.iter();
    // print_typename(v1_iter.next());
    // print_typename(v1_iter.next());
    // print_typename(v1_iter.next());
    // print_typename(v1_iter.next());
    // println!("v1: {:?}", v1);
    //
    // sumもあるよ
    // let v1_iter = v1.iter();
    // let sum: i32 = v1_iter.sum(); // ここでv1_iterはmoveされる
    // println!("v1: {:?}", v1);
    // println!("sum: {}", sum);

    // イテレータの変形
    let v1: Vec<i32> = vec![1, 2, 3];
    let plus_one: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("plus_one: {:?}", plus_one);
}

fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}
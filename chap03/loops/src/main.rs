fn main() {
    let mut count = 0;
    loop {
        println!("{}: in loop", count);
        count += 1;
        if count > 5 {
            println!("break!");
            break;
        }
    }
    println!("loop finished");

    let mut count = 0;

    while count <= 5 {
        println!("{}: in while", count);
        count += 1;
    }
    println!("while finished");

    let contents = [1, 2, 3, 4, 5];

    for e in contents.iter() {
        println!("{}: in for1", e);
    }
    println!("for1 finished");
    for e in (9..15).rev() {
        println!("{}: in for2", e);
    }
    println!("for2 finished");
}

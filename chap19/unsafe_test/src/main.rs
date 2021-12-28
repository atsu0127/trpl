use std::slice;

fn main() {
    // 生ポインタの生成方法
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 参照できない生ポインタ作成
    let address = 0x012345usize;
    let _r3 = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        // println!("r3 is: {}", *r3);
    }

    // 以下のsafe rustだと借用規則によりできない
    // let mut num = "hoge".to_string();
    // let r1 = &num;
    // let r2 = &mut num;
    // println!("r1 is: {}", *r1);
    // println!("r2 is: {}", *r2);

    // unsafeな関数やメソッドを呼ぶ
    unsafe {
        dangerous();
    }

    // unsafeコードに安全な中傷を行う
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = my_split_at_mut(r, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // from_raw_parts_mutのクラッシュかもしれない例
    let address = 0x012345usize;
    let _r = address as *mut i32;

    // 以下でクラッシュするかも
    // let slice = unsafe {
    //     slice::from_raw_parts_mut(_r, 10000000)
    // };

    // extern関数を使用して、外部のコードを呼び出す
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // 可変で静的な変数にアクセスしたり、変更する
    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

static HELLO_WORLD: &str = "Hello, world!";

extern "C" {
    fn abs(input: i32) -> i32;
}

unsafe fn dangerous() {}

fn my_split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}
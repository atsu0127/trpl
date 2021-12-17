use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

fn main() {
    // 簡単なMutexの例
    // let m = Mutex::new(5);
    //
    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }
    //
    // println!("m = {:?}", m);

    // 複数スレッドでのMutex
    // let counter = Arc::new(Mutex::new(0));
    // let mut handles = vec![];
    //
    // for _ in 0..10 {
    //     let counter = Arc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }
    //
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    //
    // println!("Result: {}", *counter.lock().unwrap());

    // デッドロックしてみる
    let plus_counter = Arc::new(Mutex::new(0));
    let minus_counter = Arc::new(Mutex::new(100));
    let mut handles = vec![];

    let plus_counter1 = Arc::clone(&plus_counter);
    let minus_counter1 = Arc::clone(&minus_counter);
    let handle1 = thread::spawn(move || {
        println!("[handle1] Start");
        println!("[handle1] Got plus lock");
        let mut num = plus_counter1.lock().unwrap();
        *num += 1;
        // ここでちゃんとロック解除しないとだめ
        println!("[handle1] Drop plus lock");
        drop(num); // 大事
        println!("[handle1] Sleeping");
        thread::sleep(Duration::from_secs(1));
        println!("[handle1] Waiting minus lock");
        let mut num = minus_counter1.lock().unwrap();
        *num -= 1;
        println!("[handle1] Finished");
    });

    handles.push(handle1);

    let plus_counter2 = Arc::clone(&plus_counter);
    let minus_counter2 = Arc::clone(&minus_counter);
    let handle2 = thread::spawn(move || {
        println!("[handle2] Start");
        println!("[handle2] Got minus lock");
        let mut num = minus_counter2.lock().unwrap();
        *num -= 1;
        // ここでちゃんとロック解除しないとだめ
        println!("[handle2] Drop minus lock");
        drop(num); // 大事
        println!("[handle2] Sleeping");
        thread::sleep(Duration::from_secs(1));
        println!("[handle2] Waiting plus lock");
        let mut num = plus_counter2.lock().unwrap();
        *num += 1;
        println!("[handle2] Finished");
    });

    handles.push(handle2);

    for handle in handles {
        println!("called: {:#?}", handle);
        handle.join().unwrap();
    }

    println!("Plus Result: {}", *plus_counter.lock().unwrap());
    println!("Minus Result: {}", *minus_counter.lock().unwrap());
}

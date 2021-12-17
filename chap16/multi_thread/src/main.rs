use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    // 一般的なspawn
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("number {} from spwaned thread!", i);
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    //
    // handle.join().unwrap();
    //
    // for i in 1..5 {
    //     println!("number {} from main thread!", i);
    //     thread::sleep(Duration::from_secs(1));
    // }

    // 他のスレッドのを使いたいときはmoveできる
    // let v = vec![1, 2, 3];
    // let handle = thread::spawn(move || {
    //     println!("vector: {:?}", v);
    // });
    // drop(v);
    //
    // handle.join().unwrap();

    // スレッド間でのデータ転送
    // let (tx, rx) = mpsc::channel();
    //
    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    // });
    //
    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    // チェンネルでの所有権の転送
    // let (tx, rx) = mpsc::channel();
    //
    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    //     println!("val is {}", val) // ここは動かない
    // });
    //
    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    // エラー処理の例
    // let (tx, rx) = mpsc::channel();
    //
    // thread::spawn(move || {
    //     let vals = vec![
    //         "hi".to_string(),
    //         "from".to_string(),
    //         "the".to_string(),
    //         "thread".to_string(),
    //     ];
    //
    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    //
    // println!("loop start");
    //
    // loop {
    //     match rx.try_recv() {
    //         Ok(str) => println!("received: {}", str),
    //         Err(mpsc::TryRecvError::Disconnected) => {
    //             println!("disconnected");
    //             break;
    //         },
    //         Err(mpsc::TryRecvError::Empty) => {
    //             println!("empty");
    //         }
    //     }
    //     thread::sleep(Duration::from_secs(1));
    // }
    // println!("finished!!");

    // 転送機をクローンして複数の生成器を作成する
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            "hi".to_string(),
            "from".to_string(),
            "the".to_string(),
            "thread".to_string(),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            "more".to_string(),
            "messages".to_string(),
            "for".to_string(),
            "you".to_string(),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
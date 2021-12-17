extern crate performance_test;

use quanta::Clock;
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    const N: usize = 5;
    const SIZE: usize = 100000000;

    println!("[make array]");
    let arr = performance_test::create_rand_array(SIZE);
    let arr2 = arr.clone();
    // println!("arr is {:?}", arr);
    let clock = Clock::new();
    println!("[run serial]");
    let start = clock.now();
    println!("sum: {}", arr.iter().fold(0, |sum, num| sum + num));
    println!("elapsed {}", clock.now().duration_since(start).as_nanos());

    println!("[run parallel1(n = {})]", N);
    let start = clock.now();
    let result = Arc::new(AtomicUsize::new(0));
    let arr = Box::new(arr);
    let arr = Box::leak(arr);
    arr.chunks(SIZE / N).map(|chunk| {
        let result = Arc::clone(&result);
        thread::spawn(move || {
            let sum = chunk.iter().fold(0, |sum, num| sum + *num);
            result.fetch_add(sum, Ordering::SeqCst);
        })
    })
    .for_each(|handle| handle.join().unwrap());

    println!("sum: {:?}", result);
    println!("elapsed {}", clock.now().duration_since(start).as_nanos());

    println!("[run parallel2(n = {})]", N);
    let start = clock.now();
    let result = Arc::new(AtomicUsize::new(0));
    let arr2 = Box::new(arr2);
    let arr2 = Box::leak(arr2);
    arr2.chunks(SIZE / N).map(|chunk| {
        let result = Arc::clone(&result);
        thread::spawn(move || {
            let sum = chunk.iter().fold(0, |sum, num| sum + *num);
            result.fetch_add(sum, Ordering::SeqCst);
        })
    })
        .collect::<Vec<std::thread::JoinHandle<()>>>()
        .into_iter()
        .for_each(|handle| handle.join().unwrap());

    println!("sum: {:?}", result);
    println!("elapsed {}", clock.now().duration_since(start).as_nanos());
}

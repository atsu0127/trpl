use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn main() {
    // Cacherの利用
    // let simulated_user_specified_value = 10;
    // let simulated_random_number = 7;
    //
    // generate_workout(
    //     simulated_user_specified_value,
    //     simulated_random_number
    // );

    // Cacherの拡張
    // let mut int_c = Cacher::new(|a| a);
    // let mut string_c = Cacher::new(|a| a);
    //
    // let v1 = int_c.value(1);
    // let v2 = int_c.value(2);
    // let v3 = string_c.value("hoge");
    // println!("v1: {}", v1);
    // println!("v2: {}", v2);
    // println!("v3: {}", v3);

    // moveしないclosure
    let x = vec![1, 2, 3];
    let equal_to_x = |z| z == x;
    let y = vec![1, 2, 3];
    println!("x: {:?}", x);
    println!("x == y = {}", equal_to_x(y));

    // moveするclosure
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    let y = vec![1, 2, 3];
    // println!("x: {:?}", x); // ← ここでエラーになる
    println!("x == y = {}", equal_to_x(y));
}

// Copyを排除できなかった
struct Cacher<T, U, V>
    where
        T: Fn(U) -> V,
        U: Eq + Hash + Copy,
        V: Copy {
    calculation: T,
    value: HashMap<U, V>
}

impl<T, U, V> Cacher<T, U, V>
    where
        T: Fn(U) -> V,
        U: Eq + Hash + Copy,
        V: Copy {
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);
    let mut expensive_result = Cacher::new(|num| {
        println!("ゆっくり計算します");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {

        println!(
            // 今日は{}回腕立て伏せをしてください！
            "Today, do {} pushups!",
            // expensive_result
            expensive_result.value(intensity)
        );

        println!(
            // 次に、{}回腹筋をしてください！
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            // 今日は休憩してください！水分補給を忘れずに！
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                // 今日は、{}分間走ってください！
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
use rand::prelude::*;

pub fn create_rand_array(num: usize) -> Vec<usize> {
    let mut arr: Vec<usize> = Vec::with_capacity(num);
    let mut rng = rand::thread_rng();
    (1..num).for_each(|_| {
        arr.push(rng.gen::<usize>() % 1000);
    });
    arr
}
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use crate::{add_two, internal_adder};

    #[test]
    fn test_add_two() {
       assert_eq!(4, add_two(2))
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2))
    }
}

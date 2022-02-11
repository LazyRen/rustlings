// iterators4.rs

pub fn factorial(num: u64) -> u64 {
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
    (1..=num).fold(1, |sum, v| sum * v)
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.reduce
    // (1..=num).reduce(|sum, v| sum * v).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}

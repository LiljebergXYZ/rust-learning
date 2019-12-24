pub fn sum_of_even_fibonacci_numbers() -> i32 {
    let mut x = 1;
    let mut y = 2;
    let mut sum = 0;

    while x <= 4000000 {
        if x % 2 == 0 {
            sum += x;
        }
        let z = x + y;
        x = y;
        y = z;
    }

    return sum;
}

#[cfg(test)]
mod test {
    #[test]
    fn test_sum_of_even_fibonacci_numbers() {
        assert_eq!(super::sum_of_even_fibonacci_numbers(), 4613732);
    }
}
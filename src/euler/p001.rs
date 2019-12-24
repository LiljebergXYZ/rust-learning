pub fn find_multiples_sum() -> i32 {
    let mut result = 0;
    for i in 0..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            result += i;
        }
    }

    return result;
}

#[cfg(test)]
mod test {
    #[test]
    fn test_find_multiples_sum() {
        assert_eq!(super::find_multiples_sum(), 233168);
    }
}
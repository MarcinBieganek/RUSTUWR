fn summy(str: &str) -> i32 {
    str.split_whitespace()
        .map(|s| {
            s.parse::<i32>().unwrap()
        })
        .sum()
}


mod tests {
    use super::*;
    
    #[test]
    fn three_numbers_test() {
        assert_eq!(summy("1 2 3"), 6);
    }

    #[test]
    fn four_numbers_test() {
        assert_eq!(summy("1 2 3 4"), 10);
    }

    #[test]
    fn five_numbers_test() {
        assert_eq!(summy("1 2 3 4 5"), 15);
    }

    #[test]
    fn same_number_test() {
        assert_eq!(summy("10 10"), 20);
    }

    #[test]
    fn zeros_test() {
        assert_eq!(summy("0 0"), 0);
    }
}
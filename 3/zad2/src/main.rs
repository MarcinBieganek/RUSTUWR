fn solution(num: i32) -> i32 {
    (3..num).filter(|i| {
        (i % 3 == 0) || (i % 5 == 0)
    })
    .sum()
}

mod tests {
    use super::solution;
    
    #[test]
    fn first_test() {
        // assertion(expected, input);
        assertion(23, 10);
    }

    #[test]
    fn second_tests() {
        // assertion(expected, input);
        assertion(33, 11);
    }

    #[test]
    fn small_tests() {
        // assertion(expected, input);
        assertion(8, 6);
    }

    #[test]
    fn bigger_tests() {
        // assertion(expected, input);
        assertion(225, 33);
    }

    #[test]
    fn big_tests() {
        // assertion(expected, input);
        assertion(3420, 123);
        assertion(543, 50);
        assertion(25719750, 10500);
    }
    
    fn assertion(expected : i32, input : i32) {
        let actual = solution(input);
        
        assert!(
            expected == actual,
            "\nTest failed!\n expected: {}\n actual: {}\n input: {}\n", expected, actual, input
        );
    }
}

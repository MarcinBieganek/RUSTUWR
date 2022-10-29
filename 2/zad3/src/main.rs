fn dont_give_me_five(start: isize, end: isize) -> isize {
    (start..end+1).map(|i| {
            i.to_string()
        })
        .filter(|s| {
            !s.contains("5")
        })
        .count() as isize
}


mod tests {
    use super::*;
    
    #[test]
    fn first_test() {
        assert_eq!(dont_give_me_five(1, 9), 8);
    }

    #[test]
    fn second_test() {
        assert_eq!(dont_give_me_five(4, 17), 12);
    }

    #[test]
    fn short_test() {
        assert_eq!(dont_give_me_five(4, 5), 1);
    }

    #[test]
    fn negative_test() {
        assert_eq!(dont_give_me_five(-5, -1), 4);
    }

    #[test]
    fn negative_positive_test() {
        assert_eq!(dont_give_me_five(-5, 5), 9);
    }
}

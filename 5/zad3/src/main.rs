fn find_digit(num: i32, nth: i32) -> i32 { 
    if nth <= 0 {
        return -1
    }

    let s = num.abs().to_string();
    if s.len() < (nth as usize) {
        return 0
    }
    s.chars()
     .nth(s.len() - (nth as usize))
     .unwrap()
     .to_digit(10)
     .unwrap() as i32
}


mod tests {
    use super::*;
    
    #[test]
    fn first_test() {
        assert_eq!(find_digit(5673, 4), 5);
    }

    #[test]
    fn second_test() {
        assert_eq!(find_digit(129, 2), 2);
    }

    #[test]
    fn negative_test() {
        assert_eq!(find_digit(-2825, 3), 8);
    }

    #[test]
    fn zero_negative_test() {
        assert_eq!(find_digit(-456, 4), 0);
    }

    #[test]
    fn zero_test() {
        assert_eq!(find_digit(0, 20), 0);
    }

}
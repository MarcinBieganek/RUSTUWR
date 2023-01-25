fn count_bits(n: i64) -> u32 {
    n.count_ones()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        assert_eq!(count_bits(0), 0);
    }

    #[test]
    fn second_test() {
        assert_eq!(count_bits(4), 1);
    }
    
    #[test]
    fn third_test() {
        assert_eq!(count_bits(7), 3);
    }

    #[test]
    fn fourth_test() {
        assert_eq!(count_bits(9), 2);
    }

    #[test]
    fn fifth_test() {
        assert_eq!(count_bits(10), 2);
    }
}


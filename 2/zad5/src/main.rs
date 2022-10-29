fn get_count(string: &str) -> usize {
    string.chars()
        .filter(|c| {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => true,
                _ => false
            }
        })
        .count()
}

mod tests {
    use super::*;
    
    #[test]
    fn first_test() {
        assert_eq!(get_count("abracadabra"), 5);
    }

    #[test]
    fn none_test() {
        assert_eq!(get_count("bwhhkkwzq"), 0);
    }

    #[test]
    fn all_test() {
        assert_eq!(get_count("aeiou"), 5);
    }

    #[test]
    fn empty_test() {
        assert_eq!(get_count(""), 0);
    }

    #[test]
    fn only_one_test() {
        assert_eq!(get_count("awwwwwwwwwwbbbbbbbbbbbdddddddddddaaddqqqqqqqqqqqqqqzzzzzzzzza"), 4);
    }
}

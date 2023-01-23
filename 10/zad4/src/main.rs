fn encode(msg: String, n: i32) -> Vec<i32> {
    let n_vec = n.to_string().chars().map(|i| i.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let n_len = n_vec.len();

    msg.chars()
       .enumerate()
       .map(|(i, el)| (el as u32 - 'a' as u32 + 1 + n_vec[i % n_len]) as i32)
       .collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn first_tests() {
        assert_eq!(encode("scout".to_string(), 1939), vec![20, 12, 18, 30, 21]);
    }

    #[test]
    fn second_tests() {
        assert_eq!(encode("masterpiece".to_string(), 1939), vec![14, 10, 22, 29, 6, 27, 19, 18, 6, 12, 8]);
    }

    #[test]
    fn simple_tests() {
        assert_eq!(encode("a".to_string(), 1939), vec![2]);
    }

    #[test]
    fn longer_tests() {
        assert_eq!(encode("abcdef".to_string(), 1), vec![2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn none_tests() {
        assert_eq!(encode("".to_string(), 1921), vec![]);
    }
}
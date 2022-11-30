fn expanded_form(n: u64) -> String {
    let s = n.to_string();
    
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            match c {
                '0' => "".to_string(),
                _ => c.to_string() + &(0..(s.len() - i - 1)).map(|_| "0").collect::<String>(),
            }
        })
        .filter(|e| {
            e != ""
        })
        .collect::<Vec<String>>()
        .join(" + ")
}

mod tests {
    use super::*;

    #[test]
    fn first_test() {
        assert_eq!(expanded_form(12), "10 + 2");
    }

    #[test]
    fn second_test() {
        assert_eq!(expanded_form(42), "40 + 2");
    }

    #[test]
    fn third_test() {
        assert_eq!(expanded_form(70304), "70000 + 300 + 4");
    }

    #[test]
    fn long_test() {
        assert_eq!(expanded_form(90000304), "90000000 + 300 + 4");
    }

    #[test]
    fn simple_test() {
        assert_eq!(expanded_form(4), "4");
    }
}
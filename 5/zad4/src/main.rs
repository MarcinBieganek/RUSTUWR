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
    fn examples() {
        assert_eq!(expanded_form(12), "10 + 2");
        assert_eq!(expanded_form(42), "40 + 2");
        assert_eq!(expanded_form(70304), "70000 + 300 + 4");
    }
}
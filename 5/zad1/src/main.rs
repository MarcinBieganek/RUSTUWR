fn spin_words(words: &str) -> String {
    words.split_whitespace()
        .map(|s| {
            if s.len() >= 5 {
                s.chars().rev().collect::<String>()
            } else {
                s.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

mod tests {
    use super::*;
    
    #[test]
    fn simple() {
        assert_eq!(spin_words("Welcome"), "emocleW");
    }

    #[test]
    fn mixed() {
        assert_eq!(spin_words("Hey fellow warriors"), "Hey wollef sroirraw");
    }

    #[test]
    fn none() {
        assert_eq!(spin_words("This is a test"), "This is a test");
    }

    #[test]
    fn longer() {
        assert_eq!(spin_words("You are almost to the last test"), "You are tsomla to the last test");
    }

    #[test]
    fn only_first() {
        assert_eq!(spin_words("Seriously this is the last one"), "ylsuoireS this is the last one");
    }

}
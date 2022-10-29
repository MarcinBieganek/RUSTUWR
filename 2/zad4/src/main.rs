fn longest(a1: &str, a2: &str) -> String {
    let mut chars: Vec<char> = (a1.to_owned() + a2).chars().collect();
    chars.sort();
    chars.dedup();

    chars.iter().collect()
}

mod tests {
    use super::*;
    
    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        println!("s1:{:?} s2:{:?}", s1, s2);
        println!("{:?} {:?}", longest(s1, s2), exp);
        println!("{}", longest(s1, s2) == exp);
        assert_eq!(&longest(s1, s2), exp)
    }

    #[test]
    fn first_test() {
        testing("aretheyhere", "yestheyarehere", "aehrsty");
    }

    #[test]
    fn second_test() {
        testing("loopingisfunbutdangerous", "lessdangerousthancoding", "abcdefghilnoprstu");   
    }

    #[test]
    fn third_test() {
        testing("testing", "anna", "aeginst");   
    }

    #[test]
    fn thesame_test() {
        testing("dcbaaaaa", "dcbaaaaa", "abcd");   
    }

    #[test]
    fn one_letter_test() {
        testing("eabcda", "aaaaaa", "abcde");   
    }
}
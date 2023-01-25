fn part_list(arr: Vec<&str>) -> String {

    (0..arr.len()-1).map(|i| {
        let first = arr[..i+1].join(" ");

        let second = arr[i+1..arr.len()].join(" ");

        format!("({}, {})", first, second)
    }).collect::<Vec<String>>().join("")
}


#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(arr: Vec<&str>, exp: &str) -> () {
        println!("arr: {:?}", arr);
        let ans = part_list(arr);
        println!("actual:\n{}", ans);
        println!("expect:\n{}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basis_test() {
        dotest(vec!["I", "wish", "I", "hadn't", "come"],
                "(I, wish I hadn't come)(I wish, I hadn't come)(I wish I, hadn't come)(I wish I hadn't, come)");
    }

    #[test]
    fn basic_test() {
        dotest(vec!["cdIw", "tzIy", "xDu", "rThG"], 
            "(cdIw, tzIy xDu rThG)(cdIw tzIy, xDu rThG)(cdIw tzIy xDu, rThG)"); 
    }

    #[test]
    fn simple_test() {
        dotest(vec!["a", "b", "c"], 
            "(a, b c)(a b, c)"); 
    }

    #[test]
    fn two_test() {
        dotest(vec!["a", "b"], 
            "(a, b)"); 
    }

    #[test]
    fn last_test() {
        dotest(vec!["aaa", "bbb", "ccc", "rThG"], 
            "(aaa, bbb ccc rThG)(aaa bbb, ccc rThG)(aaa bbb ccc, rThG)"); 
    }
}

fn camel_case(str: &str) -> String {
    str.split_whitespace()
        .map(|s| {
            s[0..1].to_uppercase() + &s[1..]
        })
        .collect()
}

mod tests {
    use super::camel_case;

    #[test]
    fn sample_test() {
        assert_eq!(camel_case("test case"), "TestCase");
        assert_eq!(camel_case("camel case method"), "CamelCaseMethod");
        assert_eq!(camel_case("say hello "), "SayHello");
        assert_eq!(camel_case(" camel case word"), "CamelCaseWord");
        assert_eq!(camel_case(""), "");
    }
}
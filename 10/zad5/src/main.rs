fn print(n: i32) -> Option<String> {
    if n < 0 || n % 2 == 0 {
        return None;
    }
    let space = " ";
    let ast = "*";
    let s = n/2 + 1;

    let res = (0..s).map(|i| {
        format!("{}{}", &space.repeat((s - i - 1).try_into().unwrap()), &ast.repeat((2 * i + 1).try_into().unwrap()))
    }).collect::<Vec<String>>().join("\n");

    if s == 1 {
        return Some(format!("{}\n", res));
    }

    let second = (0..s-1).rev().map(|i| {
        format!("{}{}", &space.repeat((s - i - 1).try_into().unwrap()), &ast.repeat((2 * i + 1).try_into().unwrap()))
    }).collect::<Vec<String>>().join("\n");

    Some(format!("{}\n{}\n", res, second))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn basic_test() {
        assert_eq!(print(3), Some(" *\n***\n *\n".to_string()) );
    }

    #[test]
    fn longer_test() {
        assert_eq!(print(5), Some("  *\n ***\n*****\n ***\n  *\n".to_string()) );
    }

    #[test]
    fn negative_test() {
        assert_eq!(print(-3),None);
    }

    #[test]
    fn even_test() {
        assert_eq!(print(2),None);
    }

    #[test]
    fn zero_test() {
        assert_eq!(print(0),None);
        assert_eq!(print(1), Some("*\n".to_string()) );  
    }

    #[test]
    fn one_test() {
        assert_eq!(print(1), Some("*\n".to_string()) );     
    }
}
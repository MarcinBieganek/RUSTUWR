fn dig_pow(n: i64, p: i32) -> i64 {
    
    let sum = n.to_string()
        .chars()
        .fold((0, p as i64), |acc, x| {
            (acc.0 + ((x.to_digit(10).unwrap() as i64).pow(acc.1 as u32)), acc.1 + 1)
        }).0;
    
    if (sum % n) == 0 {
        sum / n
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: i64, p: i32, exp: i64) -> () {
        println!(" n: {:?};", n);
        println!("p: {:?};", p);
        let ans = dig_pow(n, p);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }
    
    #[test]
    fn basic_test() {
        dotest(89, 1, 1);
    }

    #[test]
    fn none_tests() {
        dotest(92, 1, -1);
    }

    #[test]
    fn bigger_test() {
        dotest(46288, 3, 51);
    }

    #[test]
    fn simple_test() {
        dotest(695, 2, 2);
    }

    #[test]
    fn zeros_test() {
        dotest(1000, 2, -1);
    }
}

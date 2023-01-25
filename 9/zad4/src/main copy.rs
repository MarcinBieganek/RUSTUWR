
//let mut john_cash = Vec::from([(0, 0)]);

fn j(n: i32, john_cash: Vec<(i32, i32)>) -> i32 {
    let f = john_cash.iter()
             .find(|&&t| {
                t.0 == n
             });
    
    match f {
        Some(t) => t.1
        None => (n - a(j(n-1)))
    }
    
    /* 
    if n == 0 {
        0
    } else {
        (n - a(j(n-1)))
    }
    */
}

fn john(n: i32) -> Vec<i32> {
    (0..n).map(|i| {
        j(i)
    }).collect::<Vec<i32>>()
}

//let mut ann_cash = Vec::from([(0, 1)]);

fn a(n: i32) -> i32 {
    /* 
    let f = ann_cash.iter()
             .find(|&&t| {
                t.0 == n
             });
    
    match f {
        Some(t) => t.1
        None => (n - john(ann(n-1)))
    }
    */
    if n == 0 {
        1
    } else {
        (n - j(a(n-1)))
    }
}

fn ann(n: i32) -> Vec<i32> {
    (0..n).map(|i| {
        a(i)
    }).collect::<Vec<i32>>()
}

fn sum_john(n: i32) -> i32 {
    john(n).iter().sum()
}

fn sum_ann(n: i32) -> i32 {
    ann(n).iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_john() {
        assert_eq!(john(11), vec![0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6]);
        assert_eq!(john(14), vec![0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6, 7, 7, 8]);
    }
    #[test]
    fn test_ann() {
        assert_eq!(ann(6), vec![1, 1, 2, 2, 3, 3]);
        assert_eq!(ann(15), vec![1, 1, 2, 2, 3, 3, 4, 5, 5, 6, 6, 7, 8, 8, 9]);
    }
    #[test]
    fn test_sum_john() {
        assert_eq!(sum_john(75), 1720);
        assert_eq!(sum_john(78), 1861);
    }
    #[test]
    fn test_sum_ann() {
        assert_eq!(sum_ann(115), 4070);
        assert_eq!(sum_ann(150), 6930);
    }
}
